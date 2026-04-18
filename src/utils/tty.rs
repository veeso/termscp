//! ## Utils
//!
//! `Utils` implements utilities functions to work with layouts

use tuirealm::terminal::TerminalAdapter;

/// Read a secret from tty with customisable prompt
pub fn read_secret_from_tty<T>(
    terminal: &mut T,
    prompt: impl ToString,
) -> std::io::Result<Option<String>>
where
    T: TerminalAdapter,
{
    let _ = terminal.disable_raw_mode();
    let _ = terminal.leave_alternate_screen();
    let res = match rpassword::prompt_password(prompt) {
        Ok(p) if p.is_empty() => Ok(None),
        Ok(p) => Ok(Some(p)),
        Err(err) => Err(err),
    };

    let _ = terminal.enter_alternate_screen();
    let _ = terminal.enable_raw_mode();

    res
}
