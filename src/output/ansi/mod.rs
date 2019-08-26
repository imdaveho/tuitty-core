// ANSI functions for writing and styling text to be outputted to the terminal.

use std::io::Write;
use super::{
    csi, write_cout, Color, Display, Error,
    Result, Termios, TextStyle, Style
};


mod raw;
pub use raw::*;

mod style;
pub use style::*;


pub fn writeout<D: Display>(value: D) -> Result<usize> {
    write_cout!(format!("{}", value))?;
    Ok(0)
}
