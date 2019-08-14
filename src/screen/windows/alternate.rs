//! Implements platform specific functions to switch to an alternate screen.
use crate::shared::{Handle, TtyResult};


pub fn _enable_alt() -> TtyResult<()> {
    // (imdaveho) NOTE: emulating a single alternate screen like
    // UNIX; therefore this function will be wrapped at the top 
    // `Tty` level where there is an parameter, `altscrn: Option<Handle>`
    // if `altscrn` is not None, then `altscrn.show()?` else call
    // `Handle::buffer()?` and set `altscrn` to that output.
    Ok(())
}

pub fn _disable_alt() -> TtyResult<()> {
    let main = Handle::stdout()?;
    main.show()?;
    Ok(())
}