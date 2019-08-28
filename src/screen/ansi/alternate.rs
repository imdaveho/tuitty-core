// ANSI functions for enabling and disabling the alternate screen.
//
// Typically you work on the main screen but there are cases where you may want
// to switch to an temporary alternate screen. This alternative screen is
// somewhat different from a normal screen. It has the exact dimensions of the
// terminal window, without any scroll back region. It acts as its own screen
// that contains its own settings and configuration until disabled.

// For example, Vim uses the entirety of the screen to edit the file, then exits
// to bash leaving the original buffer unchanged. This is the same behavior that
// is implemented here.

use super::{csi, write_cout, Result};


pub fn enable_alt() -> Result<()> {
    write_cout!(csi!("?1049h"))?;
    Ok(())
}


pub fn disable_alt() -> Result<()> {
    write_cout!(csi!("?1049l"))?;
    Ok(())
}
