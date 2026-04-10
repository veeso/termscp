//! ## fmt
//!
//! format utilities

use std::time::SystemTime;

use chrono::{DateTime, Utc};

/// Format time using fmt string in utc time
pub fn fmt_time_utc(time: SystemTime, fmt: &str) -> String {
    let datetime: DateTime<Utc> = time.into();
    format!("{}", datetime.format(fmt))
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_fmt_time() {
        let system_time: SystemTime = SystemTime::UNIX_EPOCH;
        assert_eq!(
            fmt_time_utc(system_time, "%Y-%m-%d %H:%M"),
            String::from("1970-01-01 00:00")
        );
    }
}
