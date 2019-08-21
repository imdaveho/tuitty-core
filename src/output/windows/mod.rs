//! Platform specific functions for the library.
use super::{
    Color, Error, Result, Termios, 
    Handle, ConsoleInfo, FromStr, Style,
};
use std::fmt::Display;
use winapi::um::consoleapi::WriteConsoleW;
use winapi::shared::ntdef::{NULL, VOID};

mod raw;
pub use raw::*;

mod style;
pub use style::*;


pub fn _write<D: Display>(value: D) -> Result<usize> {
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


#[derive(PartialEq)]
pub enum TextStyle {
    Reset,
    Bold,
    Dim,
    Underline,
    Reverse,
    Hide,
}

impl From<&str> for TextStyle {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(TextStyle::Reset)
    }
}

impl FromStr for TextStyle {
    type Err = ();
    fn from_str(src: &str) -> ::std::result::Result<Self, Self::Err> {
        match src.as_ref() {
            "bold" => Ok(TextStyle::Bold),
            "dim" => Ok(TextStyle::Dim),
            "underline" => Ok(TextStyle::Underline), 
            "reverse" => Ok(TextStyle::Reverse),
            "hide" => Ok(TextStyle::Hide),
            "reset" => Ok(TextStyle::Reset),
            _ => Ok(TextStyle::Reset),
        }
    }
}
