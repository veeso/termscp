use cfg_aliases::cfg_aliases;

fn main() {
    // Setup cfg aliases
    cfg_aliases! {
        // Platforms
        macos: { target_os = "macos" },
        linux: { target_os = "linux" },
        unix: { target_family = "unix" },
        windows: { target_family = "windows" },
        // exclusive features
        smb: { not( macos ) },
        smb_unix: { all(unix, not(macos)) }
    }
}
