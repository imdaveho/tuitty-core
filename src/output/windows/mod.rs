//! Platform specific functions for the library.
use super::{
    Color, TtyResult, TtyErrorKind, Error, Result, 
    Termios, Handle, ConsoleInfo
};

mod raw;
pub use raw::*;

mod style;
pub use style::*;