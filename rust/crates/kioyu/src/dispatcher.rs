use std::collections::VecDeque;

use tokio::sync::mpsc;

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
            "job",
            job.id = %job.id,
            job.name = %job.name,
        );

        tokio::spawn(async move {
            clerk::debug!("executing");

            {
                let _enter = span.enter();
                match job.payload.execute().await {
                    Ok(_) => clerk::debug!("finished"),
                    Err(e) => clerk::error!("payload error: {}", e),
                }
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
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use arbor::indents::UnicodeIndent;
    use arbor::renders::OwnedRender;
    use arbor::trees::OwnedTree;
    use async_trait::async_trait;
    use clerk::tracing::Span;
    use clerk::tracing_subscriber::layer::SubscriberExt;
    use clerk::tracing_subscriber::util::SubscriberInitExt;
    use clerk::tracing_subscriber::{EnvFilter, Layer};
    use clerk::{LevelFilter, tracing_subscriber};
    use tempfile::tempdir;
    use tokio::time::{Duration, sleep};

    use super::*;
    use crate::job::ResourceRequest;
    use crate::kioyu_layers;
    use crate::resource::ResourceKey;
    fn dir_tree(dir: &std::path::Path) -> OwnedTree<String> {
        let name = dir
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| dir.to_string_lossy().into_owned());

        let mut node = OwnedTree::new(name);

        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut paths: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
            paths.sort();

            for path in paths {
                if path.is_dir() {
                    node.push(dir_tree(&path));
                } else {
                    // let content = std::fs::read_to_string(&path).unwrap_or_default();
                    // clerk::debug!("{}: `{}`", path.to_string_lossy().into_owned(), content);
                    node.push(OwnedTree::new(
                        path.file_name().unwrap().to_string_lossy().into_owned(),
                    ));
                }
            }
        }

        node
    }
    struct TestPayload {
        counter: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl IPayload for TestPayload {
        type Error = mischief::Report;
        async fn execute(&self) -> Result<(), Self::Error> {
            self.counter.fetch_add(1, Ordering::SeqCst);
            clerk::trace!(
                "{}, {}",
                self.counter.load(Ordering::SeqCst),
                Span::current().metadata().unwrap().name()
            );
            sleep(Duration::from_millis(50)).await;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_dispatcher() {
        let log_root = tempdir().unwrap();

        clerk::tracing_subscriber::registry()
            .with(
                kioyu_layers::<tracing_subscriber::Registry>(log_root.path())
                    .with_filter(clerk::level_filter(LevelFilter::TRACE)),
            )
            .with(
                clerk::terminal_layer(true).with_filter(
                    EnvFilter::new(LevelFilter::TRACE.to_string())
                        .add_directive("kioyu[job]=off".parse().unwrap()),
                ),
            )
            .init();
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

        //check log output
        let tree = dir_tree(log_root.path());
        let render = OwnedRender {
            tree: &tree,
            indent: UnicodeIndent,
            width: 0,
        };
        println!("{}", render);
        insta::with_settings!({filters => vec![
            (r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", "[UUID]"),
            (r"\d{4}-\d{2}-\d{2}-\d{2}-\d{2}-\d{2}-\d{3}", "[TIMESTAMP]"),
            (r"\.tmp\w+", "[LOG_ROOT_DIR]")
        ]}, {
            insta::assert_snapshot!("kioyu_log_dir_tree", render);
        });
    }
}
