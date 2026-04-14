use std::collections::VecDeque;

use clerk::tracing::Instrument;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::KIOYU_JOB_SPAN;
use crate::job::{IPayload, Job};
use crate::resource::ResourcePool;

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error("closed")]
    Closed,
}

pub enum DispatcherEvent<P> {
    Submit(Job<P>),
    FreeResource(Job<P>),
    Shutdown,
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
        let _ = self.tx.send(DispatcherEvent::Shutdown).await;
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
    queue: VecDeque<Job<P>>,
    handles: Vec<tokio::task::JoinHandle<()>>,
}

impl<P> Dispatcher<P> {
    pub fn new(rx: mpsc::Receiver<DispatcherEvent<P>>, mode: ResourceMode) -> Self {
        Self {
            rx,
            mode,
            queue: VecDeque::new(),
            handles: Vec::new(),
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

        while let Some(event) = self.rx.recv().await {
            match event {
                DispatcherEvent::Submit(job) => {
                    self.queue.push_back(job);
                }

                DispatcherEvent::FreeResource(job) => {
                    if let ResourceMode::Pooled(ref mut pool) = self.mode {
                        if let Err(e) = pool.free(job.resources.as_slice()) {
                            clerk::error!("resource free failed: {}", e);
                        }
                    }
                }

                DispatcherEvent::Shutdown => {
                    clerk::debug!("dispatcher shutting down");
                    break;
                }
            }

            self.schedule(&tx, &cancel);
        }

        for handle in self.handles.drain(..) {
            let _ = handle.await;
        }

        clerk::debug!("dispatcher stopped");
    }

    fn schedule(&mut self, tx: &mpsc::Sender<DispatcherEvent<P>>, cancel: &CancellationToken) {
        while let Some(job) = self.queue.pop_front() {
            match self.mode {
                ResourceMode::Unlimited => {
                    self.spawn_job(job, tx.clone(), cancel);
                }

                ResourceMode::Pooled(ref mut pool) => {
                    match pool.allocate(job.resources.as_slice()) {
                        Ok(true) => {
                            self.spawn_job(job, tx.clone(), cancel);
                        }
                        Ok(false) => {
                            self.queue.push_front(job);
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
        job: Job<P>,
        tx: mpsc::Sender<DispatcherEvent<P>>,
        cancel: &CancellationToken,
    ) {
        let span = clerk::tracing::span!(
            clerk::tracing::Level::DEBUG,
            KIOYU_JOB_SPAN,
            job.id = %job.id,
            job.name = %job.name,
        );

        let cancel = cancel.clone();

        let handle = tokio::spawn(
            async move {
                clerk::debug!("job executing");

                if let Err(e) = job.payload.execute(cancel.clone()).await {
                    clerk::error!("payload execute error: {}", e);
                }

                clerk::debug!("job post processing");

                if let Err(e) = job.payload.post_process().await {
                    clerk::error!("payload post process error: {}", e);
                }

                if !cancel.is_cancelled() {
                    let _ = tx.send(DispatcherEvent::FreeResource(job)).await;
                }

                clerk::debug!("job finished");
            }
            .instrument(span),
        );

        self.handles.push(handle);
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