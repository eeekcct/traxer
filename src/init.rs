use crate::config::{Color, Config, ConfigOverride, OutputFormat, Stream};
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

    let cfg = resolve_config(cfg);

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

fn resolve_config(mut cfg: Config) -> Config {
    let is_tty = is_tty(cfg.stream);
    if let Some(policy) = cfg.policy.take() {
        let override_cfg = if is_tty { policy.tty } else { policy.non_tty };
        apply_override(&mut cfg, override_cfg);
    }
    cfg
}

fn is_tty(stream: Stream) -> bool {
    match stream {
        Stream::Stdout => std::io::stdout().is_terminal(),
        Stream::Stderr => std::io::stderr().is_terminal(),
    }
}

fn apply_override(cfg: &mut Config, ov: ConfigOverride) {
    if let Some(v) = ov.output_format {
        cfg.output_format = v;
    }
    if let Some(v) = ov.stream {
        cfg.stream = v;
    }
    if let Some(v) = ov.color {
        cfg.color = v;
    }
    if let Some(v) = ov.verbose {
        cfg.verbose = v.min(2);
    }
    if let Some(v) = ov.quiet {
        cfg.quiet = v.min(2);
    }
    if let Some(v) = ov.filter_directives {
        cfg.filter_directives = Some(v);
    }
    if let Some(v) = ov.base_fields {
        cfg.base_fields = v;
    }
    if let Some(v) = ov.include_pid {
        cfg.include_pid = v;
    }
    if let Some(v) = ov.include_exe {
        cfg.include_exe = v;
    }
    if let Some(v) = ov.include_version {
        cfg.include_version = v;
    }
    if let Some(v) = ov.span {
        cfg.span = v;
    }
    if let Some(v) = ov.error_report {
        cfg.error_report = v;
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ConfigOverride, Policy};

    #[test]
    fn resolve_config_uses_tty_override() {
        let cfg = Config::new("x").policy(Policy {
            tty: ConfigOverride::new()
                .output_format(OutputFormat::Plain)
                .color(Color::Always)
                .span(true),
            non_tty: ConfigOverride::new().output_format(OutputFormat::Json),
        });
        let mut cfg2 = cfg.clone();
        cfg2.policy = cfg.policy.clone();

        let mut resolved = cfg2;
        if let Some(policy) = resolved.policy.take() {
            apply_override(&mut resolved, policy.tty);
        }

        assert!(matches!(resolved.output_format, OutputFormat::Plain));
        assert!(matches!(resolved.color, Color::Always));
        assert!(resolved.span);
    }

    #[test]
    fn apply_override_replaces_all_supported_fields() {
        let mut cfg = Config::new("x")
            .plain()
            .stream(Stream::Stderr)
            .color(Color::Auto)
            .verbose(0)
            .quiet(0)
            .span(false)
            .error_report(true);

        let ov = ConfigOverride::new()
            .output_format(OutputFormat::Json)
            .stream(Stream::Stdout)
            .color(Color::Never)
            .verbose(2)
            .quiet(1)
            .filter_directives("a=debug")
            .base_fields(vec![("k".into(), Value::String("v".into()))])
            .include_pid(true)
            .include_exe(true)
            .include_version(true)
            .span(true)
            .error_report(false);
        apply_override(&mut cfg, ov);

        assert!(matches!(cfg.output_format, OutputFormat::Json));
        assert!(matches!(cfg.stream, Stream::Stdout));
        assert!(matches!(cfg.color, Color::Never));
        assert_eq!(cfg.verbose, 2);
        assert_eq!(cfg.quiet, 1);
        assert_eq!(cfg.filter_directives.as_deref(), Some("a=debug"));
        assert_eq!(cfg.base_fields.len(), 1);
        assert!(cfg.include_pid);
        assert!(cfg.include_exe);
        assert!(cfg.include_version);
        assert!(cfg.span);
        assert!(!cfg.error_report);
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
