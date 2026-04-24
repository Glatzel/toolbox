use std::collections::VecDeque;

use clerk::tracing::Instrument;
use tokio::sync::mpsc;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use crate::KIOYU_JOB_SPAN;
use crate::job::{IPayload, Job};
use crate::resource::ResourcePool;

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error("closed")]
    Closed,
}
#[derive(Debug)]
struct RunningJob<P> {
    job: Job<P>,
    attempt: usize,
}

impl<P> RunningJob<P> {
    fn new(job: Job<P>) -> Self { Self { job, attempt: 0 } }
}

pub enum DispatcherEvent<P> {
    Submit(Job<P>),
    FreeResource(Job<P>),
    /// Sent by a worker when `execute` fails. `attempt` is the number of
    /// attempts already completed (1-based: first failure → attempt = 1).
    RetryJob {
        job: Job<P>,
        attempt: usize,
    },
}

pub struct DispatcherHandle<P> {
    pub(crate) tx: mpsc::Sender<DispatcherEvent<P>>,
    pub(crate) cancel: CancellationToken,
    pub(crate) join: tokio::task::JoinHandle<()>,
}

impl<P> DispatcherHandle<P> {
    pub async fn submit(&self, job: Job<P>) -> Result<(), DispatchError> {
        self.tx
            .send(DispatcherEvent::Submit(job))
            .await
            .map_err(|_| DispatchError::Closed)
    }

    pub async fn shutdown(self) {
        self.cancel.cancel();
        drop(self.tx);
        let _ = self.join.await;
    }
}

pub enum ResourceMode {
    Pooled(ResourcePool),
    Unlimited,
}

struct Dispatcher<P> {
    rx: mpsc::Receiver<DispatcherEvent<P>>,
    mode: ResourceMode,
    queue: VecDeque<RunningJob<P>>,
    joinset: JoinSet<()>,
}

impl<P> Dispatcher<P> {
    pub fn new(rx: mpsc::Receiver<DispatcherEvent<P>>, mode: ResourceMode) -> Self {
        Self {
            rx,
            mode,
            queue: VecDeque::new(),
            joinset: JoinSet::new(),
        }
    }
}

impl<P> Dispatcher<P>
where
    P: IPayload + Send + 'static,
{
    pub(crate) async fn run(
        mut self,
        tx: mpsc::Sender<DispatcherEvent<P>>,
        cancel: CancellationToken,
    ) {
        clerk::debug!("dispatcher started");

        loop {
            tokio::select! {
                event = self.rx.recv() => {
                    match event {
                        Some(event) => self.handle_event(event, &tx, &cancel),
                        None => {
                            clerk::debug!("dispatcher channel closed");
                            break;
                        }
                    }
                }

                Some(res) = self.joinset.join_next() => {
                    if let Err(e) = res {
                        clerk::error!("job panicked: {}", e);
                    }
                }

                _ = cancel.cancelled() => {
                    clerk::debug!("dispatcher cancelled");
                    break;
                }
            }
        }

        clerk::debug!("waiting for jobs to finish");

        while let Some(res) = self.joinset.join_next().await {
            if let Err(e) = res {
                clerk::error!("job panicked: {}", e);
            }
        }

        clerk::debug!("dispatcher stopped");
    }

    fn handle_event(
        &mut self,
        event: DispatcherEvent<P>,
        tx: &mpsc::Sender<DispatcherEvent<P>>,
        cancel: &CancellationToken,
    ) {
        match event {
            DispatcherEvent::Submit(job) => {
                self.queue.push_back(RunningJob::new(job));
            }

            DispatcherEvent::FreeResource(job) => {
                self.free_resources(&job);
            }

            DispatcherEvent::RetryJob { job, attempt } => {
                if attempt < job.max_retries {
                    clerk::debug!(
                        job.id    = %job.id,
                        job.name  = %job.name,
                        attempt,
                        max       = job.max_retries,
                        "job failed, scheduling retry",
                    );
                    self.queue.push_back(RunningJob { job, attempt });
                } else {
                    clerk::error!(
                        job.id   = %job.id,
                        job.name = %job.name,
                        "job exhausted {} retries, giving up",
                        job.max_retries,
                    );
                    self.free_resources(&job);
                }
            }
        }

        self.schedule(tx, cancel);
    }

    fn free_resources(&mut self, job: &Job<P>) {
        if let ResourceMode::Pooled(ref mut pool) = self.mode
            && let Err(e) = pool.free(job.resources.as_slice())
        {
            clerk::error!("resource free failed: {}", e);
        }
    }

    fn schedule(&mut self, tx: &mpsc::Sender<DispatcherEvent<P>>, cancel: &CancellationToken) {
        while let Some(running) = self.queue.pop_front() {
            match self.mode {
                ResourceMode::Unlimited => {
                    self.spawn_job(running, tx.clone(), cancel);
                }

                ResourceMode::Pooled(ref mut pool) => {
                    match pool.allocate(running.job.resources.as_slice()) {
                        Ok(true) => {
                            self.spawn_job(running, tx.clone(), cancel);
                        }
                        Ok(false) => {
                            self.queue.push_front(running);
                            break;
                        }
                        Err(e) => {
                            clerk::error!("resource allocation failed: {}", e);
                        }
                    }
                }
            }
        }
    }

    fn spawn_job(
        &mut self,
        running: RunningJob<P>,
        tx: mpsc::Sender<DispatcherEvent<P>>,
        cancel: &CancellationToken,
    ) {
        let RunningJob { job, attempt } = running;

        let span = clerk::tracing::span!(
            clerk::tracing::Level::DEBUG,
            KIOYU_JOB_SPAN,
            job.id      = %job.id,
            job.name    = %job.name,
            job.attempt = attempt,
        );

        let cancel = cancel.clone();
        // This attempt number, completed whether it succeeds or fails.
        let next_attempt = attempt + 1;

        self.joinset.spawn(
            async move {
                clerk::debug!(attempt, "job executing");

                match job.payload.execute(cancel.clone()).await {
                    Ok(()) => {
                        clerk::debug!("job execute succeeded, post processing");

                        if let Err(e) = job.payload.post_process().await {
                            clerk::error!("payload post process error: {}", e);
                        }

                        let _ = tx.send(DispatcherEvent::FreeResource(job)).await;

                        clerk::debug!("job finished");
                    }

                    Err(e) => {
                        clerk::error!(attempt = next_attempt, "job execute failed: {}", e);

                        let _ = tx
                            .send(DispatcherEvent::RetryJob {
                                job,
                                attempt: next_attempt,
                            })
                            .await;
                    }
                }
            }
            .instrument(span),
        );
    }
}

pub fn start_dispatcher<P>(pool: ResourcePool) -> DispatcherHandle<P>
where
    P: IPayload + Send + 'static,
{
    start_dispatcher_with_mode(ResourceMode::Pooled(pool))
}

pub fn start_dispatcher_unlimited<P>() -> DispatcherHandle<P>
where
    P: IPayload + Send + 'static,
{
    start_dispatcher_with_mode(ResourceMode::Unlimited)
}

fn start_dispatcher_with_mode<P>(mode: ResourceMode) -> DispatcherHandle<P>
where
    P: IPayload + Send + 'static,
{
    let (tx, rx) = mpsc::channel(128);
    let cancel = CancellationToken::new();
    let dispatcher = Dispatcher::new(rx, mode);

    let join = tokio::spawn({
        let tx_for_jobs = tx.clone();
        let cancel_for_dispatcher = cancel.clone();
        async move {
            dispatcher.run(tx_for_jobs, cancel_for_dispatcher).await;
        }
    });

    DispatcherHandle { tx, cancel, join }
}
