use crate::config::{Color, Config, OutputFormat, Stream};
use crate::error::InitError;
use crate::formatter::{Formatter, JsonFormatter, PlainFormatter};
use serde_json::Value;
use std::{collections::BTreeMap, io::IsTerminal, sync::OnceLock};
use tracing_subscriber::{EnvFilter, fmt::time::SystemTime};

static INIT: OnceLock<()> = OnceLock::new();

pub fn init(cfg: Config) {
    let _ = try_init(cfg);
}

pub fn try_init(cfg: Config) -> Result<(), InitError> {
    if INIT.get().is_some() {
        return Ok(());
    }

    if cfg.error_report {
        color_eyre::install().map_err(|err| InitError::InstallErrorReporter(err.to_string()))?;
    }

    let filter = build_filter(&cfg)?;
    let use_color = resolve_color(&cfg);
    let base_fields = build_base_fields(&cfg);
    let formatter = match cfg.output_format {
        OutputFormat::Json => Formatter::Json(JsonFormatter {
            name: cfg.name.clone(),
            base_fields: base_fields.clone(),
            span: cfg.span,
        }),
        OutputFormat::Plain => Formatter::Plain(PlainFormatter {
            name: cfg.name.clone(),
            color: use_color,
            base_fields,
            span: cfg.span,
        }),
    };

    let result = match cfg.stream {
        Stream::Stdout => {
            let subscriber = tracing_subscriber::fmt()
                .with_env_filter(filter)
                .with_target(false)
                .with_timer(SystemTime)
                .with_writer(std::io::stdout)
                .event_format(formatter)
                .finish();
            tracing::subscriber::set_global_default(subscriber)
        }
        Stream::Stderr => {
            let subscriber = tracing_subscriber::fmt()
                .with_env_filter(filter)
                .with_target(false)
                .with_timer(SystemTime)
                .with_writer(std::io::stderr)
                .event_format(formatter)
                .finish();
            tracing::subscriber::set_global_default(subscriber)
        }
    };

    result.map_err(|err| InitError::SetGlobalDefault(err.to_string()))?;
    let _ = INIT.set(());
    Ok(())
}

pub fn is_initialized() -> bool {
    INIT.get().is_some()
}

fn build_filter(cfg: &Config) -> Result<EnvFilter, InitError> {
    if let Ok(filter) = EnvFilter::try_from_default_env() {
        return Ok(filter);
    }

    if let Some(directives) = &cfg.filter_directives {
        return EnvFilter::try_new(directives.as_ref())
            .map_err(|err| InitError::InvalidFilterDirectives(err.to_string()));
    }

    let level = match cfg.quiet {
        0 => match cfg.verbose {
            0 => "info",
            1 => "debug",
            _ => "trace",
        },
        1 => "warn",
        _ => "error",
    };
    Ok(EnvFilter::new(level))
}

fn resolve_color(cfg: &Config) -> bool {
    match cfg.color {
        Color::Always => true,
        Color::Never => false,
        Color::Auto => {
            if std::env::var_os("NO_COLOR").is_some() {
                return false;
            }
            match cfg.stream {
                Stream::Stdout => std::io::stdout().is_terminal(),
                Stream::Stderr => std::io::stderr().is_terminal(),
            }
        }
    }
}

fn build_base_fields(cfg: &Config) -> Vec<(String, Value)> {
    let mut fields = BTreeMap::new();

    if cfg.include_pid {
        fields.insert("pid".to_string(), Value::Number(std::process::id().into()));
    }

    if cfg.include_exe {
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(name) = exe_path.file_name() {
                fields.insert(
                    "exe".to_string(),
                    Value::String(name.to_string_lossy().into_owned()),
                );
            }
        }
    }

    if cfg.include_version {
        fields.insert(
            "version".to_string(),
            Value::String(env!("CARGO_PKG_VERSION").to_string()),
        );
    }

    for (key, value) in &cfg.base_fields {
        fields.insert(key.to_string(), value.clone());
    }

    fields.into_iter().collect()
}
