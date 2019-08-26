// Windows Console API functions for enabling and disabling raw mode.
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

use winapi::um::wincon::{
    ENABLE_LINE_INPUT, ENABLE_WRAP_AT_EOL_OUTPUT
};
use super::{Handle, Result, Termios};


pub fn get_mode() -> Result<Termios> {
    // Stdout because if you're creating a
    // new screen via alternate screen, you
    // want a default set of terminal settings
    Handle::stdout().unwrap().get_mode()
}

pub fn enable_raw() -> Result<()> {
    let handle = Handle::conout()?;
    let mode = handle.get_mode()?;
    let mask = ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_LINE_INPUT;
    let raw_mode = mode & !mask;
    handle.set_mode(&raw_mode)?;
    Ok(())
}

pub fn disable_raw() -> Result<()> {
    let handle = Handle::conout()?;
    let mode = handle.get_mode()?;
    let mask = ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_LINE_INPUT;
    let cooked_mode = mode | mask;
    handle.set_mode(&cooked_mode)?;
    Ok(())
}
