use std::collections::VecDeque;

use tokio::sync::mpsc;

use crate::job::{IPayload, Job};
use crate::resource::ResourcePool;

#[derive(Debug)]
pub enum DispatchError {
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
    pub async fn shutdown(&self) {
        self.tx.send(DispatcherEvent::Shutdown).await.ok();
    }
}
pub struct Dispatcher<P> {
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
    pub async fn run(mut self, tx: mpsc::Sender<DispatcherEvent<P>>) {
        while let Some(event) = self.rx.recv().await {
            match event {
                DispatcherEvent::Submit(job) => {
                    clerk::debug!("submit job: {}", job.id);
                    self.queue.push_back(job);
                }
                DispatcherEvent::FreeResource(job) => {
                    clerk::debug!("free resource: {}", job.id);
                    let _ = self.pool.free(job.resources.as_slice());
                }
                DispatcherEvent::Shutdown => break,
            }

            self.schedule(&tx);
        }
    }

    fn schedule(&mut self, tx: &mpsc::Sender<DispatcherEvent<P>>) {
        while let Some(job) = self.queue.pop_front() {
            match self.pool.allocate(job.resources.as_slice()) {
                Ok(true) => {
                    clerk::debug!("allocate job: {}", job.id);
                    self.spawn_job(job, tx.clone());
                }
                Ok(false) => {
                    clerk::debug!("queue job: {}", job.id);
                    self.queue.push_front(job);
                    break;
                }
                Err(e) => {
                    clerk::error!("allocate error: {}", e);
                }
            }
        }
    }

    fn spawn_job(&self, job: Job<P>, tx: mpsc::Sender<DispatcherEvent<P>>) {
        tokio::spawn(async move {
            match job.payload.execute().await {
                Ok(_) => {
                    clerk::debug!("job finished: {}", job.id);
                }
                Err(e) => {
                    clerk::error!("payload error: {}", e);
                }
            };

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

    tokio::spawn({
        let tx_for_jobs = tx.clone();
        async move {
            dispatcher.run(tx_for_jobs).await;
        }
    });

    DispatcherHandle { tx }
}
