use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use arbor::indents::UnicodeIndent;
use arbor::renders::OwnedRender;
use arbor::trees::OwnedTree;
use async_trait::async_trait;
use clerk::tracing::Span;
use clerk::tracing_subscriber::Layer;
use clerk::tracing_subscriber::layer::SubscriberExt;
use clerk::tracing_subscriber::util::SubscriberInitExt;
use clerk::{LevelFilter, NotInSpanFilter, tracing_subscriber};
use kioyu::{
    CancellationToken, IPayload, Job, KIOYU_JOB_SPAN, ResourceKey, ResourcePool, ResourceRequest,
    kioyu_layers, start_dispatcher, start_dispatcher_unlimited,
};
use tempfile::tempdir;
use tokio::time::{Duration, sleep};

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
    async fn execute(&self, _cancel: CancellationToken) -> Result<(), Self::Error> {
        self.counter.fetch_add(1, Ordering::SeqCst);
        clerk::trace!(
            "{}, {}[{}]",
            self.counter.load(Ordering::SeqCst),
            Span::current().metadata().unwrap().target(),
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
                .with_filter(LevelFilter::TRACE),
        )
        .with(
            clerk::terminal_layer(true)
                .with_filter(LevelFilter::TRACE)
                .with_filter(NotInSpanFilter(KIOYU_JOB_SPAN)),
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
        (r"\d{4}-\d{2}-\d{2}T\d{2}-\d{2}-\d{2}-\d{6}Z", "[TIMESTAMP]"),
        (r"\.tmp\w+", "[LOG_ROOT_DIR]")
    ]}, {
        insta::assert_snapshot!("kioyu_log_dir_tree", render);
    });
}
#[tokio::test]
async fn test_dispatcher_unlimited() {
    let log_root = tempdir().unwrap();

    clerk::tracing_subscriber::registry()
        .with(
            kioyu_layers::<tracing_subscriber::Registry>(log_root.path())
                .with_filter(LevelFilter::TRACE),
        )
        .with(
            clerk::terminal_layer(true)
                .with_filter(LevelFilter::TRACE)
                .with_filter(NotInSpanFilter(KIOYU_JOB_SPAN)),
        )
        .init();
    let counter = Arc::new(AtomicUsize::new(0));

    let handle = start_dispatcher_unlimited::<TestPayload>();

    let job1 = Job::new(
        "job1",
        TestPayload {
            counter: counter.clone(),
        },
        ResourceRequest::none(),
    );

    let job2 = Job::new(
        "job2",
        TestPayload {
            counter: counter.clone(),
        },
        ResourceRequest::none(),
    );
    let job3 = Job::new(
        "job3",
        TestPayload {
            counter: counter.clone(),
        },
        ResourceRequest::none(),
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
        (r"\d{4}-\d{2}-\d{2}T\d{2}-\d{2}-\d{2}-\d{6}Z", "[TIMESTAMP]"),
        (r"\.tmp\w+", "[LOG_ROOT_DIR]")
    ]}, {
        insta::assert_snapshot!("kioyu_unlimited_log_dir_tree", render);
    });
}
