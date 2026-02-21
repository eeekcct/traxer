use serde_json::{Map, Number, Value};
use std::{
    borrow::Cow,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::Event;
use tracing_subscriber::{
    fmt::{FmtContext, FormatEvent, FormatFields, format::Writer},
    registry::LookupSpan,
};

pub struct JsonFormatter {
    pub name: Cow<'static, str>,
    pub base_fields: Vec<(String, Value)>,
    pub span: bool,
}

impl<S, N> FormatEvent<S, N> for JsonFormatter
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
        let mut map = Map::new();
        map.insert("timestamp".to_string(), unix_timestamp_millis());
        map.insert(
            "level".to_string(),
            Value::String(event.metadata().level().to_string()),
        );
        map.insert("name".to_string(), Value::String(self.name.to_string()));

        {
            struct JsonFieldVisitor<'a> {
                map: &'a mut Map<String, Value>,
            }

            impl tracing_subscriber::field::Visit for JsonFieldVisitor<'_> {
                fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
                    self.map
                        .insert(field.name().to_string(), Value::String(value.to_string()));
                }

                fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
                    self.map
                        .insert(field.name().to_string(), Value::Bool(value));
                }

                fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
                    self.map
                        .insert(field.name().to_string(), Value::Number(value.into()));
                }

                fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
                    self.map
                        .insert(field.name().to_string(), Value::Number(value.into()));
                }

                fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
                    let json_value = Number::from_f64(value)
                        .map(Value::Number)
                        .unwrap_or_else(|| Value::String(value.to_string()));
                    self.map.insert(field.name().to_string(), json_value);
                }

                fn record_error(
                    &mut self,
                    field: &tracing::field::Field,
                    value: &(dyn std::error::Error + 'static),
                ) {
                    self.map
                        .insert(field.name().to_string(), Value::String(value.to_string()));
                }

                fn record_debug(
                    &mut self,
                    field: &tracing::field::Field,
                    value: &dyn std::fmt::Debug,
                ) {
                    self.map.insert(
                        field.name().to_string(),
                        Value::String(format!("{value:?}")),
                    );
                }
            }

            event.record(&mut JsonFieldVisitor { map: &mut map });
        }

        for (key, value) in &self.base_fields {
            map.insert(key.clone(), value.clone());
        }

        if self.span {
            if let Some(span) = ctx.lookup_current() {
                map.insert("span".to_string(), Value::String(span.name().to_string()));
            }
        }

        map.entry("message".to_string())
            .or_insert_with(|| Value::String(String::new()));

        let object = Value::Object(map);
        writeln!(writer, "{object}")
    }
}

fn unix_timestamp_millis() -> Value {
    let millis = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis(),
        Err(_) => 0,
    };

    let millis = u64::try_from(millis).unwrap_or(u64::MAX);
    Value::Number(millis.into())
}
