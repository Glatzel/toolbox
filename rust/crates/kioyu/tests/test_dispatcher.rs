use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use async_trait::async_trait;
use clerk::LevelFilter;
use kioyu::dispatcher::start_dispatcher;
use kioyu::job::{IPayload, Job, ResourceRequest};
use kioyu::resource::{ResourceKey, ResourcePool};
use tokio::time::{Duration, sleep};

struct TestPayload {
    counter: Arc<AtomicUsize>,
}

#[async_trait]
impl IPayload for TestPayload {
    type Error = mischief::Report;

    async fn execute(&self) -> Result<(), Self::Error> {
        self.counter.fetch_add(1, Ordering::SeqCst);
        clerk::trace!("{}", self.counter.load(Ordering::SeqCst));
        sleep(Duration::from_millis(50)).await;
        Ok(())
    }
}

#[tokio::test]
async fn test_dispatcher() {
    clerk::init_log_with_level(LevelFilter::TRACE);
    let counter = Arc::new(AtomicUsize::new(0));

    let mut pool = ResourcePool::new();
    pool.register(ResourceKey::from("cpu"), 2).unwrap();

    let handle = start_dispatcher::<TestPayload>(pool);

    let job1 = Job::new(
        "job1",
        TestPayload {
            counter: counter.clone(),
        },
        ResourceRequest::new(vec![(ResourceKey::from("cpu"), 1)]),
    );

    let job2 = Job::new(
        "job2",
        TestPayload {
            counter: counter.clone(),
        },
        ResourceRequest::new(vec![(ResourceKey::from("cpu"), 1)]),
    );
    let job3 = Job::new(
        "job3",
        TestPayload {
            counter: counter.clone(),
        },
        ResourceRequest::new(vec![(ResourceKey::from("cpu"), 1)]),
    );
    handle.submit(job1).await.unwrap();
    handle.submit(job2).await.unwrap();
    handle.submit(job3).await.unwrap();

    sleep(Duration::from_millis(300)).await;
    handle.shutdown().await;
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}
