use serde_json::Value;
use std::borrow::Cow;
use tracing::{Event, Level};
use tracing_subscriber::{
    fmt::{FmtContext, FormatEvent, FormatFields, format::Writer},
    registry::LookupSpan,
};

pub struct PlainFormatter {
    pub name: Cow<'static, str>,
    pub color: bool,
    pub base_fields: Vec<(String, Value)>,
    pub span: bool,
}

impl<S, N> FormatEvent<S, N> for PlainFormatter
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        write!(writer, "[{}] ", self.name)?;
        write_level(&mut writer, event.metadata().level(), self.color)?;
        write!(writer, " ")?;
        ctx.format_fields(writer.by_ref(), event)?;

        for (key, value) in &self.base_fields {
            write!(writer, " {key}={value}")?;
        }

        if self.span {
            if let Some(span) = ctx.lookup_current() {
                write!(writer, " span={}", span.name())?;
            }
        }

        writeln!(writer)
    }
}

fn write_level(w: &mut Writer<'_>, level: &Level, color: bool) -> std::fmt::Result {
    if !color {
        return write!(w, "{level}");
    }

    let (level_name, code) = match *level {
        Level::ERROR => ("ERROR", 31),
        Level::WARN => ("WARN", 33),
        Level::INFO => ("INFO", 32),
        Level::DEBUG => ("DEBUG", 34),
        Level::TRACE => ("TRACE", 90),
    };

    write!(w, "\x1b[{code}m{level_name}\x1b[0m")
}
