use std::collections::VecDeque;

use tokio::sync::mpsc;

use crate::job::{IPayload, Job, ResourceRequest};
use crate::resource::ResourcePool;

#[derive(Debug)]
pub enum DispatchError {
    Closed,
}
pub enum DispatcherEvent<P> {
    Submit(Job<P>),
    JobFinished(ResourceRequest),
}

#[derive(Clone)]
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
}
pub struct Dispatcher<P> {
    rx: mpsc::Receiver<DispatcherEvent<P>>,
    tx: mpsc::Sender<DispatcherEvent<P>>,
    pool: ResourcePool,
    queue: VecDeque<Job<P>>,
}

impl<P> Dispatcher<P> {
    pub fn new(
        rx: mpsc::Receiver<DispatcherEvent<P>>,
        tx: mpsc::Sender<DispatcherEvent<P>>,
        pool: ResourcePool,
    ) -> Self {
        Self {
            rx,
            tx,
            pool,
            queue: VecDeque::new(),
        }
    }
}

impl<P> Dispatcher<P>
where
    P: IPayload + Send + 'static,
{
    pub async fn run(mut self) {
        while let Some(event) = self.rx.recv().await {
            match event {
                DispatcherEvent::Submit(job) => {
                    self.queue.push_back(job);
                }
                DispatcherEvent::JobFinished(resources) => {
                    let _ = self.pool.free(resources.as_slice());
                }
            }

            self.schedule();
        }
    }

    fn schedule(&mut self) {
        while let Some(job) = self.queue.pop_front() {
            match self.pool.allocate(job.resources.as_slice()) {
                Ok(true) => {
                    self.spawn_job(job);
                }
                Ok(false) => {
                    self.queue.push_front(job);
                }
                Err(e) => {
                    clerk::error!("allocate error: {}", e);
                }
            }
        }
    }

    fn spawn_job(&self, job: Job<P>) {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            match job.payload.execute().await {
                Ok(_) => ()

                Err(e) => {
                    clerk::error!("payload error: {}", e);
                }
            };
            let _ = tx.send(DispatcherEvent::JobFinished(job.resources)).await;
        }
        });
    }
}
pub fn start_dispatcher<P>(pool: ResourcePool) -> DispatcherHandle<P>
where
    P: IPayload + Send + 'static,
{
    let (tx, rx) = mpsc::channel(128);

    let dispatcher = Dispatcher::new(rx, tx.clone(), pool);

    tokio::spawn(async move {
        dispatcher.run().await;
    });

    DispatcherHandle { tx }
}
