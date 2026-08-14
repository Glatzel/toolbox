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
use mischief::IntoMischief;
use tempfile::tempdir;
use tokio::sync::mpsc;
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
/// Signals on `attempt_tx` after every execute() attempt, and on
/// `post_process_tx` after every post_process() call.
struct FailingPayload {
    fails_first: usize,
    execute_count: Arc<AtomicUsize>,
    post_process_count: Arc<AtomicUsize>,
    attempt_tx: mpsc::UnboundedSender<()>,
    post_process_tx: mpsc::UnboundedSender<()>,
}

impl FailingPayload {
    fn new(
        fails_first: usize,
    ) -> (
        Self,
        Arc<AtomicUsize>,
        Arc<AtomicUsize>,
        mpsc::UnboundedReceiver<()>,
        mpsc::UnboundedReceiver<()>,
    ) {
        let execute_count = Arc::new(AtomicUsize::new(0));
        let post_process_count = Arc::new(AtomicUsize::new(0));
        let (attempt_tx, attempt_rx) = mpsc::unbounded_channel();
        let (post_process_tx, post_process_rx) = mpsc::unbounded_channel();
        (
            Self {
                fails_first,
                execute_count: execute_count.clone(),
                post_process_count: post_process_count.clone(),
                attempt_tx,
                post_process_tx,
            },
            execute_count,
            post_process_count,
            attempt_rx,
            post_process_rx,
        )
    }
}

#[async_trait]
impl IPayload for FailingPayload {
    type Error = mischief::Report;

    async fn execute(&self, _cancel: CancellationToken) -> Result<(), Self::Error> {
        let attempt = self.execute_count.fetch_add(1, Ordering::SeqCst) + 1;
        let result = if attempt <= self.fails_first {
            Err(mischief::mischief!(
                "intentional failure on attempt {attempt}"
            ))
        } else {
            Ok(())
        };
        // Fire after the attempt is fully recorded, regardless of outcome.
        let _ = self.attempt_tx.send(());
        result
    }

    async fn post_process(&self) -> Result<(), Self::Error> {
        self.post_process_count.fetch_add(1, Ordering::SeqCst);
        let _ = self.post_process_tx.send(());
        Ok(())
    }
}

/// Waits for `n` signals on the channel, with a timeout as a safety net
/// (not the actual sync mechanism — just prevents a hang from becoming
/// a silent CI timeout with no useful message).
async fn wait_for_n(rx: &mut mpsc::UnboundedReceiver<()>, n: usize, what: &str) {
    for i in 0..n {
        tokio::time::timeout(Duration::from_secs(5), rx.recv())
            .await
            .unwrap_or_else(|_| panic!("timed out waiting for {what} (got {i}/{n})"))
            .unwrap_or_else(|| panic!("channel closed while waiting for {what} (got {i}/{n})"));
    }
}
// ── dispatcher smoke tests (unchanged) ───────────────────────────────────────

enum DispatcherMode {
    Limited,
    Unlimited,
}

async fn run_dispatcher_test(mode: DispatcherMode, snapshot_name: &str) -> mischief::Result<()> {
    let log_root = tempdir().unwrap();

    clerk::tracing_subscriber::registry()
        .with(
            kioyu_layers::<tracing_subscriber::Registry, _>(log_root.path())
                .into_mischief()?
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
    Ok(())
}

#[tokio::test]
async fn test_dispatcher() -> mischief::Result<()> {
    run_dispatcher_test(DispatcherMode::Limited, "kioyu_log_dir_tree").await
}

#[tokio::test]
async fn test_dispatcher_unlimited() -> mischief::Result<()> {
    run_dispatcher_test(DispatcherMode::Unlimited, "kioyu_unlimited_log_dir_tree").await
}

// ── retry tests
// ───────────────────────────────────────────────────────────────

/// A job that fails once and then succeeds should be retried and complete
/// successfully. `post_process` must be called exactly once.
#[tokio::test]
async fn test_retry_succeeds() {
    clerk::init_log_with_level(LevelFilter::TRACE);
    let handle = start_dispatcher_unlimited::<FailingPayload>();

    let (payload, execute_count, post_process_count, mut attempt_rx, mut post_process_rx) =
        FailingPayload::new(1);

    handle
        .submit(Job::new("retry-job", payload, ResourceRequest::none(), 1))
        .await
        .unwrap();

    // Wait for exactly the two attempts we expect (1 failure + 1 success),
    // then the one post_process call — no matter how long each takes.
    wait_for_n(&mut attempt_rx, 2, "execute attempts").await;
    wait_for_n(&mut post_process_rx, 1, "post_process call").await;

    handle.shutdown().await;

    assert_eq!(execute_count.load(Ordering::SeqCst), 2);
    assert_eq!(post_process_count.load(Ordering::SeqCst), 1);
}

/// A job whose payload always fails should be attempted `max_retries + 1`
/// times and then abandoned. `post_process` must never be called.
#[tokio::test]
async fn test_retry_exhausted() {
    clerk::init_log_with_level(LevelFilter::TRACE);
    let handle = start_dispatcher_unlimited::<FailingPayload>();

    let (payload, execute_count, post_process_count, mut attempt_rx, _post_process_rx) =
        FailingPayload::new(99);

    handle
        .submit(Job::new(
            "exhausted-job",
            payload,
            ResourceRequest::none(),
            2,
        ))
        .await
        .unwrap();

    // Wait for all 3 attempts (1 + 2 retries) regardless of how long
    // backtrace capture etc. makes each one take.
    wait_for_n(&mut attempt_rx, 3, "execute attempts").await;

    handle.shutdown().await;

    assert_eq!(execute_count.load(Ordering::SeqCst), 3);
    assert_eq!(post_process_count.load(Ordering::SeqCst), 0);
}

/// Resources should be freed after retry exhaustion so that other queued
/// jobs can run. Submits an exhausting job followed by a normal job into
/// a pool with capacity 1 and asserts the normal job eventually completes.
#[tokio::test]
async fn test_retry_exhaustion_frees_resources() {
    clerk::init_log_with_level(LevelFilter::TRACE);
    let handle = start_dispatcher_unlimited::<FailingPayload>();

    let (exhausted_payload, exhausted_exec, _, mut exhausted_rx, _) = FailingPayload::new(99);
    let (succeeding_payload, succeeding_exec, _, mut succeeding_rx, _) = FailingPayload::new(0);

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

    wait_for_n(&mut exhausted_rx, 2, "exhausted job attempts").await;
    wait_for_n(&mut succeeding_rx, 1, "succeeding job attempt").await;

    handle.shutdown().await;

    assert_eq!(exhausted_exec.load(Ordering::SeqCst), 2);
    assert_eq!(succeeding_exec.load(Ordering::SeqCst), 1);
}
