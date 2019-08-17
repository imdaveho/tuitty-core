//! Implements platform specific functions to switch to an alternate screen.
use crate::shared::{Handle, TtyResult};


// pub fn _enable_alt(handle: &Handle) -> Result<()> {
//     handle.show()?;
//     Ok(())
// }

pub fn _disable_alt() -> TtyResult<()> {
    let handle = Handle::stdout()?;
    handle.show()?;
    Ok(())
}