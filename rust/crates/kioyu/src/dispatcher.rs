use std::marker::PhantomData;
use std::sync::Arc;

use tokio::sync::mpsc;

use crate::job::{IPayload, Job};
use crate::resource::ResourcePool;

pub struct Dispatcher<H, P> {
    handler: Arc<H>,
    pool: Arc<tokio::sync::Mutex<ResourcePool>>,
    _payload: PhantomData<P>,
}

impl<H, P> Dispatcher<H, P>
where
    H: IPayload<P>,
    P: Send + 'static,
{
    pub async fn run_job(&self, job: Job<P>) {
        let mut pool = self.pool.lock().await;

        if pool.allocate(&job.resources).unwrap_or(false) {
            let handler = self.handler.clone();
            let pool = self.pool.clone();
            let req = job.resources.clone();

            tokio::spawn(async move {
                handler.execute(job).await;

                let mut pool = pool.lock().await;
                let _ = pool.free(&req);
            });
        }
    }
}
#[derive(Clone)]
pub struct DispatcherHandle {
    tx: mpsc::Sender<DispatcherEvent>,
}
impl DispatcherHandle {
    pub async fn submit(&self, job: Job) -> Result<(), SubmitError> {
        self.tx
            .send(DispatcherEvent::Submit(job))
            .await
            .map_err(|_| SubmitError::DispatcherClosed)
    }
}
pub fn start_dispatcher() -> DispatcherHandle {
    let (tx, rx) = tokio::sync::mpsc::channel(128);

    tokio::spawn(async move {
        dispatcher_loop(rx).await;
    });

    DispatcherHandle { tx }
}
