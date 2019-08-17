//! Platform specific functions for the library.
use super::{
    Color, Error, Result, Termios, 
    Handle, ConsoleInfo, FromStr, Style,
};

mod raw;
pub use raw::*;

mod style;
pub use style::*;


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