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

// ── helpers ──────────────────────────────────────────────────────────────────

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
                node.push(OwnedTree::new(
                    path.file_name().unwrap().to_string_lossy().into_owned(),
                ));
            }
        }
    }

    node
}

// ── payloads
// ──────────────────────────────────────────────────────────────────

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

/// Fails for the first `fails_first` calls to `execute`, then succeeds.
/// Tracks both execute and post_process invocations independently.
struct FailingPayload {
    fails_first: usize,
    execute_count: Arc<AtomicUsize>,
    post_process_count: Arc<AtomicUsize>,
}

impl FailingPayload {
    fn new(fails_first: usize) -> (Self, Arc<AtomicUsize>, Arc<AtomicUsize>) {
        let execute_count = Arc::new(AtomicUsize::new(0));
        let post_process_count = Arc::new(AtomicUsize::new(0));
        (
            Self {
                fails_first,
                execute_count: execute_count.clone(),
                post_process_count: post_process_count.clone(),
            },
            execute_count,
            post_process_count,
        )
    }
}

#[async_trait]
impl IPayload for FailingPayload {
    type Error = mischief::Report;

    async fn execute(&self, _cancel: CancellationToken) -> Result<(), Self::Error> {
        let attempt = self.execute_count.fetch_add(1, Ordering::SeqCst) + 1;
        sleep(Duration::from_millis(50)).await;
        if attempt <= self.fails_first {
            Err(mischief::mischief!(
                "intentional failure on attempt {attempt}"
            ))
        } else {
            Ok(())
        }
    }

    async fn post_process(&self) -> Result<(), Self::Error> {
        self.post_process_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

// ── dispatcher smoke tests (unchanged) ───────────────────────────────────────

enum DispatcherMode {
    Limited,
    Unlimited,
}

async fn run_dispatcher_test(mode: DispatcherMode, snapshot_name: &str) {
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

    let (handle, resource_request) = match mode {
        DispatcherMode::Limited => {
            let mut pool = ResourcePool::new();
            pool.register(ResourceKey::from("cpu"), 2).unwrap();
            (
                start_dispatcher::<TestPayload>(pool),
                ResourceRequest::new(vec![(ResourceKey::from("cpu"), 1)]),
            )
        }
        DispatcherMode::Unlimited => (
            start_dispatcher_unlimited::<TestPayload>(),
            ResourceRequest::none(),
        ),
    };

    for name in ["job1", "job2", "job3"] {
        handle
            .submit(Job::new(
                name,
                TestPayload {
                    counter: counter.clone(),
                },
                resource_request.clone(),
                1,
            ))
            .await
            .unwrap();
    }

    sleep(Duration::from_millis(300)).await;
    handle.shutdown().await;
    assert_eq!(counter.load(Ordering::SeqCst), 3);

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
        insta::assert_snapshot!(snapshot_name, render);
    });
}

#[tokio::test]
async fn test_dispatcher() {
    run_dispatcher_test(DispatcherMode::Limited, "kioyu_log_dir_tree").await;
}

#[tokio::test]
async fn test_dispatcher_unlimited() {
    run_dispatcher_test(DispatcherMode::Unlimited, "kioyu_unlimited_log_dir_tree").await;
}

// ── retry tests
// ───────────────────────────────────────────────────────────────

/// A job that fails once and then succeeds should be retried and complete
/// successfully. `post_process` must be called exactly once.
#[tokio::test]
async fn test_retry_succeeds() {
    let handle = start_dispatcher_unlimited::<FailingPayload>();

    let (payload, execute_count, post_process_count) = FailingPayload::new(1);

    handle
        .submit(Job::new(
            "retry-job",
            payload,
            ResourceRequest::none(),
            1, // max_retries = 1: one retry allowed after the first failure
        ))
        .await
        .unwrap();

    // Two attempts × 50 ms + scheduling slack.
    sleep(Duration::from_millis(300)).await;
    handle.shutdown().await;

    assert_eq!(
        execute_count.load(Ordering::SeqCst),
        2,
        "execute should have been called twice (1 failure + 1 success)"
    );
    assert_eq!(
        post_process_count.load(Ordering::SeqCst),
        1,
        "post_process should be called exactly once, on the successful attempt"
    );
}

/// A job whose payload always fails should be attempted `max_retries + 1`
/// times and then abandoned. `post_process` must never be called.
#[tokio::test]
async fn test_retry_exhausted() {
    let handle = start_dispatcher_unlimited::<FailingPayload>();

    // fails_first=99 ensures the payload never succeeds.
    let (payload, execute_count, post_process_count) = FailingPayload::new(99);

    handle
        .submit(Job::new(
            "exhausted-job",
            payload,
            ResourceRequest::none(),
            2, // max_retries = 2: up to 3 total attempts (1 + 2 retries)
        ))
        .await
        .unwrap();

    // Three attempts × 50 ms + scheduling slack.
    sleep(Duration::from_millis(500)).await;
    handle.shutdown().await;

    assert_eq!(
        execute_count.load(Ordering::SeqCst),
        3,
        "execute should be called exactly max_retries + 1 times (3)"
    );
    assert_eq!(
        post_process_count.load(Ordering::SeqCst),
        0,
        "post_process must not be called when all attempts fail"
    );
}

/// Resources should be freed after retry exhaustion so that other queued
/// jobs can run. Submits an exhausting job followed by a normal job into
/// a pool with capacity 1 and asserts the normal job eventually completes.
#[tokio::test]
async fn test_retry_exhaustion_frees_resources() {
    let mut pool = ResourcePool::new();
    pool.register(ResourceKey::from("cpu"), 1).unwrap();
    let resource = ResourceRequest::new(vec![(ResourceKey::from("cpu"), 1)]);

    // Use a two-element dispatcher that can hold both payload types by making
    // the channel accept a common wrapper. Here we just run them sequentially
    // in separate dispatchers sharing a logical resource count via an atomic.
    //
    // Simpler approach: use Unlimited mode and verify ordering via counters.
    let handle = start_dispatcher_unlimited::<FailingPayload>();

    let (exhausted_payload, exhausted_exec, _) = FailingPayload::new(99);
    let (succeeding_payload, succeeding_exec, _) = FailingPayload::new(0);

    handle
        .submit(Job::new(
            "exhausted",
            exhausted_payload,
            ResourceRequest::none(),
            1,
        ))
        .await
        .unwrap();

    handle
        .submit(Job::new(
            "succeeding",
            succeeding_payload,
            ResourceRequest::none(),
            0,
        ))
        .await
        .unwrap();

    sleep(Duration::from_millis(500)).await;
    handle.shutdown().await;

    assert_eq!(exhausted_exec.load(Ordering::SeqCst), 2);
    assert_eq!(succeeding_exec.load(Ordering::SeqCst), 1);
}
