//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

use std::path::PathBuf;
use std::str::FromStr;

// locals
use super::{FileTransferActivity, LogLevel};

/// Terminal command
#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Cd(String),
    Exec(String),
    Exit,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match parts.next() {
            Some("cd") => {
                if let Some(path) = parts.next() {
                    Ok(Command::Cd(path.to_string()))
                } else {
                    Err("cd command requires a path".to_string())
                }
            }
            Some("exit") | Some("logout") => Ok(Command::Exit),
            Some(cmd) => Ok(Command::Exec(cmd.to_string())),
            None => Err("".to_string()),
        }
    }
}

impl FileTransferActivity {
    pub(crate) fn action_exec_cmd(&mut self, input: String) {
        self.action_exec(input);
    }

    fn action_exec(&mut self, cmd: String) {
        if cmd.is_empty() {
            self.print_terminal("".to_string());
        }

        let cmd = match Command::from_str(&cmd) {
            Ok(cmd) => cmd,
            Err(err) => {
                self.log(LogLevel::Error, format!("Invalid command: {err}"));
                self.print_terminal(err);
                return;
            }
        };

        match cmd {
            Command::Cd(path) => {
                self.action_exec_cd(path);
            }
            Command::Exec(executable) => {
                self.action_exec_executable(executable);
            }
            Command::Exit => {
                self.action_exec_exit();
            }
        }
    }

    fn action_exec_exit(&mut self) {
        self.browser.toggle_terminal(false);
        self.umount_exec();
    }

    fn action_exec_cd(&mut self, input: String) {
        let dir_path = self.pane_to_abs_path(PathBuf::from(input.as_str()).as_path());
        self.pane_changedir(dir_path.as_path(), true);

        self.update_browser_file_list();

        // update prompt and print the new directory
        self.update_terminal_prompt();
        self.print_terminal(dir_path.display().to_string());
    }

    /// Execute a command via the active tab's pane.
    fn action_exec_executable(&mut self, cmd: String) {
        match self.browser.fs_pane_mut().fs.exec(cmd.as_str()) {
            Ok(output) => {
                self.print_terminal(output);
            }
            Err(err) => {
                self.log(
                    LogLevel::Error,
                    format!("Could not execute command \"{cmd}\": {err}"),
                );
                self.print_terminal(err.to_string());
            }
        }
    }
}
