use tracing::{Metadata, Subscriber};
use tracing_subscriber::layer::{Context, Filter};
use tracing_subscriber::registry::LookupSpan;

pub struct NotInSpanFilter(pub &'static str);

impl<S> Filter<S> for NotInSpanFilter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn enabled(&self, _meta: &Metadata<'_>, ctx: &Context<'_, S>) -> bool {
        !ctx.lookup_current().is_some_and(|span| {
            std::iter::successors(Some(span), tracing_subscriber::registry::SpanRef::parent)
                .any(|s| s.name() == self.0)
        })
    }
}
