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
            ENABLE_ECHO_INPUT,
            ENABLE_PROCESSED_INPUT,
            ENABLE_VIRTUAL_TERMINAL_PROCESSING as ENABLE_VT,
            WriteConsoleOutputW
        }
    },
    shared::ntdef::{ NULL, VOID }
};
use super::handle::Handle;

pub use winapi::um::wincon::{ CHAR_INFO, COORD, SMALL_RECT };


pub fn prints(content: &str, conout: &Handle) -> Result<()> {
    let text: Vec<u16> = content.encode_utf16().collect();
    let mut size = 0;
    unsafe {
        // https://docs.microsoft.com/en-us/windows/console/writeconsole
        if WriteConsoleW(
            conout.0,
            text.as_ptr() as *const VOID,
            text.len() as u32,
            &mut size, NULL
        ) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn writebuf(
    buffer: *const CHAR_INFO, size: COORD, 
    coord: COORD, dest: &mut SMALL_RECT,
    conout: &Handle
) -> Result<()> {
    unsafe {
        if WriteConsoleOutputW(conout.0, buffer, size, coord, dest) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn enable_raw(conin: &Handle) -> Result<()> {
    let mode = conin.get_mode()?;
    let mask = ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT;
    let raw_mode = mode & !mask;
    conin.set_mode(&raw_mode)?;
    Ok(())
}

pub fn disable_raw(conin: &Handle) -> Result<()> {
    let mode = conin.get_mode()?;
    let mask = ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT;
    let cooked_mode = mode | mask;
    conin.set_mode(&cooked_mode)?;
    Ok(())
}


pub fn get_mode() -> Result<u32> {
    // Stdout because if you're creating a
    // new screen via alternate screen, you
    // want a default set of terminal settings
    let handle = Handle::stdout()?;
    let mode = handle.get_mode();
    mode
}


pub fn is_ansi_enabled() -> bool {
    const TERMS: [&'static str; 15] = [
        "xterm",  // xterm, PuTTY, Mintty
        "rxvt",   // RXVT
        "eterm",  // Eterm
        "screen", // GNU screen, tmux
        "tmux",   // tmux
        "vt100", "vt102", "vt220", "vt320",   // DEC VT series
        "ansi",    // ANSI
        "scoansi", // SCO ANSI
        "cygwin",  // Cygwin, MinGW
        "linux",   // Linux console
        "konsole", // Konsole
        "bvterm",  // Bitvise SSH Client
    ];

    let matched_terms = match std::env::var("TERM") {
        Ok(val) => val != "dumb" || TERMS.contains(&val.as_str()),
        Err(_) => false,
    };

    if matched_terms {
        return true
    } else {
        let handle = match Handle::stdout() {
            Ok(h) => h,
            Err(_) => return false,
        };
        let mode = match handle.get_mode() {
            Ok(m) => m,
            Err(_) => return false,
        };
        match handle.set_mode(&(mode | ENABLE_VT)) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}