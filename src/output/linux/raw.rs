//! Implements platform specific method to convert output into raw mode.
use std::mem;
use std::io:: Error;
use libc::c_int;
use super::{Result, Termios};


pub fn _get_terminal_attr() -> Result<Termios> {
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
pub fn _enable_raw() -> Result<()> {
    extern "C" {
        pub fn cfmakeraw(termpt: *mut Termios);
    }

    // fn unwrap(t: i32) -> Result<()> {
    //     if t == -1 {
    //         Err(Error::last_os_error())
    //     } else {
    //         Ok(())
    //     }
    // }

    unsafe {
        // Get the current terminal attrs.
        let mut termios = _get_terminal_attr()?;
        // Apply the raw attr to the current terminal attrs.
        // There is no effect until a subsequent call to tcsetattr().
        // https://www.mkssoftware.com/docs/man3/cfmakeraw.3.asp
        cfmakeraw(&mut termios);
        // Set the current terminal with raw-enabled attrs.
        // unwrap(tcsetattr(0, 0, &termios)).and(Ok(()))
        _set_terminal_attr(&termios)?;
        Ok(())
    }
}

pub fn _set_terminal_attr(termios: &Termios) -> Result<()> {
    extern "C" {
        pub fn tcsetattr(fd: c_int, opt: c_int, termpt: *const Termios) -> c_int;
    }
    if unsafe { tcsetattr(0, 0, termios) } == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}