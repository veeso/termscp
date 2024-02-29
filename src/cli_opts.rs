//! ## CLI opts
//!
//! defines the types for main.rs types

use std::path::PathBuf;
use std::time::Duration;

use argh::FromArgs;

use crate::activity_manager::NextActivity;
use crate::filetransfer::FileTransferParams;
use crate::system::logging::LogLevel;

pub enum Task {
    Activity(NextActivity),
    ImportTheme(PathBuf),
    InstallUpdate,
}

#[derive(FromArgs)]
#[argh(description = "
where positional can be: 
        - [address]         [local-wrkdir]
    OR
        - [bookmark-Name]   [local-wrkdir]

Address syntax can be:

    - `protocol://user@address:port:wrkdir` for protocols such as Sftp, Scp, Ftp
    - `s3://bucket-name@region:profile:/wrkdir` for Aws S3 protocol
    - `\\\\<server>[:port]\\<share>[\\path]` for SMB (on Windows)
    - `smb://[user@]<server>[:port]</share>[/path]` for SMB (on other systems)

Please, report issues to <https://github.com/veeso/termscp>
Please, consider supporting the author <https://ko-fi.com/veeso>")]
pub struct Args {
    #[argh(subcommand)]
    pub nested: Option<ArgsSubcommands>,
    /// resolve address argument as a bookmark name
    #[argh(switch, short = 'b')]
    pub address_as_bookmark: bool,
    /// enable TRACE log level
    #[argh(switch, short = 'D')]
    pub debug: bool,
    /// provide password from CLI
    #[argh(option, short = 'P')]
    pub password: Option<String>,
    /// disable logging
    #[argh(switch, short = 'q')]
    pub quiet: bool,
    /// set UI ticks; default 10ms
    #[argh(option, short = 'T', default = "10")]
    pub ticks: u64,
    /// print version
    #[argh(switch, short = 'v')]
    pub version: bool,
    // -- positional
    #[argh(
        positional,
        description = "protocol://user@address:port:wrkdir local-wrkdir"
    )]
    pub positional: Vec<String>,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum ArgsSubcommands {
    Config(ConfigArgs),
    LoadTheme(LoadThemeArgs),
    Update(UpdateArgs),
}

#[derive(FromArgs)]
/// open termscp configuration
#[argh(subcommand, name = "config")]
pub struct ConfigArgs {}

#[derive(FromArgs)]
/// import the specified theme
#[argh(subcommand, name = "update")]
pub struct UpdateArgs {}

#[derive(FromArgs)]
/// import the specified theme
#[argh(subcommand, name = "theme")]
pub struct LoadThemeArgs {
    #[argh(positional)]
    /// theme file
    pub theme: PathBuf,
}

pub struct RunOpts {
    pub remote: Remote,
    pub ticks: Duration,
    pub log_level: LogLevel,
    pub task: Task,
}

impl RunOpts {
    pub fn config() -> Self {
        Self {
            task: Task::Activity(NextActivity::SetupActivity),
            ..Default::default()
        }
    }

    pub fn update() -> Self {
        Self {
            task: Task::InstallUpdate,
            ..Default::default()
        }
    }

    pub fn import_theme(theme: PathBuf) -> Self {
        Self {
            task: Task::ImportTheme(theme),
            ..Default::default()
        }
    }
}

impl Default for RunOpts {
    fn default() -> Self {
        Self {
            remote: Remote::None,
            ticks: Duration::from_millis(10),
            log_level: LogLevel::Info,
            task: Task::Activity(NextActivity::Authentication),
        }
    }
}

#[allow(clippy::large_enum_variant)]
pub enum Remote {
    Bookmark(BookmarkParams),
    Host(HostParams),
    None,
}

pub struct BookmarkParams {
    pub name: String,
    pub password: Option<String>,
}

pub struct HostParams {
    pub params: FileTransferParams,
    pub password: Option<String>,
}

impl BookmarkParams {
    pub fn new<S: AsRef<str>>(name: S, password: Option<S>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            password: password.map(|x| x.as_ref().to_string()),
        }
    }
}

impl HostParams {
    pub fn new<S: AsRef<str>>(params: FileTransferParams, password: Option<S>) -> Self {
        Self {
            params,
            password: password.map(|x| x.as_ref().to_string()),
        }
    }
}
