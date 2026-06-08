use cfg_aliases::cfg_aliases;

fn main() {
    // Setup cfg aliases
    cfg_aliases! {
        // Platforms
        macos: { target_os = "macos" },
        linux: { target_os = "linux" },
        posix: { target_family = "unix" },
        win: { target_family = "windows" },
        // exclusive features
        smb: { feature = "smb" },
        smb_unix: { all(unix, feature = "smb") },
        smb_windows: { all(windows, feature = "smb") }
    }
}
