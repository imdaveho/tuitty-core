//! Implements platform specific method to convert output into raw mode.
use crate::shared::{Handle, Termios};
use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::wincon::{ENABLE_LINE_INPUT, ENABLE_WRAP_AT_EOL_OUTPUT};
use std::io::{Error, Result};


pub fn _enable_raw() -> Result<()> {
    let mut termios = _get_terminal_attr()?;
    let mask = ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_LINE_INPUT;
    termios.mode = termios.mode | !mask;
    _set_terminal_attr(&termios)?;
    Ok(())
}

pub fn _disable_raw() -> Result<()> {
    let mut termios = _get_terminal_attr()?;
    let mask = ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_LINE_INPUT;
    termios.mode = termios.mode | mask;
    _set_terminal_attr(&termios)?;
    Ok(())
}

// Reason for &u32 is only to mirror the &Termios from unix
pub fn _set_terminal_attr(termios: &Termios) -> Result<()> {
    // (imdaveho) NOTE: shouldn't this be conout?
    let handle = Handle::stdout()?;
    unsafe {
        if !(SetConsoleMode(handle.0, termios.mode) == 0) {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn _get_terminal_attr() -> Result<Termios> {
    let mut mode = 0;
    // (imdaveho) NOTE: shouldn't this be conout?
    let handle = Handle::stdout()?;
    unsafe {
        if !(GetConsoleMode(handle.0, &mut mode) == 0 ) {
            return Err(Error::last_os_error());
        }
    }
    Ok(Termios{mode: mode, color: 0})
}