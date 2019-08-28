// Windows Console API functions for writing and styling text to the console.

use winapi::um::consoleapi::WriteConsoleW;
use winapi::shared::ntdef::{NULL, VOID};
use super::{
    Display, Error, Result, Color, Style,
    Handle, ConsoleInfo, Termios, TextStyle
};

#[cfg(windows)]
mod raw;

#[cfg(windows)]
pub use raw::*;

mod style;
pub use style::*;


pub fn writeout<D: Display>(value: D) -> Result<usize> {
    let handle = Handle::conout()?;
    let text = format!("{}", value).as_str()
        .encode_utf16()
        .map(|x| x)
        .collect::<Vec<u16>>();
    let length = text.len() as u32;
    let mut size = 0;
    unsafe {
        // https://docs.microsoft.com/en-us/windows/console/writeconsole
        if WriteConsoleW(
            handle.0,
            text.as_ptr() as *const VOID,
            length, 
            &mut size, NULL
        ) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(size as usize)
}
