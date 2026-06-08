//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

use std::fmt;
use std::time::Instant;

use bytesize::ByteSize;

/// Tracks transfer progress with two exact levels.
///
/// - Partial: the current file's byte progress (`cur_written / cur_size`).
/// - Full: file-count weighted, `(files_completed + cur_fraction) / files_total`.
///
/// No total-size estimation: `files_total` is exact (from the pre-scan).
pub struct TransferProgress {
    files_total: usize,
    files_completed: usize,
    cur_file_size: usize,
    cur_file_written: usize,
    total_bytes_written: usize,
    pub(crate) started: Instant,
}

impl Default for TransferProgress {
    fn default() -> Self {
        Self {
            files_total: 0,
            files_completed: 0,
            cur_file_size: 0,
            cur_file_written: 0,
            total_bytes_written: 0,
            started: Instant::now(),
        }
    }
}

impl fmt::Display for TransferProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let eta = match self.calc_eta() {
            0 => String::from("--:--"),
            seconds => format!("{:02}:{:02}", seconds / 60, seconds % 60),
        };
        write!(
            f,
            "{} / {} — {:.1}% — ETA {} ({}/s)",
            ByteSize(self.cur_file_written as u64),
            ByteSize(self.cur_file_size as u64),
            self.calc_partial_progress() * 100.0,
            eta,
            ByteSize(self.calc_bytes_per_second()),
        )
    }
}

impl TransferProgress {
    /// Initialize for a new transfer batch with an exact file count.
    pub fn init(&mut self, files_total: usize) {
        self.files_total = files_total;
        self.files_completed = 0;
        self.cur_file_size = 0;
        self.cur_file_written = 0;
        self.total_bytes_written = 0;
        self.started = Instant::now();
    }

    /// Begin a new file with a known size.
    pub fn start_file(&mut self, size: usize) {
        self.cur_file_size = size;
        self.cur_file_written = 0;
    }

    /// Add bytes written to the current file.
    pub fn add_bytes(&mut self, delta: usize) {
        self.cur_file_written += delta;
        self.total_bytes_written += delta;
    }

    /// Mark the current file as fully transferred.
    ///
    /// Clears the current-file byte counters so a finished file is only ever
    /// counted via `files_completed` and never double-counted as an in-progress
    /// fraction in [`Self::calc_full_progress`].
    pub fn finish_file(&mut self) {
        self.files_completed += 1;
        self.cur_file_size = 0;
        self.cur_file_written = 0;
    }

    /// Mark an unchanged file as done (counts toward the full bar, no bytes).
    pub fn skip_file(&mut self) {
        self.files_completed += 1;
    }

    /// Fraction of the current file written (0.0..=1.0). Zero-byte file => 1.0.
    pub fn calc_partial_progress(&self) -> f64 {
        if self.cur_file_size == 0 {
            return 1.0;
        }
        (self.cur_file_written as f64 / self.cur_file_size as f64).min(1.0)
    }

    /// Overall progress (0.0..=1.0): file-weighted with intra-file interpolation.
    ///
    /// The current file only contributes a fraction while it is genuinely in
    /// progress (`cur_file_size > 0` and not all files completed). A finished
    /// file clears `cur_file_size` (see [`Self::finish_file`]) so it is counted
    /// exactly once via `files_completed`.
    pub fn calc_full_progress(&self) -> f64 {
        if self.files_total == 0 {
            return 0.0;
        }
        let cur_fraction = if self.cur_file_size == 0 || self.files_completed >= self.files_total {
            0.0
        } else {
            self.calc_partial_progress()
        };
        ((self.files_completed as f64 + cur_fraction) / self.files_total as f64).min(1.0)
    }

    pub fn is_single_file(&self) -> bool {
        self.files_total <= 1
    }

    #[cfg(test)]
    pub fn total_bytes_written(&self) -> usize {
        self.total_bytes_written
    }

    #[cfg(test)]
    pub fn files_completed(&self) -> usize {
        self.files_completed
    }

    /// Bytes per second over the whole transfer.
    pub fn calc_bytes_per_second(&self) -> u64 {
        let elapsed_secs = self.started.elapsed().as_secs();
        match elapsed_secs {
            0 => self.total_bytes_written as u64,
            _ => self.total_bytes_written as u64 / elapsed_secs,
        }
    }

    /// ETA in seconds based on full progress.
    pub fn calc_eta(&self) -> u64 {
        let elapsed_secs = self.started.elapsed().as_secs();
        let percent = (self.calc_full_progress() * 100.0) as u64;
        match percent {
            0 => 0,
            p => ((elapsed_secs * 100) / p).saturating_sub(elapsed_secs),
        }
    }

    /// File counter for the full-bar title: "(7/312)".
    pub fn file_count_display(&self) -> String {
        format!(
            "({}/{})",
            self.files_completed.min(self.files_total),
            self.files_total
        )
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
        self.progress.total_bytes_written
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
        assert_eq!(progress.calc_full_progress(), 0.0);
        assert!(progress.is_single_file());

        progress.init(1);
        assert!(progress.is_single_file());
        assert_eq!(progress.calc_full_progress(), 0.0);

        progress.start_file(1024);
        assert_eq!(progress.calc_partial_progress(), 0.0);
        assert_eq!(progress.calc_full_progress(), 0.0);

        // Partial and full track together for a single file.
        progress.add_bytes(256);
        assert!((progress.calc_partial_progress() - 0.25).abs() < 1e-9);
        assert!((progress.calc_full_progress() - 0.25).abs() < 1e-9);

        progress.add_bytes(768);
        assert!((progress.calc_partial_progress() - 1.0).abs() < 1e-9);

        progress.finish_file();
        assert!((progress.calc_full_progress() - 1.0).abs() < 1e-9);
        assert_eq!(progress.files_completed(), 1);
    }

    #[test]
    fn test_transfer_progress_multi_file() {
        let mut progress = TransferProgress::default();
        progress.init(4);
        assert!(!progress.is_single_file());

        // File 1 fully transferred => full ≈ 0.25
        progress.start_file(1000);
        progress.add_bytes(1000);
        progress.finish_file();
        assert!((progress.calc_full_progress() - 0.25).abs() < 1e-9);

        // File 2 half transferred => partial ≈ 0.5, full ≈ 0.375
        progress.start_file(1000);
        progress.add_bytes(500);
        assert!((progress.calc_partial_progress() - 0.5).abs() < 1e-9);
        assert!((progress.calc_full_progress() - 0.375).abs() < 1e-9);
    }

    #[test]
    fn test_transfer_progress_skipped_file() {
        let mut progress = TransferProgress::default();
        progress.init(2);

        // One file actually transferred.
        progress.start_file(100);
        progress.add_bytes(100);
        progress.finish_file();

        // One file skipped (unchanged): counts toward completion, no bytes.
        progress.skip_file();

        assert_eq!(progress.files_completed(), 2);
        assert!((progress.calc_full_progress() - 1.0).abs() < 1e-9);
        // Only the transferred file contributes bytes.
        assert_eq!(progress.total_bytes_written(), 100);
    }

    #[test]
    fn test_transfer_progress_zero_size_file() {
        let mut progress = TransferProgress::default();
        progress.init(1);

        progress.start_file(0);
        assert!((progress.calc_partial_progress() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_transfer_progress_timing() {
        let mut progress = TransferProgress::default();
        progress.init(1);
        progress.start_file(1024);

        progress.started = progress
            .started
            .checked_sub(Duration::from_secs(4))
            .unwrap();
        progress.add_bytes(256);

        // 256 bytes over 4 seconds => 64 bytes/s
        assert_eq!(progress.calc_bytes_per_second(), 64);
        // 25% done after 4s => total 16s => 12s remaining
        assert_eq!(progress.calc_eta(), 12);
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
