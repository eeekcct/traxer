use serde_json::Value;
use std::borrow::Cow;

#[derive(Clone, Copy, Debug)]
pub enum OutputFormat {
    Plain,
    Json,
}

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Auto,
    Always,
    Never,
}

#[derive(Clone, Copy, Debug)]
pub enum Stream {
    Stdout,
    Stderr,
}

#[derive(Clone, Debug, Default)]
pub struct ConfigOverride {
    pub output_format: Option<OutputFormat>,
    pub stream: Option<Stream>,
    pub color: Option<Color>,
    pub verbose: Option<u8>,
    pub quiet: Option<u8>,
    pub filter_directives: Option<Cow<'static, str>>,
    pub base_fields: Option<Vec<(Cow<'static, str>, Value)>>,
    pub include_pid: Option<bool>,
    pub include_exe: Option<bool>,
    pub include_version: Option<bool>,
    pub span: Option<bool>,
    pub error_report: Option<bool>,
}

impl ConfigOverride {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn output_format(mut self, output_format: OutputFormat) -> Self {
        self.output_format = Some(output_format);
        self
    }
    pub fn stream(mut self, stream: Stream) -> Self {
        self.stream = Some(stream);
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub fn verbose(mut self, verbose: u8) -> Self {
        self.verbose = Some(verbose.min(2));
        self
    }
    pub fn quiet(mut self, quiet: u8) -> Self {
        self.quiet = Some(quiet.min(2));
        self
    }
    pub fn filter_directives(mut self, directives: impl Into<Cow<'static, str>>) -> Self {
        self.filter_directives = Some(directives.into());
        self
    }
    pub fn base_fields(mut self, base_fields: Vec<(Cow<'static, str>, Value)>) -> Self {
        self.base_fields = Some(base_fields);
        self
    }
    pub fn include_pid(mut self, enabled: bool) -> Self {
        self.include_pid = Some(enabled);
        self
    }
    pub fn include_exe(mut self, enabled: bool) -> Self {
        self.include_exe = Some(enabled);
        self
    }
    pub fn include_version(mut self, enabled: bool) -> Self {
        self.include_version = Some(enabled);
        self
    }
    pub fn span(mut self, enabled: bool) -> Self {
        self.span = Some(enabled);
        self
    }
    pub fn error_report(mut self, enabled: bool) -> Self {
        self.error_report = Some(enabled);
        self
    }
}

#[derive(Clone, Debug)]
pub struct Policy {
    pub tty: ConfigOverride,
    pub non_tty: ConfigOverride,
}

impl Policy {
    pub fn default_auto() -> Self {
        Self {
            tty: ConfigOverride::new()
                .output_format(OutputFormat::Plain)
                .color(Color::Auto)
                .span(false)
                .error_report(true)
                .verbose(0)
                .quiet(0)
                .include_pid(false)
                .include_version(false),
            non_tty: ConfigOverride::new()
                .output_format(OutputFormat::Json)
                .color(Color::Never)
                .span(false)
                .error_report(true)
                .verbose(0)
                .quiet(0)
                .include_pid(true)
                .include_version(true),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub name: Cow<'static, str>,
    pub policy: Option<Policy>,
    pub output_format: OutputFormat,
    pub stream: Stream,
    pub color: Color,
    pub verbose: u8,
    pub quiet: u8,
    pub filter_directives: Option<Cow<'static, str>>,
    pub base_fields: Vec<(Cow<'static, str>, Value)>,
    pub include_pid: bool,
    pub include_exe: bool,
    pub include_version: bool,
    pub span: bool,
    pub error_report: bool,
}

impl Config {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            policy: None,
            output_format: OutputFormat::Plain,
            stream: Stream::Stderr,
            color: Color::Auto,
            verbose: 0,
            quiet: 0,
            filter_directives: None,
            base_fields: Vec::new(),
            include_pid: false,
            include_exe: false,
            include_version: false,
            span: false,
            error_report: true,
        }
    }
    pub fn from_argv0() -> Self {
        let name = std::env::args()
            .next()
            .and_then(|p| {
                std::path::Path::new(&p)
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "name".to_string());
        Self::new(name)
    }
    pub fn plain(mut self) -> Self {
        self.output_format = OutputFormat::Plain;
        self
    }
    pub fn policy(mut self, policy: Policy) -> Self {
        self.policy = Some(policy);
        self
    }
    pub fn json(mut self) -> Self {
        self.output_format = OutputFormat::Json;
        self
    }
    pub fn stream(mut self, stream: Stream) -> Self {
        self.stream = stream;
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn verbose(mut self, verbose: u8) -> Self {
        self.verbose = verbose.min(2);
        self
    }
    pub fn quiet(mut self, quiet: u8) -> Self {
        self.quiet = quiet.min(2);
        self
    }
    pub fn with_filter_directives(mut self, directives: impl Into<Cow<'static, str>>) -> Self {
        self.filter_directives = Some(directives.into());
        self
    }
    pub fn with_base_field(
        mut self,
        key: impl Into<Cow<'static, str>>,
        value: impl Into<Value>,
    ) -> Self {
        self.base_fields.push((key.into(), value.into()));
        self
    }
    pub fn with_pid(mut self) -> Self {
        self.include_pid = true;
        self
    }
    pub fn with_exe(mut self) -> Self {
        self.include_exe = true;
        self
    }
    pub fn with_version(mut self) -> Self {
        self.include_version = true;
        self
    }
    pub fn span(mut self, enabled: bool) -> Self {
        self.span = enabled;
        self
    }
    pub fn error_report(mut self, enabled: bool) -> Self {
        self.error_report = enabled;
        self
    }
}
