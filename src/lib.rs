mod config;
mod error;
mod formatter;
mod init;
pub use config::{Color, Config, OutputFormat, Stream};
pub use error::InitError;
pub use init::{init, is_initialized, try_init};
pub use tracing::{debug, error, info, trace, warn};
