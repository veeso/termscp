//! ## Explorer
//!
//! `explorer` is the module which provides an Helper in handling Directory status through

// Mods
pub(crate) mod builder;
mod formatter;
// Locals
use std::cmp::Reverse;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use formatter::Formatter;
// Ext
use remotefs::fs::File;

bitflags! {
    /// ExplorerOpts are bit options which provides different behaviours to `FileExplorer`
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub(crate) struct ExplorerOpts: u32 {
        const SHOW_HIDDEN_FILES = 0b00000001;
    }
}

/// FileSorting defines the criteria for sorting files
#[derive(Copy, Clone, PartialEq, Eq, std::fmt::Debug)]
pub enum FileSorting {
    Name,
    ModifyTime,
    CreationTime,
    Size,
    None,
}

/// GroupDirs defines how directories should be grouped in sorting files
#[derive(PartialEq, Eq, std::fmt::Debug)]
pub enum GroupDirs {
    First,
    Last,
}

/// File explorer states
pub struct FileExplorer {
    pub wrkdir: PathBuf,                      // Current directory
    pub(crate) dirstack: VecDeque<PathBuf>,   // Stack of visited directory (max 16)
    pub(crate) stack_size: usize,             // Directory stack size
    pub(crate) file_sorting: FileSorting,     // File sorting criteria
    pub(crate) group_dirs: Option<GroupDirs>, // If Some, defines how to group directories
    pub(crate) opts: ExplorerOpts,            // Explorer options
    pub(crate) fmt: Formatter,                // File formatter
    files: Vec<File>,                         // Files in directory
}

impl Default for FileExplorer {
    fn default() -> Self {
        FileExplorer {
            wrkdir: PathBuf::from("/"),
            dirstack: VecDeque::with_capacity(16),
            stack_size: 16,
            file_sorting: FileSorting::Name,
            group_dirs: None,
            opts: ExplorerOpts::empty(),
            fmt: Formatter::default(),
            files: Vec::new(),
        }
    }
}

impl FileExplorer {
    /// push directory to stack
    pub fn pushd(&mut self, dir: &Path) {
        // Check if stack would overflow the size
        while self.dirstack.len() >= self.stack_size {
            self.dirstack.pop_front(); // Start cleaning events from back
        }
        // Eventually push front the new record
        self.dirstack.push_back(PathBuf::from(dir));
    }

    /// Pop directory from the stack and return the directory
    pub fn popd(&mut self) -> Option<PathBuf> {
        self.dirstack.pop_back()
    }

    /// Set Explorer files
    /// This method will also sort entries based on current options
    /// Once all sorting have been performed, index is moved to first valid entry.
    pub fn set_files(&mut self, files: Vec<File>) {
        self.files = files;
        // Sort
        self.sort();
    }

    /// Delete file at provided index
    pub fn del_entry(&mut self, idx: usize) {
        if self.files.len() > idx {
            self.files.remove(idx);
        }
    }

    /// Iterate over files
    /// Filters are applied based on current options (e.g. hidden files not returned)
    pub fn iter_files(&self) -> impl Iterator<Item = &File> + '_ {
        // Filter
        let opts: ExplorerOpts = self.opts;
        Box::new(self.files.iter().filter(move |x| {
            // If true, element IS NOT filtered
            let mut pass: bool = true;
            // If hidden files SHOULDN'T be shown, AND pass with not hidden
            if !opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES) {
                pass &= !x.is_hidden();
            }
            pass
        }))
    }

    /// Iterate all files; doesn't care about options
    pub fn iter_files_all(&self) -> impl Iterator<Item = &File> + '_ {
        Box::new(self.files.iter())
    }

    /// Get file at relative index
    pub fn get(&self, idx: usize) -> Option<&File> {
        let opts: ExplorerOpts = self.opts;
        let filtered = self
            .files
            .iter()
            .filter(move |x| {
                // If true, element IS NOT filtered
                let mut pass: bool = true;
                // If hidden files SHOULDN'T be shown, AND pass with not hidden
                if !opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES) {
                    pass &= !x.is_hidden();
                }
                pass
            })
            .collect::<Vec<_>>();
        filtered.get(idx).copied()
    }

    // Formatting

    /// Format a file entry
    pub fn fmt_file(&self, entry: &File) -> String {
        self.fmt.fmt(entry)
    }

    // Sorting

    /// Choose sorting method; then sort files
    pub fn sort_by(&mut self, sorting: FileSorting) {
        // If method HAS ACTUALLY CHANGED, sort (performance!)
        if self.file_sorting != sorting {
            self.file_sorting = sorting;
            self.sort();
        }
    }

    /// Get current file sorting method
    pub fn get_file_sorting(&self) -> FileSorting {
        self.file_sorting
    }

    /// Choose group dirs method; then sort files
    pub fn group_dirs_by(&mut self, group_dirs: Option<GroupDirs>) {
        // If method HAS ACTUALLY CHANGED, sort (performance!)
        if self.group_dirs != group_dirs {
            self.group_dirs = group_dirs;
            self.sort();
        }
    }

    /// Sort files based on Explorer options.
    fn sort(&mut self) {
        // Choose sorting method
        match &self.file_sorting {
            FileSorting::Name => self.sort_files_by_name(),
            FileSorting::CreationTime => self.sort_files_by_creation_time(),
            FileSorting::ModifyTime => self.sort_files_by_mtime(),
            FileSorting::Size => self.sort_files_by_size(),
            FileSorting::None => {}
        }
        // Directories first (NOTE: MUST COME AFTER OTHER SORTING)
        // Group directories if necessary
        if let Some(group_dirs) = &self.group_dirs {
            match group_dirs {
                GroupDirs::First => self.sort_files_directories_first(),
                GroupDirs::Last => self.sort_files_directories_last(),
            }
        }
    }

    /// Sort explorer files by their name. All names are converted to lowercase
    fn sort_files_by_name(&mut self) {
        self.files.sort_by_key(|x: &File| x.name().to_lowercase());
    }

    /// Sort files by mtime; the newest comes first
    fn sort_files_by_mtime(&mut self) {
        self.files
            .sort_by_key(|b: &File| Reverse(b.metadata().modified));
    }

    /// Sort files by creation time; the newest comes first
    fn sort_files_by_creation_time(&mut self) {
        self.files
            .sort_by_key(|b: &File| Reverse(b.metadata().created));
    }

    /// Sort files by size
    fn sort_files_by_size(&mut self) {
        self.files
            .sort_by_key(|b: &File| Reverse(b.metadata().size));
    }

    /// Sort files; directories come first
    fn sort_files_directories_first(&mut self) {
        self.files.sort_by_key(|x: &File| !x.is_dir());
    }

    /// Sort files; directories come last
    fn sort_files_directories_last(&mut self) {
        self.files.sort_by_key(|x: &File| x.is_dir());
    }

    /// Enable/disable hidden files
    pub fn toggle_hidden_files(&mut self) {
        self.opts.toggle(ExplorerOpts::SHOW_HIDDEN_FILES);
    }

    /// Returns whether hidden files are visible
    pub fn hidden_files_visible(&self) -> bool {
        self.opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES)
    }
}

// Traits

impl std::fmt::Display for FileSorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileSorting::CreationTime => "by_creation_time",
                FileSorting::ModifyTime => "by_mtime",
                FileSorting::Name => "by_name",
                FileSorting::Size => "by_size",
                FileSorting::None => "none",
            }
        )
    }
}

impl FromStr for FileSorting {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "by_creation_time" => Ok(FileSorting::CreationTime),
            "by_mtime" => Ok(FileSorting::ModifyTime),
            "by_name" => Ok(FileSorting::Name),
            "by_size" => Ok(FileSorting::Size),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for GroupDirs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GroupDirs::First => "first",
                GroupDirs::Last => "last",
            }
        )
    }
}

impl FromStr for GroupDirs {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "first" => Ok(GroupDirs::First),
            "last" => Ok(GroupDirs::Last),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::thread::sleep;
    use std::time::{Duration, SystemTime};

    use pretty_assertions::assert_eq;
    use remotefs::fs::{File, FileType, Metadata, UnixPex};

    use super::*;
    use crate::utils::fmt::fmt_time;

    #[test]
    fn test_fs_explorer_new() {
        let explorer: FileExplorer = FileExplorer::default();
        // Verify
        assert_eq!(explorer.dirstack.len(), 0);
        assert_eq!(explorer.files.len(), 0);
        assert_eq!(explorer.opts, ExplorerOpts::empty());
        assert_eq!(explorer.wrkdir, PathBuf::from("/"));
        assert_eq!(explorer.stack_size, 16);
        assert_eq!(explorer.group_dirs, None);
        assert_eq!(explorer.file_sorting, FileSorting::Name);
        assert_eq!(explorer.get_file_sorting(), FileSorting::Name);
    }

    #[test]
    fn test_fs_explorer_stack() {
        let mut explorer = FileExplorer {
            stack_size: 2,
            dirstack: VecDeque::with_capacity(2),
            ..Default::default()
        };
        explorer.dirstack = VecDeque::with_capacity(2);
        // Push dir
        explorer.pushd(Path::new("/tmp"));
        explorer.pushd(Path::new("/home/omar"));
        // Pop
        assert_eq!(explorer.popd().unwrap(), PathBuf::from("/home/omar"));
        assert_eq!(explorer.dirstack.len(), 1);
        assert_eq!(explorer.popd().unwrap(), PathBuf::from("/tmp"));
        assert_eq!(explorer.dirstack.len(), 0);
        // Dirstack is empty now
        assert!(explorer.popd().is_none());
        // Exceed limit
        explorer.pushd(Path::new("/tmp"));
        explorer.pushd(Path::new("/home/omar"));
        explorer.pushd(Path::new("/dev"));
        assert_eq!(explorer.dirstack.len(), 2);
        assert_eq!(*explorer.dirstack.get(1).unwrap(), PathBuf::from("/dev"));
        assert_eq!(
            *explorer.dirstack.get(0).unwrap(),
            PathBuf::from("/home/omar")
        );
    }

    #[test]
    fn test_fs_explorer_files() {
        let mut explorer: FileExplorer = FileExplorer::default();
        // Don't show hidden files
        explorer.opts.remove(ExplorerOpts::SHOW_HIDDEN_FILES);
        assert_eq!(explorer.hidden_files_visible(), false);
        // Create files
        explorer.set_files(vec![
            make_fs_entry("README.md", false),
            make_fs_entry("src", true),
            make_fs_entry(".git", true),
            make_fs_entry("CONTRIBUTING.md", false),
            make_fs_entry("codecov.yml", false),
            make_fs_entry(".gitignore", false),
        ]);
        assert!(explorer.get(0).is_some());
        assert!(explorer.get(100).is_none());
        //assert_eq!(explorer.count(), 6);
        // Verify (files are sorted by name)
        assert_eq!(explorer.files.get(0).unwrap().name(), ".git");
        // Iter files (all)
        assert_eq!(explorer.iter_files_all().count(), 6);
        // Iter files (hidden excluded) (.git, .gitignore are hidden)
        assert_eq!(explorer.iter_files().count(), 4);
        // Toggle hidden
        explorer.toggle_hidden_files();
        assert_eq!(explorer.hidden_files_visible(), true);
        assert_eq!(explorer.iter_files().count(), 6); // All files are returned now
    }

    #[test]
    fn test_fs_explorer_sort_by_name() {
        let mut explorer: FileExplorer = FileExplorer::default();
        // Create files (files are then sorted by name)
        explorer.set_files(vec![
            make_fs_entry("README.md", false),
            make_fs_entry("src", true),
            make_fs_entry("CONTRIBUTING.md", false),
            make_fs_entry("CODE_OF_CONDUCT.md", false),
            make_fs_entry("CHANGELOG.md", false),
            make_fs_entry("LICENSE", false),
            make_fs_entry("Cargo.toml", false),
            make_fs_entry("Cargo.lock", false),
            make_fs_entry("codecov.yml", false),
        ]);
        explorer.sort_by(FileSorting::Name);
        // First entry should be "Cargo.lock"
        assert_eq!(explorer.files.get(0).unwrap().name(), "Cargo.lock");
        // Last should be "src"
        assert_eq!(explorer.files.get(8).unwrap().name(), "src");
    }

    #[test]
    fn test_fs_explorer_sort_by_mtime() {
        let mut explorer: FileExplorer = FileExplorer::default();
        let entry1: File = make_fs_entry("README.md", false);
        // Wait 1 sec
        sleep(Duration::from_secs(1));
        let entry2: File = make_fs_entry("CODE_OF_CONDUCT.md", false);
        // Create files (files are then sorted by name)
        explorer.set_files(vec![entry1, entry2]);
        explorer.sort_by(FileSorting::ModifyTime);
        // First entry should be "CODE_OF_CONDUCT.md"
        assert_eq!(explorer.files.get(0).unwrap().name(), "CODE_OF_CONDUCT.md");
        // Last should be "src"
        assert_eq!(explorer.files.get(1).unwrap().name(), "README.md");
    }

    #[test]
    fn test_fs_explorer_sort_by_creation_time() {
        let mut explorer: FileExplorer = FileExplorer::default();
        let entry1: File = make_fs_entry("README.md", false);
        // Wait 1 sec
        sleep(Duration::from_secs(1));
        let entry2: File = make_fs_entry("CODE_OF_CONDUCT.md", false);
        // Create files (files are then sorted by name)
        explorer.set_files(vec![entry1, entry2]);
        explorer.sort_by(FileSorting::CreationTime);
        // First entry should be "CODE_OF_CONDUCT.md"
        assert_eq!(explorer.files.get(0).unwrap().name(), "CODE_OF_CONDUCT.md");
        // Last should be "src"
        assert_eq!(explorer.files.get(1).unwrap().name(), "README.md");
    }

    #[test]
    fn test_fs_explorer_sort_by_size() {
        let mut explorer: FileExplorer = FileExplorer::default();
        // Create files (files are then sorted by name)
        explorer.set_files(vec![
            make_fs_entry_with_size("README.md", false, 1024),
            make_fs_entry_with_size("src", true, 4096),
            make_fs_entry_with_size("CONTRIBUTING.md", false, 256),
        ]);
        explorer.sort_by(FileSorting::Size);
        // Directory has size 4096
        assert_eq!(explorer.files.get(0).unwrap().name(), "src");
        assert_eq!(explorer.files.get(1).unwrap().name(), "README.md");
        assert_eq!(explorer.files.get(2).unwrap().name(), "CONTRIBUTING.md");
    }

    #[test]
    fn test_fs_explorer_sort_by_name_and_dirs_first() {
        let mut explorer: FileExplorer = FileExplorer::default();
        // Create files (files are then sorted by name)
        explorer.set_files(vec![
            make_fs_entry("README.md", false),
            make_fs_entry("src", true),
            make_fs_entry("docs", true),
            make_fs_entry("CONTRIBUTING.md", false),
            make_fs_entry("CODE_OF_CONDUCT.md", false),
            make_fs_entry("CHANGELOG.md", false),
            make_fs_entry("LICENSE", false),
            make_fs_entry("Cargo.toml", false),
            make_fs_entry("Cargo.lock", false),
            make_fs_entry("codecov.yml", false),
        ]);
        explorer.sort_by(FileSorting::Name);
        explorer.group_dirs_by(Some(GroupDirs::First));
        // First entry should be "docs"
        assert_eq!(explorer.files.get(0).unwrap().name(), "docs");
        assert_eq!(explorer.files.get(1).unwrap().name(), "src");
        // 3rd is file first for alphabetical order
        assert_eq!(explorer.files.get(2).unwrap().name(), "Cargo.lock");
        // Last should be "README.md" (last file for alphabetical ordening)
        assert_eq!(explorer.files.get(9).unwrap().name(), "README.md");
    }

    #[test]
    fn test_fs_explorer_sort_by_name_and_dirs_last() {
        let mut explorer: FileExplorer = FileExplorer::default();
        // Create files (files are then sorted by name)
        explorer.set_files(vec![
            make_fs_entry("README.md", false),
            make_fs_entry("src", true),
            make_fs_entry("docs", true),
            make_fs_entry("CONTRIBUTING.md", false),
            make_fs_entry("CODE_OF_CONDUCT.md", false),
            make_fs_entry("CHANGELOG.md", false),
            make_fs_entry("LICENSE", false),
            make_fs_entry("Cargo.toml", false),
            make_fs_entry("Cargo.lock", false),
            make_fs_entry("codecov.yml", false),
        ]);
        explorer.sort_by(FileSorting::Name);
        explorer.group_dirs_by(Some(GroupDirs::Last));
        // Last entry should be "src"
        assert_eq!(explorer.files.get(8).unwrap().name(), "docs");
        assert_eq!(explorer.files.get(9).unwrap().name(), "src");
        // first is file for alphabetical order
        assert_eq!(explorer.files.get(0).unwrap().name(), "Cargo.lock");
        // Last in files should be "README.md" (last file for alphabetical ordening)
        assert_eq!(explorer.files.get(7).unwrap().name(), "README.md");
    }

    #[test]
    fn test_fs_explorer_fmt() {
        let explorer: FileExplorer = FileExplorer::default();
        // Create fs entry
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
            explorer.fmt_file(&entry),
            format!(
                "bar.txt                  -rw-r--r-- root         8.2 KB     {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
        #[cfg(windows)]
        assert_eq!(
            explorer.fmt_file(&entry),
            format!(
                "bar.txt                  -rw-r--r-- 0            8.2 KB     {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
    }

    #[test]
    fn test_fs_explorer_to_string_from_str_traits() {
        // File Sorting
        assert_eq!(FileSorting::CreationTime.to_string(), "by_creation_time");
        assert_eq!(FileSorting::ModifyTime.to_string(), "by_mtime");
        assert_eq!(FileSorting::Name.to_string(), "by_name");
        assert_eq!(FileSorting::Size.to_string(), "by_size");
        assert_eq!(
            FileSorting::from_str("by_creation_time").ok().unwrap(),
            FileSorting::CreationTime
        );
        assert_eq!(
            FileSorting::from_str("by_mtime").ok().unwrap(),
            FileSorting::ModifyTime
        );
        assert_eq!(
            FileSorting::from_str("by_name").ok().unwrap(),
            FileSorting::Name
        );
        assert_eq!(
            FileSorting::from_str("by_size").ok().unwrap(),
            FileSorting::Size
        );
        assert!(FileSorting::from_str("omar").is_err());
        // Group dirs
        assert_eq!(GroupDirs::First.to_string(), "first");
        assert_eq!(GroupDirs::Last.to_string(), "last");
        assert_eq!(GroupDirs::from_str("first").ok().unwrap(), GroupDirs::First);
        assert_eq!(GroupDirs::from_str("last").ok().unwrap(), GroupDirs::Last);
        assert!(GroupDirs::from_str("omar").is_err());
    }

    #[test]
    fn test_fs_explorer_del_entry() {
        let mut explorer: FileExplorer = FileExplorer::default();
        // Create files (files are then sorted by name)
        explorer.set_files(vec![
            make_fs_entry("CONTRIBUTING.md", false),
            make_fs_entry("docs", true),
            make_fs_entry("src", true),
            make_fs_entry("README.md", false),
        ]);
        explorer.del_entry(0);
        assert_eq!(explorer.files.len(), 3);
        assert_eq!(explorer.files[0].name(), "docs");
        explorer.del_entry(5);
        assert_eq!(explorer.files.len(), 3);
    }

    fn make_fs_entry(name: &str, is_dir: bool) -> File {
        let t: SystemTime = SystemTime::now();
        let metadata = Metadata {
            accessed: Some(t),
            created: Some(t),
            modified: Some(t),
            file_type: if is_dir {
                FileType::Directory
            } else {
                FileType::File
            },
            symlink: None,
            gid: Some(0),
            uid: Some(0),
            mode: Some(UnixPex::from(if is_dir { 0o755 } else { 0o644 })),
            size: 64,
        };
        File {
            path: PathBuf::from(name),
            metadata,
        }
    }

    fn make_fs_entry_with_size(name: &str, is_dir: bool, size: usize) -> File {
        let t: SystemTime = SystemTime::now();
        let metadata = Metadata {
            accessed: Some(t),
            created: Some(t),
            modified: Some(t),
            file_type: if is_dir {
                FileType::Directory
            } else {
                FileType::File
            },
            symlink: None,
            gid: Some(0),
            uid: Some(0),
            mode: Some(UnixPex::from(if is_dir { 0o755 } else { 0o644 })),
            size: size as u64,
        };
        File {
            path: PathBuf::from(name),
            metadata,
        }
    }
}
