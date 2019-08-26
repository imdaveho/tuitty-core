// Windows Console API handling of disabling the alternate screen. (Enabling is
// put into the top `wincon` module).
//
// Typically you work on the main screen but there are cases where you may want
// to switch to an temporary alternate screen. The alternative screen on Windows
// is created by associating a new `Handle` with some kind of `File` with Read /
// Write traits.

use super::Handle;


pub fn _disable_alt() -> Result<()> {
    let handle = Handle::stdout()?;
    handle.show()?;
    Ok(())
}

// pub fn _enable_alt(handle: &Handle) -> Result<()> { ... }
