//! ## Parser
//!
//! parser utils

use std::time::{Duration, SystemTime};

use chrono::format::ParseError;
use chrono::prelude::*;
use chrono::LocalResult;

/// Convert ls syntax time to System Time
/// ls time has two possible syntax:
/// 1. if year is current: %b %d %H:%M (e.g. Nov 5 13:46)
/// 2. else: %b %d %Y (e.g. Nov 5 2019)
pub fn parse_lstime(tm: &str, fmt_year: &str, fmt_hours: &str) -> Result<SystemTime, ParseError> {
    let datetime: NaiveDateTime = match NaiveDate::parse_from_str(tm, fmt_year) {
        Ok(date) => {
            // Case 2.
            // Return NaiveDateTime from NaiveDate with time 00:00:00
            date.and_hms_opt(0, 0, 0).unwrap()
        }
        Err(_) => {
            // Might be case 1.
            // We need to add Current Year at the end of the string
            let this_year: i32 = Utc::now().year();
            let date_time_str: String = format!("{tm} {this_year}");
            // Now parse
            NaiveDateTime::parse_from_str(
                date_time_str.as_ref(),
                format!("{fmt_hours} %Y").as_ref(),
            )?
        }
    };
    // `ls -l` output is expressed in the remote host's local wall-clock time.
    // Interpret the parsed value as local time instead of UTC so the resulting
    // absolute timestamp matches what the remote filesystem actually reports.
    let datetime = match Local.from_local_datetime(&datetime) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(dt, _) => dt,
        LocalResult::None => Utc.from_utc_datetime(&datetime).with_timezone(&Local),
    };
    let timestamp = datetime.timestamp();
    if timestamp < 0 {
        return Ok(SystemTime::UNIX_EPOCH);
    }
    let sys_time: SystemTime = SystemTime::UNIX_EPOCH;
    Ok(sys_time
        .checked_add(Duration::from_secs(timestamp as u64))
        .unwrap_or(SystemTime::UNIX_EPOCH))
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::utils::fmt::fmt_time_utc;

    #[test]
    fn should_parse_lstime() {
        // Good cases
        assert_eq!(
            fmt_time_utc(
                parse_lstime("Nov 5 16:32", "%b %d %Y", "%b %d %H:%M")
                    .ok()
                    .unwrap(),
                "%m %d %M"
            )
            .as_str(),
            "11 05 32"
        );
        assert_eq!(
            fmt_time_utc(
                parse_lstime("Dec 2 21:32", "%b %d %Y", "%b %d %H:%M")
                    .ok()
                    .unwrap(),
                "%m %d %M"
            )
            .as_str(),
            "12 02 32"
        );
        assert_eq!(
            parse_lstime("Nov 5 2018", "%b %d %Y", "%b %d %H:%M")
                .ok()
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1541376000)
        );
        assert_eq!(
            parse_lstime("Mar 18 2018", "%b %d %Y", "%b %d %H:%M")
                .ok()
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1521331200)
        );
        // bad cases
        assert!(parse_lstime("Oma 31 2018", "%b %d %Y", "%b %d %H:%M").is_err());
        assert!(parse_lstime("Feb 31 2018", "%b %d %Y", "%b %d %H:%M").is_err());
        assert!(parse_lstime("Feb 15 25:32", "%b %d %Y", "%b %d %H:%M").is_err());
    }
}
