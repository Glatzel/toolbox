use std::collections::VecDeque;

use clerk::tracing::Instrument;
use tokio::sync::mpsc;

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
}

impl<P> DispatcherHandle<P> {
    pub async fn submit(&self, job: Job<P>) -> Result<(), DispatchError> {
        self.tx
            .send(DispatcherEvent::Submit(job))
            .await
            .map_err(|_| DispatchError::Closed)
    }

    pub async fn shutdown(&self) { self.tx.send(DispatcherEvent::Shutdown).await.ok(); }
}

struct Dispatcher<P> {
    rx: mpsc::Receiver<DispatcherEvent<P>>,
    pool: ResourcePool,
    queue: VecDeque<Job<P>>,
}

impl<P> Dispatcher<P> {
    pub fn new(rx: mpsc::Receiver<DispatcherEvent<P>>, pool: ResourcePool) -> Self {
        Self {
            rx,
            pool,
            queue: VecDeque::new(),
        }
    }
}

impl<P> Dispatcher<P>
where
    P: IPayload + Send + 'static,
{
    async fn run(mut self, tx: mpsc::Sender<DispatcherEvent<P>>) {
        clerk::debug!("dispatcher running");
        while let Some(event) = self.rx.recv().await {
            match event {
                DispatcherEvent::Submit(job) => {
                    clerk::debug!("submitted");
                    self.queue.push_back(job);
                }
                DispatcherEvent::FreeResource(job) => {
                    clerk::debug!("freeing resources");
                    if let Err(e) = self.pool.free(job.resources.as_slice()) {
                        clerk::error!("free failed: {}", e);
                    }
                }
                DispatcherEvent::Shutdown => {
                    clerk::debug!("dispatcher shutting down");
                    return;
                }
            }

            self.schedule(&tx);
        }
    }

    fn schedule(&mut self, tx: &mpsc::Sender<DispatcherEvent<P>>) {
        while let Some(job) = self.queue.pop_front() {
            match self.pool.allocate(job.resources.as_slice()) {
                Ok(true) => {
                    clerk::debug!("allocated, spawning");
                    self.spawn_job(job, tx.clone());
                }
                Ok(false) => {
                    clerk::debug!("insufficient resources, re-queued");
                    self.queue.push_front(job);
                    break;
                }
                Err(e) => {
                    clerk::error!("allocation error: {}", e);
                }
            }
        }
    }

    fn spawn_job(&self, job: Job<P>, tx: mpsc::Sender<DispatcherEvent<P>>) {
        // The span must be created before spawn and explicitly moved in,
        // since the executor may run this on a different thread.
        let span = clerk::tracing::span!(
            clerk::tracing::Level::DEBUG,
            KIOYU_JOB_SPAN,
            job.id = %job.id,
            job.name = %job.name,
        );
        tokio::spawn(async move {
            clerk::debug!("executing");
            match job.payload.execute().instrument(span).await {
                Ok(_) => clerk::debug!("finished"),
                Err(e) => clerk::error!("payload error: {}", e),
            }
            clerk::debug!("releasing resources");
            let _ = tx.send(DispatcherEvent::FreeResource(job)).await;
        });
    }
}

pub fn start_dispatcher<P>(pool: ResourcePool) -> DispatcherHandle<P>
where
    P: IPayload + Send + 'static,
{
    let (tx, rx) = mpsc::channel(128);
    let dispatcher = Dispatcher::new(rx, pool);
    clerk::debug!("dispatcher started");

    tokio::spawn({
        let tx_for_jobs = tx.clone();
        async move { dispatcher.run(tx_for_jobs).await }
    });

    DispatcherHandle { tx }
}
