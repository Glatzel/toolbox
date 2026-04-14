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
}

impl<P> DispatcherHandle<P> {
    pub async fn submit(&self, job: Job<P>) -> Result<(), DispatchError> {
        self.tx
            .send(DispatcherEvent::Submit(job))
            .await
            .map_err(|_| DispatchError::Closed)
    }

    pub async fn shutdown(&self) {
        self.cancel.cancel();
        self.tx.send(DispatcherEvent::Shutdown).await.ok();
    }
}

pub enum ResourceMode {
    /// Normal mode: allocate/free via the pool.
    Pooled(ResourcePool),
    /// Unlimited mode: skip allocation entirely, spawn immediately.
    Unlimited,
}

struct Dispatcher<P> {
    rx: mpsc::Receiver<DispatcherEvent<P>>,
    mode: ResourceMode, // replaces bare `pool` field
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
        clerk::debug!("dispatcher running");
        while let Some(event) = self.rx.recv().await {
            match event {
                DispatcherEvent::Submit(job) => {
                    clerk::debug!("submitted");
                    self.queue.push_back(job);
                }
                DispatcherEvent::FreeResource(job) => {
                    // Only meaningful in Pooled mode; ignored in Unlimited.
                    if let ResourceMode::Pooled(ref mut pool) = self.mode {
                        clerk::debug!("freeing resources");
                        if let Err(e) = pool.free(job.resources.as_slice()) {
                            clerk::error!("free failed: {}", e);
                        }
                    }
                }
                DispatcherEvent::Shutdown => {
                    clerk::debug!("dispatcher shutting down");

                    // wait running jobs
                    for handle in self.handles.drain(..) {
                        handle.await.ok();
                    }

                    clerk::debug!("dispatcher all jobs done");
                    return;
                }
            }
            self.schedule(&tx, &cancel);
        }
    }

    fn schedule(&mut self, tx: &mpsc::Sender<DispatcherEvent<P>>, cancel: &CancellationToken) {
        while let Some(job) = self.queue.pop_front() {
            match self.mode {
                // --- Unlimited: always spawn immediately, no allocation. ---
                ResourceMode::Unlimited => {
                    clerk::debug!("unlimited mode, spawning immediately");
                    self.spawn_job(job, tx.clone(), cancel);
                }
                // --- Pooled: original allocation logic unchanged. ----------
                ResourceMode::Pooled(ref mut pool) => match pool.allocate(job.resources.as_slice())
                {
                    Ok(true) => {
                        clerk::debug!("allocated, spawning");
                        self.spawn_job(job, tx.clone(), cancel);
                    }
                    Ok(false) => {
                        clerk::debug!("insufficient resources, re-queued");
                        self.queue.push_front(job);
                        break;
                    }
                    Err(e) => {
                        clerk::error!("allocation error: {}", e);
                    }
                },
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
                clerk::debug!("executing");
                match job.payload.execute().await {
                    Ok(_) => clerk::debug!("finished"),
                    Err(e) => clerk::error!("payload error: {}", e),
                }

                clerk::debug!("post processing");
                match job.payload.post_process().await {
                    Ok(_) => clerk::debug!("post processed"),
                    Err(e) => clerk::error!("post process error: {}", e),
                }
                clerk::debug!("post process returned");
                if !cancel.is_cancelled() {
                    clerk::debug!("releasing resources");
                    let _ = tx.send(DispatcherEvent::FreeResource(job)).await;
                }
            }
            .instrument(span),
        );
        self.handles.push(handle);
    }
}

// Two constructors on the public API — callers pick their mode.
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
    clerk::debug!("dispatcher started");
    tokio::spawn({
        let tx_for_jobs = tx.clone();
        let cancel_for_dispatcher = cancel.clone();
        async move { dispatcher.run(tx_for_jobs, cancel_for_dispatcher).await }
    });
    DispatcherHandle { tx, cancel }
}
