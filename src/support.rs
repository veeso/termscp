//! ## Support
//!
//! this module exposes some extra run modes for termscp, meant to be used for "support", such as installing themes

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
// mod
use crate::system::{environment, theme_provider::ThemeProvider};
use std::fs;
use std::path::{Path, PathBuf};

/// ### import_theme
///
/// Import theme at provided path into termscp
pub fn import_theme(p: &Path) -> Result<(), String> {
    if !p.exists() {
        return Err(String::from(
            "Could not import theme: No such file or directory",
        ));
    }
    // Validate theme file
    ThemeProvider::new(p).map_err(|e| format!("Invalid theme error: {}", e))?;
    // get config dir
    let cfg_dir: PathBuf = get_config_dir()?;
    // Get theme directory
    let theme_file: PathBuf = environment::get_theme_path(cfg_dir.as_path());
    // Copy theme to theme_dir
    fs::copy(p, theme_file.as_path())
        .map(|_| ())
        .map_err(|e| format!("Could not import theme: {}", e))
}

/// ### get_config_dir
///
/// Get configuration directory
fn get_config_dir() -> Result<PathBuf, String> {
    match environment::init_config_dir() {
        Ok(Some(config_dir)) => Ok(config_dir),
        Ok(None) => Err(String::from(
            "Your system doesn't provide a configuration directory",
        )),
        Err(err) => Err(format!(
            "Could not initialize configuration directory: {}",
            err
        )),
    }
}
