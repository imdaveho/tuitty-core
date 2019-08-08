//! Implements platform specific functions to switch to an alternate screen.
use crate::{csi, write_cout};
use super::{TtyResult, Write};


pub fn _enable_alt() -> TtyResult<()> {
    write_cout!(csi!("?1049h"))?;
    Ok(())
}


pub fn _disable_alt() -> TtyResult<()> {
    write_cout!(csi!("?1049l"))?;
    Ok(())
}
