//! ## Mock
//!
//! Contains mock for test units

pub mod ssh;
// -- logger

pub fn logger() {
    use std::sync::Once;

    static INIT: Once = Once::new();

    INIT.call_once(|| {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Trace)
            .is_test(true)
            .format_source_path(true)
            .format_line_number(true)
            .try_init();
    });
}
