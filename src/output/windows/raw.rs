//! Implements platform specific method to convert output into raw mode.
use crate::shared::Handle;
use winapi::um::wincon::{
    ENABLE_LINE_INPUT, ENABLE_WRAP_AT_EOL_OUTPUT
};
use std::io::Result;
use super::Termios;


pub fn _enable_raw() -> Result<()> {
    let handle = Handle::conout()?;
    let mode = handle.get_mode()?;
    let mask = ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_LINE_INPUT;
    let raw_mode = mode & !mask;
    handle.set_mode(&raw_mode)?;
    Ok(())
}

pub fn _disable_raw() -> Result<()> {
    let handle = Handle::conout()?;
    let mode = handle.get_mode()?;
    let mask = ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_LINE_INPUT;
    let cooked_mode = mode | mask;
    handle.set_mode(&cooked_mode)?;
    Ok(())
}

// pub fn _set_terminal_attr(mode: &u32) -> Result<()> {
//     // Since this just takes the current output handle
//     // and calls `SetConsoleMode` on it, this would be
//     // more explicit and clear through a method on the 
//     // Handle struct.
//     let handle = Handle::conout()?;
//     unsafe {
//         if SetConsoleMode(handle.0, mode) == 0 {
//             return Err(Error::last_os_error());
//         }
//     }
//     Ok(())
// }

pub fn _get_terminal_attr() -> Result<Termios> {
    // Stdout because if you're creating a 
    // new screen via alternate screen, you
    // want a default set of terminal settings
    Handle::stdout().unwrap().get_mode()
}