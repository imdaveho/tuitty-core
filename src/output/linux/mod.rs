//! Platform specific functions for the library.
use std::io::Write;
use std::fmt::Display;
use super::{Color, TtyResult, Result, Termios};


mod raw;
pub use raw::*;

mod style;
pub use style::*;


// #[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
// Copy, Clone needed for impl Display
#[derive(Copy, Clone)]
pub enum Attribute {
    Reset = 0,
    Bold = 1,
    Dim = 2,
    Underline = 4,
    Reverse = 7,
    Hide = 8,
}

// needed so that attrs can have the .to_string() method
impl Display for Attribute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", *self as u16)
    }
}

pub enum Style {
    Fg(Color),
    Bg(Color),
    Attr(Attribute)
}
