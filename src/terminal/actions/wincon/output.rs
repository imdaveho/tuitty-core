// Windows Console API specific functions to print to the terminal.
// and for enabling and disabling raw mode.
//
// Normally the terminal uses line buffering. This means input will be sent
// line by line. With raw mode, input is sent one byte at a time. Because of
// this, all input needs to be outputted manually. Characters are not processed
// by the system, rather it is sent straight through. For example, a `backspace`
// is not interpreted as a removing a character one space to the left, instead
// the byte representation of `backspace` is sent for the user or program to
// handle.
//
// Also, escape characters like `\n` and `\r` will move the cursor to a new line
// but will be in the same position as it was in the previous line. It is up to
// the user or program to move it to where they would like the cursor to be.

use std::io::{Error, Result};
use winapi::{
    um::{
        consoleapi::WriteConsoleW,
        wincon::{
            ENABLE_LINE_INPUT,
            ENABLE_WRAP_AT_EOL_OUTPUT
        }
    },
    shared::ntdef::{ NULL, VOID }
};
use super::handle::Handle;


pub fn prints(content: &str) -> Result<()> {
    let handle = Handle::conout()?;
    let text = format!("{}", content).as_str()
        .encode_utf16()
        .map(|x| x)
        .collect::<Vec<u16>>();
    let mut size = 0;
    unsafe {
        // https://docs.microsoft.com/en-us/windows/console/writeconsole
        if WriteConsoleW(
            handle.0,
            text.as_ptr() as *const VOID,
            text.len() as u32,
            &mut size, NULL
        ) == 0 {
            return Err(Error::last_os_error());
        }
    }
    handle.close()?;
    Ok(())
}


pub fn get_mode() -> Result<u32> {
    // Stdout because if you're creating a
    // new screen via alternate screen, you
    // want a default set of terminal settings
    let handle = Handle::stdout()?;
    let mode = handle.get_mode();
    handle.close()?;
    mode
}


pub fn enable_raw() -> Result<()> {
    let handle = Handle::conout()?;
    let mode = handle.get_mode()?;
    let mask = ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_LINE_INPUT;
    let raw_mode = mode & !mask;
    handle.set_mode(&raw_mode)?;
    handle.close()?;
    Ok(())
}

pub fn disable_raw() -> Result<()> {
    let handle = Handle::conout()?;
    let mode = handle.get_mode()?;
    let mask = ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_LINE_INPUT;
    let cooked_mode = mode | mask;
    handle.set_mode(&cooked_mode)?;
    handle.close()?;
    Ok(())
}
