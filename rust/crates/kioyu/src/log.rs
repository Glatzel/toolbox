use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::sync::Mutex;

use chrono::Local;
use clerk::tracing_subscriber::layer::Context;
use clerk::tracing_subscriber::registry::LookupSpan;
use clerk::tracing_subscriber::{self, Layer};
use clerk::{ClerkFormatter, FormatEventToWriter, file_layer, tracing_core};
use tracing_core::{Event, Subscriber};
pub const KIOYU_JOB_SPAN: &str = "kioyu-job";

struct JobId(String, String);

struct JobIdVisitor {
    id: Option<String>,
    name: Option<String>,
}

impl JobIdVisitor {
    fn new() -> Self {
        Self {
            id: None,
            name: None,
        }
    }
}

impl tracing_core::field::Visit for JobIdVisitor {
    fn record_str(&mut self, field: &tracing_core::Field, value: &str) {
        match field.name() {
            "job.id" => self.id = Some(value.to_owned()),
            "job.name" => self.name = Some(value.to_owned()),
            _ => {}
        }
    }

    fn record_debug(&mut self, field: &tracing_core::Field, value: &dyn std::fmt::Debug) {
        match field.name() {
            "job.id" => self.id = Some(format!("{:?}", value)),
            "job.name" => self.name = Some(format!("{:?}", value)),
            _ => {}
        }
    }
}

struct JobFileLayer {
    jobs_dir: PathBuf,
    handles: Mutex<HashMap<String, File>>,
    formatter: ClerkFormatter,
}

impl JobFileLayer {
    pub fn new(jobs_dir: impl Into<PathBuf>) -> Self {
        Self {
            jobs_dir: jobs_dir.into(),
            handles: Mutex::new(HashMap::new()),
            formatter: ClerkFormatter { color: false },
        }
    }
}

impl<S> Layer<S> for JobFileLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_new_span(
        &self,
        attrs: &tracing_core::span::Attributes<'_>,
        id: &tracing_core::span::Id,
        ctx: Context<'_, S>,
    ) {
        if attrs.metadata().name() != KIOYU_JOB_SPAN {
            return;
        }
        let mut visitor = JobIdVisitor::new();
        attrs.record(&mut visitor);
        if let (Some(job_id), Some(job_name)) = (visitor.id, visitor.name)
            && let Some(span) = ctx.span(id)
        {
            span.extensions_mut().insert(JobId(job_id, job_name));
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let job_span = ctx.lookup_current().and_then(|span| {
            std::iter::successors(Some(span), |s| s.parent()).find(|s| s.name() == KIOYU_JOB_SPAN)
        });

        let Some(job_span) = job_span else { return };

        let exts = job_span.extensions();
        let Some(job_id) = exts.get::<JobId>() else {
            return;
        };

        let mut handles = self.handles.lock().unwrap();
        let file = handles.entry(job_id.0.clone()).or_insert_with(|| {
            let filename = format!(
                "{}.{}.{}.log",
                Local::now().format("%Y-%m-%dT%H-%M-%S-%6fZ"),
                job_id.1, // job name
                job_id.0, // job id
            );
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(self.jobs_dir.join(filename))
                .unwrap()
        });

        self.formatter.format_to_writer(file, event);
    }
}

/// Sets up kioyu's logging layers under `log_root`.
///
/// ```text
/// <log_root>/
/// └── kioyu/
///     └── 2024-01-15-14-30-00-123/
///         ├── kioyu.log
///         └── jobs/
///             ├── <time>.<name>.<id>.log
///             └── ...
/// ```
///
/// # Span filter limitation
///
/// `kioyu[job]=off` currently does **not work** as expected due to a limitation
/// in `tracing` span filtering. Use [`clerk::NotInSpanFilter`] instead.
///
/// See: <https://github.com/tokio-rs/tracing/issues/1181>
///
/// # Examples
///
/// ```
/// use clerk::tracing_subscriber::layer::SubscriberExt;
/// use clerk::tracing_subscriber::util::SubscriberInitExt;
/// use clerk::tracing_subscriber::{EnvFilter, Layer};
/// use clerk::{LevelFilter, NotInSpanFilter, tracing_subscriber};
/// use tempfile::tempdir;
///
/// let log_root = tempdir().unwrap();
/// clerk::tracing_subscriber::registry()
///     .with(
///         kioyu::kioyu_layers::<tracing_subscriber::Registry>(log_root.path())
///             .with_filter(LevelFilter::TRACE),
///     )
///     .with(
///         clerk::terminal_layer(true)
///             .with_filter(LevelFilter::TRACE)
///             .with_filter(NotInSpanFilter(kioyu::KIOYU_JOB_SPAN)),
///     )
///     .init();
/// ```
pub fn kioyu_layers<S>(
    log_root: impl AsRef<std::path::Path>,
) -> Vec<Box<dyn Layer<S> + Send + Sync>>
where
    S: Subscriber + for<'a> LookupSpan<'a> + Send + Sync,
{
    let run_dir = log_root
        .as_ref()
        .join("kioyu")
        .join(Local::now().format("%Y-%m-%dT%H-%M-%S-%6fZ").to_string());

    let jobs_dir = run_dir.join("jobs");
    std::fs::create_dir_all(&jobs_dir).unwrap();

    let kioyu_log = file_layer(run_dir.join("kioyu.log"), true).with_filter(
        tracing_subscriber::filter::filter_fn(|meta| meta.target().starts_with("kioyu")),
    );

    let job_log = JobFileLayer::new(jobs_dir);

    vec![kioyu_log.boxed(), job_log.boxed()]
}
