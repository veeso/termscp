//! ## Formatter
//!
//! `formatter` is the module which provides formatting utilities for `FileExplorer`

// Locals
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

// Ext
use bytesize::ByteSize;
use lazy_regex::{Lazy, Regex};
use remotefs::File;
use unicode_width::UnicodeWidthStr;
#[cfg(posix)]
use uzers::{get_group_by_gid, get_user_by_uid};

use crate::utils::fmt::{fmt_path_elide, fmt_pex, fmt_time};
use crate::utils::path::diff_paths;
use crate::utils::string::secure_substring;
// Types
// FmtCallback: Formatter, fsentry: &File, cur_str, prefix, length, extra
type FmtCallback = fn(&Formatter, &File, &str, &str, Option<&usize>, Option<&String>) -> String;

// Keys
const FMT_KEY_ATIME: &str = "ATIME";
const FMT_KEY_CTIME: &str = "CTIME";
const FMT_KEY_GROUP: &str = "GROUP";
const FMT_KEY_MTIME: &str = "MTIME";
const FMT_KEY_NAME: &str = "NAME";
const FMT_KEY_PATH: &str = "PATH";
const FMT_KEY_PEX: &str = "PEX";
const FMT_KEY_SIZE: &str = "SIZE";
const FMT_KEY_SYMLINK: &str = "SYMLINK";
const FMT_KEY_USER: &str = "USER";
// Default
const FMT_DEFAULT_STX: &str = "{NAME} {PEX} {USER} {SIZE} {MTIME}";
/**
 * Regex matches:
 *  - group 0: KEY NAME
 *  - group 1?: LENGTH
 *  - group 2?: EXTRA
 */
static FMT_KEY_REGEX: Lazy<Regex> = lazy_regex!(r"\{(.*?)\}");
static FMT_ATTR_REGEX: Lazy<Regex> = lazy_regex!(r"(?:([A-Z]+))(:?([0-9]+))?(:?(.+))?");

/// Call Chain block is a block in a chain of functions which are called in order to format the File.
/// A callChain is instantiated starting from the Formatter syntax and the regex, once the groups are found
/// a chain of function is made using the Formatters method.
/// This method provides an extremely fast way to format fs entries
struct CallChainBlock {
    /// The function to call to format current item
    func: FmtCallback,
    /// All the content which is between two `{KEY}` items
    prefix: String,
    /// The fmt len, specied for key as `{KEY:LEN}`
    fmt_len: Option<usize>,
    /// The extra argument for formatting, specified for key as `{KEY:LEN:EXTRA}`
    fmt_extra: Option<String>,
    /// The next block to format
    next_block: Option<Box<CallChainBlock>>,
}

impl CallChainBlock {
    /// Create a new `CallChainBlock`
    pub fn new(
        func: FmtCallback,
        prefix: String,
        fmt_len: Option<usize>,
        fmt_extra: Option<String>,
    ) -> Self {
        CallChainBlock {
            func,
            prefix,
            fmt_len,
            fmt_extra,
            next_block: None,
        }
    }

    /// Call next callback in the CallChain
    pub fn next(&self, fmt: &Formatter, fsentry: &File, cur_str: &str) -> String {
        // Call func
        let new_str: String = (self.func)(
            fmt,
            fsentry,
            cur_str,
            self.prefix.as_str(),
            self.fmt_len.as_ref(),
            self.fmt_extra.as_ref(),
        );
        // If next is some, call next, otherwise (END OF CHAIN) return new_str
        match &self.next_block {
            Some(block) => block.next(fmt, fsentry, new_str.as_str()),
            None => new_str,
        }
    }

    /// Push func to the last element in the Call chain
    pub fn push(
        &mut self,
        func: FmtCallback,
        prefix: String,
        fmt_len: Option<usize>,
        fmt_extra: Option<String>,
    ) {
        // Call recursively until an element with next_block equal to None is found
        match &mut self.next_block {
            None => {
                self.next_block = Some(Box::new(CallChainBlock::new(
                    func, prefix, fmt_len, fmt_extra,
                )))
            }
            Some(block) => block.push(func, prefix, fmt_len, fmt_extra),
        }
    }
}

/// Formatter takes care of formatting FsEntries according to the provided keys.
/// Formatting is performed using the `CallChainBlock`, which composed makes a Call Chain. This method is extremely fast compared to match the format groups
/// at each fmt call.
pub struct Formatter {
    call_chain: CallChainBlock,
}

impl Default for Formatter {
    /// Instantiates a Formatter with the default fmt syntax
    fn default() -> Self {
        Formatter {
            call_chain: Self::make_callchain(FMT_DEFAULT_STX),
        }
    }
}

impl Formatter {
    /// Instantiates a new `Formatter` with the provided format string
    pub fn new(fmt_str: &str) -> Self {
        Formatter {
            call_chain: Self::make_callchain(fmt_str),
        }
    }

    /// Format fsentry
    pub fn fmt(&self, fsentry: &File) -> String {
        // Execute callchain blocks
        self.call_chain.next(self, fsentry, "")
    }

    // Fmt methods

    /// Format last access time
    fn fmt_atime(
        &self,
        fsentry: &File,
        cur_str: &str,
        prefix: &str,
        fmt_len: Option<&usize>,
        fmt_extra: Option<&String>,
    ) -> String {
        // Get date (use extra args as format or default "%b %d %Y %H:%M")
        let datetime: String = fmt_time(
            fsentry.metadata().accessed.unwrap_or(UNIX_EPOCH),
            match fmt_extra {
                Some(fmt) => fmt.as_ref(),
                None => "%b %d %Y %H:%M",
            },
        );
        // Add to cur str, prefix and the key value
        format!(
            "{}{}{:0width$}",
            cur_str,
            prefix,
            datetime,
            width = fmt_len.unwrap_or(&17)
        )
    }

    /// Format creation time
    fn fmt_ctime(
        &self,
        fsentry: &File,
        cur_str: &str,
        prefix: &str,
        fmt_len: Option<&usize>,
        fmt_extra: Option<&String>,
    ) -> String {
        // Get date
        let datetime: String = fmt_time(
            fsentry.metadata().created.unwrap_or(UNIX_EPOCH),
            match fmt_extra {
                Some(fmt) => fmt.as_ref(),
                None => "%b %d %Y %H:%M",
            },
        );
        // Add to cur str, prefix and the key value
        format!(
            "{}{}{:0width$}",
            cur_str,
            prefix,
            datetime,
            width = fmt_len.unwrap_or(&17)
        )
    }

    /// Format owner group
    fn fmt_group(
        &self,
        fsentry: &File,
        cur_str: &str,
        prefix: &str,
        fmt_len: Option<&usize>,
        _fmt_extra: Option<&String>,
    ) -> String {
        // Get username
        #[cfg(posix)]
        let group: String = match fsentry.metadata().gid {
            Some(gid) => match get_group_by_gid(gid) {
                Some(user) => user.name().to_string_lossy().to_string(),
                None => gid.to_string(),
            },
            None => 0.to_string(),
        };
        #[cfg(windows)]
        let group: String = match fsentry.metadata().gid {
            Some(gid) => gid.to_string(),
            None => 0.to_string(),
        };
        // Add to cur str, prefix and the key value
        format!(
            "{}{}{:0width$}",
            cur_str,
            prefix,
            group,
            width = fmt_len.unwrap_or(&12)
        )
    }

    /// Format last change time
    fn fmt_mtime(
        &self,
        fsentry: &File,
        cur_str: &str,
        prefix: &str,
        fmt_len: Option<&usize>,
        fmt_extra: Option<&String>,
    ) -> String {
        // Get date
        let datetime: String = fmt_time(
            fsentry.metadata().modified.unwrap_or(UNIX_EPOCH),
            match fmt_extra {
                Some(fmt) => fmt.as_ref(),
                None => "%b %d %Y %H:%M",
            },
        );
        // Add to cur str, prefix and the key value
        format!(
            "{}{}{:0width$}",
            cur_str,
            prefix,
            datetime,
            width = fmt_len.unwrap_or(&17)
        )
    }

    /// Format file name
    fn fmt_name(
        &self,
        fsentry: &File,
        cur_str: &str,
        prefix: &str,
        fmt_len: Option<&usize>,
        _fmt_extra: Option<&String>,
    ) -> String {
        // Get file name (or elide if too long)
        let file_len: usize = match fmt_len {
            Some(l) => *l,
            None => 24,
        };
        let name = fsentry.name();
        let last_idx: usize = match fsentry.is_dir() {
            // NOTE: For directories is l - 2, since we push '/' to name
            true => file_len - 2,
            false => file_len - 1,
        };
        let mut name: String = match name.width() >= file_len {
            false => name,
            true => format!("{}…", secure_substring(&name, 0, last_idx)),
        };
        if fsentry.is_dir() {
            name.push('/');
        }
        // Add to cur str, prefix and the key value
        format!("{cur_str}{prefix}{name:0file_len$}")
    }

    /// Format path
    fn fmt_path(
        &self,
        fsentry: &File,
        cur_str: &str,
        prefix: &str,
        fmt_len: Option<&usize>,
        fmt_extra: Option<&String>,
    ) -> String {
        let p = match fmt_extra {
            None => fsentry.path().to_path_buf(),
            Some(rel) => diff_paths(fsentry.path(), PathBuf::from(rel.as_str()).as_path())
                .unwrap_or_else(|| fsentry.path().to_path_buf()),
        };
        format!(
            "{}{}{}",
            cur_str,
            prefix,
            match fmt_len {
                None => p.display().to_string(),
                Some(len) => fmt_path_elide(p.as_path(), *len),
            }
        )
    }

    /// Format file permissions
    fn fmt_pex(
        &self,
        fsentry: &File,
        cur_str: &str,
        prefix: &str,
        _fmt_len: Option<&usize>,
        _fmt_extra: Option<&String>,
    ) -> String {
        // Create mode string
        let mut pex: String = String::with_capacity(10);
        let file_type: char = match fsentry.metadata().symlink.is_some() {
            true => 'l',
            false => match fsentry.is_dir() {
                true => 'd',
                false => '-',
            },
        };
        pex.push(file_type);
        match fsentry.metadata().mode {
            None => pex.push_str("?????????"),
            Some(mode) => pex.push_str(
                format!(
                    "{}{}{}",
                    fmt_pex(mode.user()),
                    fmt_pex(mode.group()),
                    fmt_pex(mode.others())
                )
                .as_str(),
            ),
        }
        // Add to cur str, prefix and the key value
        format!("{cur_str}{prefix}{pex:10}")
    }

    /// Format file size
    fn fmt_size(
        &self,
        fsentry: &File,
        cur_str: &str,
        prefix: &str,
        _fmt_len: Option<&usize>,
        _fmt_extra: Option<&String>,
    ) -> String {
        if fsentry.is_file() {
            // Get byte size
            let size: ByteSize = ByteSize(fsentry.metadata().size);
            // Add to cur str, prefix and the key value
            format!("{cur_str}{prefix}{size:10}")
        } else if fsentry.metadata().symlink.is_some() {
            let size = ByteSize(
                fsentry
                    .metadata()
                    .symlink
                    .as_ref()
                    .unwrap()
                    .to_string_lossy()
                    .len() as u64,
            );
            format!("{cur_str}{prefix}{size:10}")
        } else {
            // Add to cur str, prefix and the key value
            format!("{cur_str}{prefix}          ")
        }
    }

    /// Format file symlink (if any)
    fn fmt_symlink(
        &self,
        fsentry: &File,
        cur_str: &str,
        prefix: &str,
        fmt_len: Option<&usize>,
        _fmt_extra: Option<&String>,
    ) -> String {
        // Get file name (or elide if too long)
        let file_len: usize = match fmt_len {
            Some(l) => *l,
            None => 21,
        };
        // Replace `FMT_KEY_NAME` with name
        match fsentry.metadata().symlink.as_deref() {
            None => format!("{cur_str}{prefix}                        "),
            Some(p) => format!(
                "{}{}-> {:0width$}",
                cur_str,
                prefix,
                fmt_path_elide(p, file_len - 1),
                width = file_len
            ),
        }
    }

    /// Format owner user
    fn fmt_user(
        &self,
        fsentry: &File,
        cur_str: &str,
        prefix: &str,
        _fmt_len: Option<&usize>,
        _fmt_extra: Option<&String>,
    ) -> String {
        // Get username
        #[cfg(posix)]
        let username: String = match fsentry.metadata().uid {
            Some(uid) => match get_user_by_uid(uid) {
                Some(user) => user.name().to_string_lossy().to_string(),
                None => uid.to_string(),
            },
            None => 0.to_string(),
        };
        #[cfg(windows)]
        let username: String = match fsentry.metadata().uid {
            Some(uid) => uid.to_string(),
            None => 0.to_string(),
        };
        // Add to cur str, prefix and the key value
        format!("{cur_str}{prefix}{username:12}")
    }

    /// Fallback function in case the format key is unknown
    /// It does nothing, just returns cur_str
    fn fmt_fallback(
        &self,
        _fsentry: &File,
        cur_str: &str,
        prefix: &str,
        _fmt_len: Option<&usize>,
        _fmt_extra: Option<&String>,
    ) -> String {
        // Add to cur str and prefix
        format!("{cur_str}{prefix}")
    }

    // Static

    /// Make a callchain starting from the fmt str
    fn make_callchain(fmt_str: &str) -> CallChainBlock {
        // Init chain block
        let mut callchain: Option<CallChainBlock> = None;
        // Track index of the last match found, to get the prefix for each token
        let mut last_index: usize = 0;
        // Match fmt str against regex
        for regex_match in FMT_KEY_REGEX.captures_iter(fmt_str) {
            // Get match index (unwrap is safe, since always exists)
            let index: usize = fmt_str.find(&regex_match[0]).unwrap();
            // Get prefix
            let prefix: String = String::from(&fmt_str[last_index..index]);
            // Increment last index (sum prefix lenght and the length of the key)
            last_index += prefix.len() + regex_match[0].len();
            // Match attributes
            match FMT_ATTR_REGEX.captures(&regex_match[1]) {
                Some(regex_match) => {
                    // Match group 0 (which is name)
                    let callback: FmtCallback = match &regex_match.get(1) {
                        Some(key) => match key.as_str() {
                            FMT_KEY_ATIME => Self::fmt_atime,
                            FMT_KEY_CTIME => Self::fmt_ctime,
                            FMT_KEY_GROUP => Self::fmt_group,
                            FMT_KEY_MTIME => Self::fmt_mtime,
                            FMT_KEY_NAME => Self::fmt_name,
                            FMT_KEY_PATH => Self::fmt_path,
                            FMT_KEY_PEX => Self::fmt_pex,
                            FMT_KEY_SIZE => Self::fmt_size,
                            FMT_KEY_SYMLINK => Self::fmt_symlink,
                            FMT_KEY_USER => Self::fmt_user,
                            _ => Self::fmt_fallback,
                        },
                        None => Self::fmt_fallback,
                    };
                    // Match format length: group 3
                    let fmt_len: Option<usize> = match &regex_match.get(3) {
                        Some(len) => match len.as_str().parse::<usize>() {
                            Ok(len) => Some(len),
                            Err(_) => None,
                        },
                        None => None,
                    };
                    // Match format extra: group 2 + 1
                    let fmt_extra: Option<String> = regex_match
                        .get(5)
                        .as_ref()
                        .map(|extra| extra.as_str().to_string());
                    // Create a callchain or push new element to its back
                    match callchain.as_mut() {
                        None => {
                            callchain =
                                Some(CallChainBlock::new(callback, prefix, fmt_len, fmt_extra))
                        }
                        Some(chain_block) => chain_block.push(callback, prefix, fmt_len, fmt_extra),
                    }
                }
                None => continue,
            }
        }
        // Finalize and return
        match callchain {
            Some(callchain) => callchain,
            None => CallChainBlock::new(Self::fmt_fallback, String::new(), None, None),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;
    use std::time::SystemTime;

    use pretty_assertions::assert_eq;
    use remotefs::fs::{File, FileType, Metadata, UnixPex};

    use super::*;

    #[test]
    fn test_fs_explorer_formatter_callchain() {
        // Make a dummy formatter
        let dummy_formatter: Formatter = Formatter::new("");
        // Make a dummy entry
        let t: SystemTime = SystemTime::now();
        let dummy_entry = File {
            path: PathBuf::from("/bar.txt"),
            metadata: Metadata {
                accessed: Some(t),
                created: Some(t),
                modified: Some(t),
                file_type: FileType::File,
                size: 8192,
                symlink: None,
                uid: Some(0),
                gid: Some(0),
                mode: Some(UnixPex::from(0o644)),
            },
        };
        let prefix: String = String::from("h");
        let mut callchain: CallChainBlock = CallChainBlock::new(dummy_fmt, prefix, None, None);
        assert!(callchain.next_block.is_none());
        assert_eq!(callchain.prefix, String::from("h"));
        // Execute
        assert_eq!(
            callchain.next(&dummy_formatter, &dummy_entry, ""),
            String::from("hA")
        );
        // Push 4 new blocks
        callchain.push(dummy_fmt, String::from("h"), None, None);
        callchain.push(dummy_fmt, String::from("h"), None, None);
        callchain.push(dummy_fmt, String::from("h"), None, None);
        callchain.push(dummy_fmt, String::from("h"), None, None);
        // Verify
        assert_eq!(
            callchain.next(&dummy_formatter, &dummy_entry, ""),
            String::from("hAhAhAhAhA")
        );
    }

    #[test]
    fn test_fs_explorer_formatter_format_files() {
        // Make default
        let formatter: Formatter = Formatter::default();
        // Experiments :D
        let t: SystemTime = SystemTime::now();
        let entry = File {
            path: PathBuf::from("/bar.txt"),
            metadata: Metadata {
                accessed: Some(t),
                created: Some(t),
                modified: Some(t),
                file_type: FileType::File,
                size: 8192,
                symlink: None,
                uid: Some(0),
                gid: Some(0),
                mode: Some(UnixPex::from(0o644)),
            },
        };
        #[cfg(posix)]
        assert_eq!(
            formatter.fmt(&entry),
            format!(
                "bar.txt                  -rw-r--r-- root         8.2 KB     {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
        #[cfg(windows)]
        assert_eq!(
            formatter.fmt(&entry),
            format!(
                "bar.txt                  -rw-r--r-- 0            8.2 KB     {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
        // Elide name
        let entry = File {
            path: PathBuf::from("/piroparoporoperoperupupu.txt"),
            metadata: Metadata {
                accessed: Some(t),
                created: Some(t),
                modified: Some(t),
                file_type: FileType::File,
                size: 8192,
                symlink: None,
                uid: Some(0),
                gid: Some(0),
                mode: Some(UnixPex::from(0o644)),
            },
        };
        #[cfg(posix)]
        assert_eq!(
            formatter.fmt(&entry),
            format!(
                "piroparoporoperoperupup… -rw-r--r-- root         8.2 KB     {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
        #[cfg(windows)]
        assert_eq!(
            formatter.fmt(&entry),
            format!(
                "piroparoporoperoperupup… -rw-r--r-- 0            8.2 KB     {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
        // No pex
        let entry = File {
            path: PathBuf::from("/bar.txt"),
            metadata: Metadata {
                accessed: Some(t),
                created: Some(t),
                modified: Some(t),
                file_type: FileType::File,
                size: 8192,
                symlink: None,
                uid: Some(0),
                gid: Some(0),
                mode: None,
            },
        };
        #[cfg(posix)]
        assert_eq!(
            formatter.fmt(&entry),
            format!(
                "bar.txt                  -????????? root         8.2 KB     {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
        #[cfg(windows)]
        assert_eq!(
            formatter.fmt(&entry),
            format!(
                "bar.txt                  -????????? 0            8.2 KB     {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
        // No user
        let entry = File {
            path: PathBuf::from("/bar.txt"),
            metadata: Metadata {
                accessed: Some(t),
                created: Some(t),
                modified: Some(t),
                file_type: FileType::File,
                size: 8192,
                symlink: None,
                uid: None,
                gid: Some(0),
                mode: None,
            },
        };
        #[cfg(posix)]
        assert_eq!(
            formatter.fmt(&entry),
            format!(
                "bar.txt                  -????????? 0            8.2 KB     {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
        #[cfg(windows)]
        assert_eq!(
            formatter.fmt(&entry),
            format!(
                "bar.txt                  -????????? 0            8.2 KB     {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
    }

    #[test]
    fn test_fs_explorer_formatter_format_dirs() {
        // Make default
        let formatter: Formatter = Formatter::default();
        // Experiments :D
        let t: SystemTime = SystemTime::now();
        let entry = File {
            path: PathBuf::from("/home/cvisintin/projects"),
            metadata: Metadata {
                accessed: Some(t),
                created: Some(t),
                modified: Some(t),
                file_type: FileType::Directory,
                size: 4096,
                symlink: None,
                uid: Some(0),
                gid: Some(0),
                mode: Some(UnixPex::from(0o755)),
            },
        };
        #[cfg(posix)]
        assert_eq!(
            formatter.fmt(&entry),
            format!(
                "projects/                drwxr-xr-x root                    {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
        #[cfg(windows)]
        assert_eq!(
            formatter.fmt(&entry),
            format!(
                "projects/                drwxr-xr-x 0                       {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
        // No pex, no user
        let entry = File {
            path: PathBuf::from("/home/cvisintin/projects"),
            metadata: Metadata {
                accessed: Some(t),
                created: Some(t),
                modified: Some(t),
                file_type: FileType::Directory,
                size: 4096,
                symlink: None,
                uid: None,
                gid: Some(0),
                mode: None,
            },
        };
        #[cfg(posix)]
        assert_eq!(
            formatter.fmt(&entry),
            format!(
                "projects/                d????????? 0                       {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
        #[cfg(windows)]
        assert_eq!(
            formatter.fmt(&entry),
            format!(
                "projects/                d????????? 0                       {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
    }

    #[test]
    fn test_fs_explorer_formatter_all_together_now() {
        let formatter: Formatter =
            Formatter::new("{NAME:16} {SYMLINK:12} {GROUP} {USER} {PEX} {SIZE} {ATIME:20:%a %b %d %Y %H:%M} {CTIME:20:%a %b %d %Y %H:%M} {MTIME:20:%a %b %d %Y %H:%M}");
        // Directory (with symlink)
        let t: SystemTime = SystemTime::now();
        let entry = File {
            path: PathBuf::from("/home/cvisintin/projects"),
            metadata: Metadata {
                accessed: Some(t),
                created: Some(t),
                modified: Some(t),
                file_type: FileType::Symlink,
                size: 4096,
                symlink: Some(PathBuf::from("project.info")),
                uid: None,
                gid: None,
                mode: Some(UnixPex::from(0o755)),
            },
        };
        assert_eq!(formatter.fmt(&entry), format!(
            "projects         -> project.info 0            0            lrwxr-xr-x 12 B       {} {} {}",
            fmt_time(t, "%a %b %d %Y %H:%M"), 
            fmt_time(t, "%a %b %d %Y %H:%M"), 
            fmt_time(t, "%a %b %d %Y %H:%M"), 
        ));
        // Directory without symlink
        let entry = File {
            path: PathBuf::from("/home/cvisintin/projects"),
            metadata: Metadata {
                accessed: Some(t),
                created: Some(t),
                modified: Some(t),
                file_type: FileType::Directory,
                size: 4096,
                symlink: None,
                uid: None,
                gid: None,
                mode: Some(UnixPex::from(0o755)),
            },
        };
        assert_eq!(formatter.fmt(&entry), format!(
            "projects/                                 0            0            drwxr-xr-x            {} {} {}",
            fmt_time(t, "%a %b %d %Y %H:%M"), 
            fmt_time(t, "%a %b %d %Y %H:%M"), 
            fmt_time(t, "%a %b %d %Y %H:%M"), 
        ));
        // File with symlink
        let entry = File {
            path: PathBuf::from("/bar.txt"),
            metadata: Metadata {
                accessed: Some(t),
                created: Some(t),
                modified: Some(t),
                file_type: FileType::Symlink,
                size: 8192,
                symlink: Some(PathBuf::from("project.info")),
                uid: None,
                gid: None,
                mode: Some(UnixPex::from(0o644)),
            },
        };
        assert_eq!(formatter.fmt(&entry), format!(
            "bar.txt          -> project.info 0            0            lrw-r--r-- 12 B       {} {} {}",
            fmt_time(t, "%a %b %d %Y %H:%M"), 
            fmt_time(t, "%a %b %d %Y %H:%M"), 
            fmt_time(t, "%a %b %d %Y %H:%M"), 
        ));
        // File without symlink
        let entry = File {
            path: PathBuf::from("/bar.txt"),
            metadata: Metadata {
                accessed: Some(t),
                created: Some(t),
                modified: Some(t),
                file_type: FileType::File,
                size: 8192,
                symlink: None,
                uid: None,
                gid: None,
                mode: Some(UnixPex::from(0o644)),
            },
        };
        assert_eq!(formatter.fmt(&entry), format!(
            "bar.txt                                   0            0            -rw-r--r-- 8.2 KB     {} {} {}",
            fmt_time(t, "%a %b %d %Y %H:%M"), 
            fmt_time(t, "%a %b %d %Y %H:%M"), 
            fmt_time(t, "%a %b %d %Y %H:%M"), 
        ));
    }

    #[test]
    #[cfg(posix)]
    fn should_fmt_path() {
        let t: SystemTime = SystemTime::now();
        let entry = File {
            path: PathBuf::from("/tmp/a/b/c/bar.txt"),
            metadata: Metadata {
                accessed: Some(t),
                created: Some(t),
                modified: Some(t),
                file_type: FileType::Symlink,
                size: 8192,
                symlink: Some(PathBuf::from("project.info")),
                uid: None,
                gid: None,
                mode: Some(UnixPex::from(0o644)),
            },
        };
        let formatter: Formatter = Formatter::new("File path: {PATH}");
        assert_eq!(
            formatter.fmt(&entry).as_str(),
            "File path: /tmp/a/b/c/bar.txt"
        );
        let formatter: Formatter = Formatter::new("File path: {PATH:8}");
        assert_eq!(
            formatter.fmt(&entry).as_str(),
            "File path: /tmp/…/c/bar.txt"
        );
        let formatter: Formatter = Formatter::new("File path: {PATH:128:/tmp/a/b}");
        assert_eq!(formatter.fmt(&entry).as_str(), "File path: c/bar.txt");
    }

    #[test]
    #[cfg(posix)]
    fn should_fmt_utf8_path() {
        let t: SystemTime = SystemTime::now();
        let entry = File {
            path: PathBuf::from("/tmp/a/b/c/россия"),
            metadata: Metadata {
                accessed: Some(t),
                created: Some(t),
                modified: Some(t),
                file_type: FileType::Symlink,
                size: 8192,
                symlink: Some(PathBuf::from("project.info")),
                uid: None,
                gid: None,
                mode: Some(UnixPex::from(0o644)),
            },
        };
        let formatter: Formatter = Formatter::new("File path: {PATH}");
        assert_eq!(
            formatter.fmt(&entry).as_str(),
            "File path: /tmp/a/b/c/россия"
        );
        let formatter: Formatter = Formatter::new("File path: {PATH:8}");
        assert_eq!(formatter.fmt(&entry).as_str(), "File path: /tmp/…/c/россия");
    }

    #[test]
    fn should_fmt_short_ascii_name() {
        let entry = File {
            path: PathBuf::from("/tmp/foo.txt"),
            metadata: Metadata {
                accessed: None,
                created: None,
                modified: None,
                file_type: FileType::File,
                size: 8192,
                symlink: None,
                uid: None,
                gid: None,
                mode: None,
            },
        };
        let formatter: Formatter = Formatter::new("{NAME:8}");
        assert_eq!(formatter.fmt(&entry).as_str(), "foo.txt ");
    }

    #[test]
    fn should_fmt_exceeding_length_ascii_name() {
        let entry = File {
            path: PathBuf::from("/tmp/christian-visintin.txt"),
            metadata: Metadata {
                accessed: None,
                created: None,
                modified: None,
                file_type: FileType::File,
                size: 8192,
                symlink: None,
                uid: None,
                gid: None,
                mode: None,
            },
        };
        let formatter: Formatter = Formatter::new("{NAME:8}");
        assert_eq!(formatter.fmt(&entry).as_str(), "christi…");
    }

    #[test]
    fn should_fmt_short_utf8_name() {
        let entry = File {
            path: PathBuf::from("/tmp/россия"),
            metadata: Metadata {
                accessed: None,
                created: None,
                modified: None,
                file_type: FileType::File,
                size: 8192,
                symlink: None,
                uid: None,
                gid: None,
                mode: None,
            },
        };
        let formatter: Formatter = Formatter::new("{NAME:8}");
        assert_eq!(formatter.fmt(&entry).as_str(), "россия  ");
    }

    #[test]
    fn should_fmt_long_utf8_name() {
        let entry = File {
            path: PathBuf::from("/tmp/喵喵喵喵喵喵喵喵喵喵喵喵喵喵喵喵喵喵喵喵喵喵"),
            metadata: Metadata {
                accessed: None,
                created: None,
                modified: None,
                file_type: FileType::File,
                size: 8192,
                symlink: None,
                uid: None,
                gid: None,
                mode: None,
            },
        };
        let formatter: Formatter = Formatter::new("{NAME:8}");
        assert_eq!(formatter.fmt(&entry).as_str(), "喵喵喵喵喵喵喵…");
    }

    /// Dummy formatter, just yelds an 'A' at the end of the current string
    fn dummy_fmt(
        _fmt: &Formatter,
        _entry: &File,
        cur_str: &str,
        prefix: &str,
        _fmt_len: Option<&usize>,
        _fmt_extra: Option<&String>,
    ) -> String {
        format!("{cur_str}{prefix}A")
    }
}
