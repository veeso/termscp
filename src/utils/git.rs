//! ## git
//!
//! `git` is the module which provides utilities to interact through the GIT API and to perform some stuff at git level

/*
*
*   Copyright (C) 2020-2021 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
*/

// Deps
extern crate ureq;
// Locals
use super::parser::parse_semver;
// Others
use serde::Deserialize;

#[derive(Deserialize)]
struct TagInfo {
    tag_name: String,
}

/// ### check_for_updates
///
/// Check if there is a new version available for termscp.
/// This is performed through the Github API
/// In case of success returns Ok(Option<String>), where the Option is Some(new_version); otherwise if no version is available, return None
/// In case of error returns Error with the error description

pub fn check_for_updates(current_version: &str) -> Result<Option<String>, String> {
    // Send request
    let github_version: Result<String, String> =
        match ureq::get("https://api.github.com/repos/veeso/termscp/releases/latest").call() {
            Ok(response) => match response.into_json::<TagInfo>() {
                Ok(tag_info) => Ok(tag_info.tag_name),
                Err(err) => Err(err.to_string()),
            },
            Err(err) => Err(err.to_string()),
        };
    // Check version
    match github_version {
        Err(err) => Err(err),
        Ok(version) => {
            // Parse version
            match parse_semver(version.as_str()) {
                Some(new_version) => {
                    // Check if version is different
                    if new_version.as_str() > current_version {
                        Ok(Some(new_version)) // New version is available
                    } else {
                        Ok(None) // No new version
                    }
                }
                None => Err(String::from("Got bad response from Github")),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[cfg(not(all(target_os = "macos", feature = "githubActions")))]
    fn test_utils_git_check_for_updates() {
        assert!(check_for_updates("100.0.0").ok().unwrap().is_none());
        assert!(check_for_updates("0.0.1").ok().unwrap().is_some());
    }
}
