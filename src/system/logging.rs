//! ## Logging
//!
//! `logging` is the module which initializes the logging system for termscp

// locals
use crate::system::environment::{get_log_paths, init_config_dir};
use crate::utils::file::open_file;
// ext
pub use simplelog::LevelFilter as LogLevel;
use simplelog::{ConfigBuilder, WriteLogger};
use std::fs::File;
use std::path::PathBuf;

/// ### init
///
/// Initialize logger
pub fn init(level: LogLevel) -> Result<(), String> {
    // Init config dir
    let config_dir: PathBuf = match init_config_dir() {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Err(String::from(
                "This system doesn't seem to support CONFIG_DIR",
            ))
        }
        Err(err) => return Err(err),
    };
    let log_file_path: PathBuf = get_log_paths(config_dir.as_path());
    // Open log file
    let file: File = open_file(log_file_path.as_path(), true, true, false)
        .map_err(|e| format!("Failed to open file {}: {}", log_file_path.display(), e))?;
    // Prepare log config
    let config = ConfigBuilder::new().set_time_format_rfc3339().build();
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
