mod json;
mod plain;

use tracing::Event;
use tracing_subscriber::{
    fmt::{FmtContext, FormatEvent, FormatFields, format::Writer},
    registry::LookupSpan,
};

pub use json::JsonFormatter;
pub use plain::PlainFormatter;

pub enum Formatter {
    Plain(PlainFormatter),
    Json(JsonFormatter),
}

impl<S, N> FormatEvent<S, N> for Formatter
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Plain(formatter) => formatter.format_event(ctx, writer, event),
            Self::Json(formatter) => formatter.format_event(ctx, writer, event),
        }
    }
}
