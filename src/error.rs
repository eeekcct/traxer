#[derive(Debug)]
pub enum InitError {
    InvalidFilterDirectives(String),
    InstallErrorReporter(String),
    SetGlobalDefault(String),
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFilterDirectives(details) => {
                write!(f, "invalid filter directives: {details}")
            }
            Self::InstallErrorReporter(details) => {
                write!(f, "failed to install error reporter: {details}")
            }
            Self::SetGlobalDefault(details) => {
                write!(f, "failed to set global tracing subscriber: {details}")
            }
        }
    }
}

impl std::error::Error for InitError {}
