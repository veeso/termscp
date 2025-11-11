use cfg_aliases::cfg_aliases;
use vergen_git2::{BuildBuilder, CargoBuilder, Emitter, Git2Builder, RustcBuilder, SysinfoBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let build = BuildBuilder::all_build()?;
    let cargo = CargoBuilder::all_cargo()?;
    let git2 = Git2Builder::all_git()?;
    let rustc = RustcBuilder::all_rustc()?;
    let si = SysinfoBuilder::all_sysinfo()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&git2)?
        .add_instructions(&rustc)?
        .add_instructions(&si)?
        .emit()?;

    Ok(())
}
