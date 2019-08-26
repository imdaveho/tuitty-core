// ANSI functions for enabling and disabling raw mode.
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
use libc::c_int;
use super::{Error, Result, Termios};


pub fn get_mode() -> Result<Termios> {
    extern "C" {
        pub fn tcgetattr(fd: c_int, termpt: *mut Termios) -> c_int;
    }
    unsafe {
        let mut termios = mem::zeroed();
        if tcgetattr(0, &mut termios) == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(termios)
        }
    }
}

/// This function enables raw mode in the current screen.
pub fn enable_raw() -> Result<()> {
    extern "C" {
        pub fn cfmakeraw(termpt: *mut Termios);
    }

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
    extern "C" {
        pub fn tcsetattr(fd: c_int, opt: c_int, termpt: *const Termios) -> c_int;
    }
    if unsafe { tcsetattr(0, 0, termios) } == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}
