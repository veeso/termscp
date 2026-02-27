//! ## Popups
//!
//! popups components

mod chmod;
mod copy;
mod delete;
mod disconnect;
mod error;
mod file_info;
mod filter;
mod goto;
mod keybindings;
mod mkdir;
mod newfile;
mod open_with;
mod progress_bar;
mod quit;
mod rename;
mod replace;
mod save_as;
mod sorting;
mod status_bar;
mod symlink;
mod wait;
mod watcher;

pub use self::chmod::ChmodPopup;
pub use self::copy::CopyPopup;
pub use self::delete::DeletePopup;
pub use self::disconnect::DisconnectPopup;
pub use self::error::{ErrorPopup, FatalPopup};
pub use self::file_info::FileInfoPopup;
pub use self::filter::FilterPopup;
pub use self::goto::{ATTR_FILES, GotoPopup};
pub use self::keybindings::KeybindingsPopup;
pub use self::mkdir::{MkdirPopup, SyncBrowsingMkdirPopup};
pub use self::newfile::NewfilePopup;
pub use self::open_with::OpenWithPopup;
pub use self::progress_bar::{ProgressBarFull, ProgressBarPartial};
pub use self::quit::QuitPopup;
pub use self::rename::RenamePopup;
pub use self::replace::ReplacePopup;
pub use self::save_as::SaveAsPopup;
pub use self::sorting::SortingPopup;
pub use self::status_bar::{StatusBarLocal, StatusBarRemote};
pub use self::symlink::SymlinkPopup;
pub use self::wait::{WaitPopup, WalkdirWaitPopup};
pub use self::watcher::{WatchedPathsList, WatcherPopup};
