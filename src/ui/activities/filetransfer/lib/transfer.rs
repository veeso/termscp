//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

use std::fmt;
use std::time::Instant;

use bytesize::ByteSize;

/// Tracks overall transfer progress with byte-level estimation.
///
/// For single-file transfers, progress is exact (known file size).
/// For multi-file transfers, uses lazy accumulation: as each file starts,
/// its size is added to `known_total_bytes`, and the remaining files'
/// total is estimated from the running average file size.
pub struct TransferProgress {
    files_completed: usize,
    files_total: usize,
    bytes_written: usize,
    known_total_bytes: usize,
    nonzero_files_started: usize,
    files_started: usize,
    pub(crate) started: Instant,
}

impl Default for TransferProgress {
    fn default() -> Self {
        Self {
            files_completed: 0,
            files_total: 0,
            bytes_written: 0,
            known_total_bytes: 0,
            nonzero_files_started: 0,
            files_started: 0,
            started: Instant::now(),
        }
    }
}

impl fmt::Display for TransferProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let eta = match self.calc_eta() {
            0 => String::from("--:--"),
            seconds => format!(
                "{:0width$}:{:0width$}",
                seconds / 60,
                seconds % 60,
                width = 2
            ),
        };
        if self.is_single_file() {
            write!(
                f,
                "{} / {} — {:.1}% — ETA {} ({}/s)",
                ByteSize(self.bytes_written as u64),
                ByteSize(self.known_total_bytes as u64),
                self.calc_progress() * 100.0,
                eta,
                ByteSize(self.calc_bytes_per_second()),
            )
        } else {
            write!(
                f,
                "{} transferred — ~{:.1}% — ETA {} ({}/s)",
                ByteSize(self.bytes_written as u64),
                self.calc_progress() * 100.0,
                eta,
                ByteSize(self.calc_bytes_per_second()),
            )
        }
    }
}

impl TransferProgress {
    /// Initialize for a new transfer batch.
    pub fn init(&mut self, total_files: usize) {
        self.files_completed = 0;
        self.files_total = total_files;
        self.bytes_written = 0;
        self.known_total_bytes = 0;
        self.nonzero_files_started = 0;
        self.files_started = 0;
        self.started = Instant::now();
    }

    /// Update files_total without resetting byte accumulators.
    /// Used by recursive directory transfers that re-discover file counts.
    pub fn set_files_total(&mut self, total: usize) {
        self.files_total = total;
    }

    /// Register a file that is about to be transferred.
    pub fn register_file(&mut self, size: usize) {
        self.files_started += 1;
        if size > 0 {
            self.nonzero_files_started += 1;
            self.known_total_bytes += size;
        }
    }

    /// Register a file that was skipped (unchanged).
    /// Atomically registers, adds bytes, and increments completion.
    pub fn register_skipped_file(&mut self, size: usize) {
        self.register_file(size);
        self.add_bytes(size);
        self.increment();
    }

    /// Add transferred bytes.
    pub fn add_bytes(&mut self, delta: usize) {
        self.bytes_written += delta;
    }

    /// Mark one file as completed.
    pub fn increment(&mut self) {
        self.files_completed += 1;
    }

    /// Estimate the total transfer size in bytes.
    pub fn estimated_total(&self) -> usize {
        if self.files_total <= 1 || self.files_started >= self.files_total {
            return self.known_total_bytes;
        }
        if self.nonzero_files_started == 0 {
            return self.known_total_bytes;
        }
        let avg = self.known_total_bytes / self.nonzero_files_started;
        let remaining = self.files_total - self.files_started;
        self.known_total_bytes + avg * remaining
    }

    /// Calculate progress as 0.0..=1.0.
    pub fn calc_progress(&self) -> f64 {
        let total = self.estimated_total();
        if total == 0 {
            return 0.0;
        }
        (self.bytes_written as f64 / total as f64).min(1.0)
    }

    pub fn is_single_file(&self) -> bool {
        self.files_total <= 1
    }

    #[cfg(test)]
    pub fn bytes_written(&self) -> usize {
        self.bytes_written
    }

    #[cfg(test)]
    pub fn files_completed(&self) -> usize {
        self.files_completed
    }

    #[cfg(test)]
    pub fn files_started(&self) -> usize {
        self.files_started
    }

    /// Calculate bytes per second based on elapsed time.
    pub fn calc_bytes_per_second(&self) -> u64 {
        let elapsed_secs = self.started.elapsed().as_secs();
        match elapsed_secs {
            0 => {
                if self.bytes_written > 0 && self.bytes_written >= self.estimated_total() {
                    self.bytes_written as u64
                } else {
                    0
                }
            }
            _ => self.bytes_written as u64 / elapsed_secs,
        }
    }

    /// Calculate ETA in seconds.
    pub fn calc_eta(&self) -> u64 {
        let elapsed_secs = self.started.elapsed().as_secs();
        let percent = self.calc_progress() * 100.0;
        match percent as u64 {
            0 => 0,
            p => ((elapsed_secs * 100) / p) - elapsed_secs,
        }
    }

    /// Format the file count string for multi-file title: "(3/12)"
    /// Uses `files_started` (not `files_completed`) so the display shows the
    /// file currently being transferred, not the last one that finished.
    pub fn file_count_display(&self) -> String {
        format!("({}/{})", self.files_started, self.files_total)
    }
}

/// Contains the states related to the transfer process.
pub struct TransferStates {
    aborted: bool,
    pub progress: TransferProgress,
}

impl Default for TransferStates {
    fn default() -> Self {
        Self::new()
    }
}

impl TransferStates {
    pub fn new() -> Self {
        Self {
            aborted: false,
            progress: TransferProgress::default(),
        }
    }

    pub fn reset(&mut self) {
        self.aborted = false;
    }

    pub fn abort(&mut self) {
        self.aborted = true;
    }

    pub fn aborted(&self) -> bool {
        self.aborted
    }

    /// Total bytes transferred (for notification threshold).
    pub fn full_size(&self) -> usize {
        self.progress.bytes_written
    }
}

// -- Options

/// Defines the transfer options for transfer actions
#[derive(Default)]
pub struct TransferOpts {
    /// Save file as
    pub save_as: Option<String>,
}

impl TransferOpts {
    /// Define the name of the file to be saved
    pub fn save_as<S: AsRef<str>>(mut self, n: Option<S>) -> Self {
        self.save_as = n.map(|x| x.as_ref().to_string());
        self
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_transfer_progress_single_file() {
        let mut progress = TransferProgress::default();
        assert_eq!(progress.calc_progress(), 0.0);
        assert!(progress.is_single_file());

        progress.init(1);
        assert!(progress.is_single_file());
        assert_eq!(progress.calc_progress(), 0.0);

        progress.register_file(1024);
        assert_eq!(progress.estimated_total(), 1024);
        assert_eq!(progress.calc_progress(), 0.0);

        progress.add_bytes(256);
        assert_eq!(progress.bytes_written(), 256);
        assert_eq!(progress.calc_progress(), 0.25);

        progress.add_bytes(768);
        assert_eq!(progress.calc_progress(), 1.0);
        progress.increment();
        assert_eq!(progress.files_completed(), 1);
    }

    #[test]
    fn test_transfer_progress_multi_file() {
        let mut progress = TransferProgress::default();
        progress.init(4);
        assert!(!progress.is_single_file());

        progress.register_file(1000);
        assert_eq!(progress.estimated_total(), 4000);

        progress.add_bytes(1000);
        progress.increment();
        assert!((progress.calc_progress() - 0.25).abs() < 0.001);

        progress.register_file(500);
        assert_eq!(progress.estimated_total(), 3000);

        progress.add_bytes(500);
        progress.increment();
        assert!((progress.calc_progress() - 0.5).abs() < 0.001);

        progress.register_file(500);
        assert_eq!(progress.estimated_total(), 2666);

        progress.add_bytes(500);
        progress.increment();

        progress.register_file(2000);
        assert_eq!(progress.estimated_total(), 4000);

        progress.add_bytes(2000);
        progress.increment();
        assert_eq!(progress.calc_progress(), 1.0);
    }

    #[test]
    fn test_transfer_progress_skipped_file() {
        let mut progress = TransferProgress::default();
        progress.init(3);

        progress.register_file(100);
        progress.add_bytes(100);
        progress.increment();

        progress.register_skipped_file(200);
        assert_eq!(progress.bytes_written(), 300);
        assert_eq!(progress.files_completed(), 2);
        assert_eq!(progress.files_started(), 2);
    }

    #[test]
    fn test_transfer_progress_zero_size_files() {
        let mut progress = TransferProgress::default();
        progress.init(3);

        progress.register_file(0);
        progress.add_bytes(0);
        progress.increment();

        progress.register_file(1000);
        assert_eq!(progress.estimated_total(), 2000);
    }

    #[test]
    fn test_transfer_progress_set_files_total() {
        let mut progress = TransferProgress::default();
        progress.init(2);
        progress.register_file(500);
        progress.add_bytes(500);
        progress.increment();

        progress.set_files_total(3);
        assert_eq!(progress.bytes_written(), 500);
        assert_eq!(progress.files_started(), 1);
    }

    #[test]
    fn test_transfer_progress_timing() {
        let mut progress = TransferProgress::default();
        progress.init(1);
        progress.register_file(1024);

        progress.started = progress
            .started
            .checked_sub(Duration::from_secs(4))
            .unwrap();
        progress.add_bytes(256);

        assert_eq!(progress.calc_bytes_per_second(), 64);
        assert_eq!(progress.calc_eta(), 12);
    }

    #[test]
    fn test_transfer_progress_display_single() {
        let mut progress = TransferProgress::default();
        progress.init(1);
        progress.register_file(1024);
        progress.started = progress
            .started
            .checked_sub(Duration::from_secs(4))
            .unwrap();
        progress.add_bytes(256);

        let display = progress.to_string();
        assert!(display.contains("/ 1.0 KiB"));
        assert!(!display.contains('~'));
    }

    #[test]
    fn test_transfer_progress_display_multi() {
        let mut progress = TransferProgress::default();
        progress.init(4);
        progress.register_file(1024);
        progress.started = progress
            .started
            .checked_sub(Duration::from_secs(4))
            .unwrap();
        progress.add_bytes(256);

        let display = progress.to_string();
        assert!(display.contains("transferred"));
        assert!(display.contains('~'));
    }

    #[test]
    fn test_transfer_states() {
        let mut states = TransferStates::default();
        assert!(!states.aborted());
        states.abort();
        assert!(states.aborted());
        states.reset();
        assert!(!states.aborted());
    }

    #[test]
    fn transfer_opts() {
        let opts = TransferOpts::default();
        assert!(opts.save_as.is_none());
        let opts = TransferOpts::default().save_as(Some("omar.txt"));
        assert_eq!(opts.save_as.as_deref().unwrap(), "omar.txt");
    }
}
