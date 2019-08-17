//! Platform specific functions for the library.
use std::io::Write;
use std::fmt::Display;
use super::{Color, TtyResult, Result, Termios, Style};


mod raw;
pub use raw::*;

mod style;
pub use style::*;


#[derive(Copy, Clone, PartialEq)]
pub enum TextStyle {
    Reset = 0,
    Bold = 1,
    Dim = 2,
    Underline = 4,
    Reverse = 7,
    Hide = 8,
}

impl Display for TextStyle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", *self as u16)
    }
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
            "bold" => Ok(Attribute::Bold),
            "dim" => Ok(Attribute::Dim),
            "underline" => Ok(Attribute::Underline), 
            "reverse" => Ok(Attribute::Reverse),
            "hide" => Ok(Attribute::Hide),
            "reset" => Ok(Attribute::Reset),
            _ => Ok(Attribute::Reset),
        }
    }
}