//! ## git
//!
//! `git` is the module which provides utilities to interact through the GIT API and to perform some stuff at git level

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// Locals
use super::parser::parse_semver;
// Others
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// ## GithubTag
///
/// Info related to a github tag
pub struct GithubTag {
    pub tag_name: String,
    pub body: String,
}

/// ### check_for_updates
///
/// Check if there is a new version available for termscp.
/// This is performed through the Github API
/// In case of success returns Ok(Option<GithubTag>), where the Option is Some(new_version); otherwise if no version is available, return None
/// In case of error returns Error with the error description

pub fn check_for_updates(current_version: &str) -> Result<Option<GithubTag>, String> {
    // Send request
    let github_tag: Result<GithubTag, String> =
        match ureq::get("https://api.github.com/repos/veeso/termscp/releases/latest").call() {
            Ok(response) => response.into_json::<GithubTag>().map_err(|x| x.to_string()),
            Err(err) => Err(err.to_string()),
        };
    // Check version
    match github_tag {
        Err(err) => Err(err),
        Ok(tag) => {
            // Parse version
            match parse_semver(tag.tag_name.as_str()) {
                Some(new_version) => {
                    // Check if version is different
                    if new_version.as_str() > current_version {
                        Ok(Some(tag)) // New version is available
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
    #[cfg(not(all(
        any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "netbsd"
        ),
        feature = "github-actions"
    )))]
    fn test_utils_git_check_for_updates() {
        assert!(check_for_updates("100.0.0").ok().unwrap().is_none());
        assert!(check_for_updates("0.0.1").ok().unwrap().is_some());
    }
}
