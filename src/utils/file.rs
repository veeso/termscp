//! ## File
//!
//! `file` is the module which exposes file related utilities

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
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;

/// ### open_file
///
/// Open file provided as parameter
pub fn open_file<P>(filename: P, create: bool, write: bool, append: bool) -> io::Result<File>
where
    P: AsRef<Path>,
{
    OpenOptions::new()
        .create(create)
        .write(write)
        .append(append)
        .truncate(!append)
        .open(filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utils_file_open() {
        let tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        assert!(open_file(tmpfile.path(), true, true, true).is_ok());
    }
}
