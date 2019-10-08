// Unix specific functions for enabling and disabling raw mode.
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

use std::mem;
use std::io::{Error, Result};
use libc::termios as Termios;
use libc::{cfmakeraw, tcgetattr, tcsetattr, STDIN_FILENO, TCSANOW};


pub fn get_mode() -> Result<Termios> {
    unsafe {
        let mut termios = mem::zeroed();
        if tcgetattr(STDIN_FILENO, &mut termios) == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(termios)
        }
    }
}

/// This function enables raw mode in the current screen.
pub fn enable_raw() -> Result<()> {
    unsafe {
        // Get the current terminal attrs.
        let mut termios = get_mode()?;
        // Apply the raw attr to the current terminal attrs.
        // There is no effect until a subsequent call to tcsetattr().
        // https://www.mkssoftware.com/docs/man3/cfmakeraw.3.asp
        cfmakeraw(&mut termios);
        // Set the current terminal with raw-enabled attrs.
        // unwrap(tcsetattr(0, 0, &termios)).and(Ok(()))
        set_mode(&termios)?;
        Ok(())
    }
}

pub fn set_mode(termios: &Termios) -> Result<()> {
    if unsafe { tcsetattr(STDIN_FILENO, TCSANOW, termios) } == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}