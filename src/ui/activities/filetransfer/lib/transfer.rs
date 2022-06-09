//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

use bytesize::ByteSize;
use std::fmt;
use std::time::Instant;

// -- States and progress

/// ### TransferStates
///
/// TransferStates contains the states related to the transfer process
pub struct TransferStates {
    aborted: bool,               // Describes whether the transfer process has been aborted
    pub full: ProgressStates,    // full transfer states
    pub partial: ProgressStates, // Partial transfer states
}

/// ### ProgressStates
///
/// Progress states describes the states for the progress of a single transfer part
pub struct ProgressStates {
    started: Instant,
    total: usize,
    written: usize,
}

impl Default for TransferStates {
    fn default() -> Self {
        Self::new()
    }
}

impl TransferStates {
    /// Instantiates a new transfer states
    pub fn new() -> TransferStates {
        TransferStates {
            aborted: false,
            full: ProgressStates::default(),
            partial: ProgressStates::default(),
        }
    }

    /// Re-intiialize transfer states
    pub fn reset(&mut self) {
        self.aborted = false;
    }

    /// Set aborted to true
    pub fn abort(&mut self) {
        self.aborted = true;
    }

    /// Returns whether transfer has been aborted
    pub fn aborted(&self) -> bool {
        self.aborted
    }

    /// Returns the size of the entire transfer
    pub fn full_size(&self) -> usize {
        self.full.total
    }
}

impl Default for ProgressStates {
    fn default() -> Self {
        ProgressStates {
            started: Instant::now(),
            written: 0,
            total: 0,
        }
    }
}

impl fmt::Display for ProgressStates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let eta: String = match self.calc_eta() {
            0 => String::from("--:--"),
            seconds => format!(
                "{:0width$}:{:0width$}",
                (seconds / 60),
                (seconds % 60),
                width = 2
            ),
        };
        write!(
            f,
            "{:.2}% - ETA {} ({}/s)",
            self.calc_progress_percentage(),
            eta,
            ByteSize(self.calc_bytes_per_second())
        )
    }
}

impl ProgressStates {
    /// Initialize a new Progress State
    pub fn init(&mut self, sz: usize) {
        self.started = Instant::now();
        self.total = sz;
        self.written = 0;
    }

    /// Update progress state
    pub fn update_progress(&mut self, delta: usize) -> f64 {
        self.written += delta;
        self.calc_progress_percentage()
    }

    /// Calculate progress in a range between 0.0 to 1.0
    pub fn calc_progress(&self) -> f64 {
        // Prevent dividing by 0
        if self.total == 0 {
            return 0.0;
        }
        let prog: f64 = (self.written as f64) / (self.total as f64);
        match prog > 1.0 {
            true => 1.0,
            false => prog,
        }
    }

    /// Get started
    pub fn started(&self) -> Instant {
        self.started
    }

    /// Calculate the current transfer progress as percentage
    fn calc_progress_percentage(&self) -> f64 {
        self.calc_progress() * 100.0
    }

    /// Generic function to calculate bytes per second using elapsed time since transfer started and the bytes written
    /// and the total amount of bytes to write
    pub fn calc_bytes_per_second(&self) -> u64 {
        // bytes_written : elapsed_secs = x : 1
        let elapsed_secs: u64 = self.started.elapsed().as_secs();
        match elapsed_secs {
            0 => match self.written == self.total {
                // NOTE: would divide by 0 :D
                true => self.total as u64, // Download completed in less than 1 second
                false => 0,                // 0 B/S
            },
            _ => self.written as u64 / elapsed_secs,
        }
    }

    /// Calculate ETA for current transfer as seconds
    fn calc_eta(&self) -> u64 {
        let elapsed_secs: u64 = self.started.elapsed().as_secs();
        let prog: f64 = self.calc_progress_percentage();
        match prog as u64 {
            0 => 0,
            _ => ((elapsed_secs * 100) / (prog as u64)) - elapsed_secs,
        }
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

    use super::*;

    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[test]
    fn test_ui_activities_filetransfer_lib_transfer_progress_states() {
        let mut states: ProgressStates = ProgressStates::default();
        assert_eq!(states.total, 0);
        assert_eq!(states.written, 0);
        assert!(states.started().elapsed().as_secs() < 5);
        // Init new transfer
        states.init(1024);
        assert_eq!(states.total, 1024);
        assert_eq!(states.written, 0);
        assert_eq!(states.calc_bytes_per_second(), 0);
        assert_eq!(states.calc_eta(), 0);
        assert_eq!(states.calc_progress_percentage(), 0.0);
        assert_eq!(states.calc_progress(), 0.0);
        assert_eq!(states.to_string().as_str(), "0.00% - ETA --:-- (0 B/s)");
        // Wait 4 second (virtually)
        states.started = states.started.checked_sub(Duration::from_secs(4)).unwrap();
        // Update state
        states.update_progress(256);
        assert_eq!(states.total, 1024);
        assert_eq!(states.written, 256);
        assert_eq!(states.calc_bytes_per_second(), 64); // 256 bytes in 4 seconds
        assert_eq!(states.calc_eta(), 12); // 16 total sub 4
        assert_eq!(states.calc_progress_percentage(), 25.0);
        assert_eq!(states.calc_progress(), 0.25);
        assert_eq!(states.to_string().as_str(), "25.00% - ETA 00:12 (64 B/s)");
        // 100%
        states.started = states.started.checked_sub(Duration::from_secs(12)).unwrap();
        states.update_progress(768);
        assert_eq!(states.total, 1024);
        assert_eq!(states.written, 1024);
        assert_eq!(states.calc_bytes_per_second(), 64); // 256 bytes in 4 seconds
        assert_eq!(states.calc_eta(), 0); // 16 total sub 4
        assert_eq!(states.calc_progress_percentage(), 100.0);
        assert_eq!(states.calc_progress(), 1.0);
        assert_eq!(states.to_string().as_str(), "100.00% - ETA --:-- (64 B/s)");
        // Check if terminated at started
        states.started = Instant::now();
        assert_eq!(states.calc_bytes_per_second(), 1024);
        // Divide by zero
        let states: ProgressStates = ProgressStates::default();
        assert_eq!(states.total, 0);
        assert_eq!(states.written, 0);
        assert_eq!(states.calc_progress(), 0.0);
    }

    #[test]
    fn test_ui_activities_filetransfer_lib_transfer_states() {
        let mut states: TransferStates = TransferStates::default();
        assert_eq!(states.aborted, false);
        assert_eq!(states.full.total, 0);
        assert_eq!(states.full.written, 0);
        assert!(states.full.started.elapsed().as_secs() < 5);
        assert_eq!(states.partial.total, 0);
        assert_eq!(states.partial.written, 0);
        assert!(states.partial.started.elapsed().as_secs() < 5);
        // Aborted
        states.abort();
        assert_eq!(states.aborted(), true);
        states.reset();
        assert_eq!(states.aborted(), false);
        states.full.total = 1024;
        assert_eq!(states.full_size(), 1024);
    }

    #[test]
    fn transfer_opts() {
        let opts = TransferOpts::default();
        assert!(opts.save_as.is_none());
        let opts = TransferOpts::default().save_as(Some("omar.txt"));
        assert_eq!(opts.save_as.as_deref().unwrap(), "omar.txt");
    }
}
