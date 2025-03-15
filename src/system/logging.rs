//! ## Logging
//!
//! `logging` is the module which initializes the logging system for termscp

pub use simplelog::LevelFilter as LogLevel;
use simplelog::{ConfigBuilder, WriteLogger};

use super::environment::{get_log_paths, init_cache_dir};
use crate::utils::file::open_file;

/// Initialize logger
pub fn init(level: LogLevel) -> Result<(), String> {
    // Init cache dir
    let cache_dir = match init_cache_dir() {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Err(String::from(
                "This system doesn't seem to support CACHE_DIR",
            ));
        }
        Err(err) => return Err(err),
    };
    let log_file_path = get_log_paths(cache_dir.as_path());
    // Open log file
    let file = open_file(log_file_path.as_path(), true, true, false)
        .map_err(|e| format!("Failed to open file {}: {}", log_file_path.display(), e))?;
    // Prepare log config
    let config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .add_filter_allow_str("termscp")
        .add_filter_allow_str("remotefs")
        .build();
    // Make logger
    WriteLogger::init(level, config, file).map_err(|e| format!("Failed to initialize logger: {e}"))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_system_logging_setup() {
        assert!(init(LogLevel::Trace).is_ok());
    }
}
