//! Handles the printing to the visible part
//! of the TTY. It also handles styling and
//! enabling/disabling raw modes.
// use std::io::Result;
use std::str::FromStr;
use std::io::{Error, Result};
use crate::shared::{TtyResult, TtyErrorKind};

use crate::Termios;

#[cfg(windows)]
use crate::shared::{Handle, ConsoleInfo};

#[cfg(unix)]
mod linux;

#[cfg(windows)]
mod windows;


// (imdaveho) TODO: wrap the children functions into a single interface
// (see below) -- this also is needed so that windows and linux are called
// via single module
// pub fn enable_raw() -> Result<()> {
//     #[cfg(unix)]
//     linux::_enable_raw()
// }

// (imdaveho) TODO: implement a paint() or set_cell() function that effectively
// takes in a value like `"red"` or `3` and applies it to fg, bg, and/or attrs
// and resets appropriately.
// This will be what TimonPost tried to do with ObjectStyle and the .on(),
// .with(), and .attr() methods and the def_color! macros...
// but this will be a simple function that does the same thing without the
// stupid complexity or trying to be too clever...
// NOTE: tuitty is simply trying to expose the primitives and wrap it in a
// simple API -- syntactical sugar is left up to the implementer

// (imdaveho) TODO: remove the write_cout! macro and separate out the flush()
// call...

// (imdaveho) TODO: because disable leverages _set_terminal_attr to
// the original term mode, which would be in global TtyState, this
// would be better wrapped and exposed at the top level to keep modules
// as straight up functions.
// pub fn disable_raw(ios: &Termios) -> Result<()> {
//     _disable_raw(ios)
// }


// Enum with the different colors to color your test and terminal.
// #[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[derive(PartialEq)]
pub enum Color {
    // This resets the color.
    Reset,

    Black,
    DarkGrey,

    Red,
    DarkRed,

    Green,
    DarkGreen,

    Yellow,
    DarkYellow,

    Blue,
    DarkBlue,

    Magenta,
    DarkMagenta,

    Cyan,
    DarkCyan,

    White,
    Grey,
    /// Color representing RGB-colors;
    /// r = red
    /// g = green
    /// b = blue
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    AnsiValue(u8),
}

// in order to do something like this: `Color::from("blue")`
impl From<&str> for Color {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

/// impl FromStr so that From for Color can implicitly obtain the .parse() method
impl FromStr for Color {
    type Err = ();

    fn from_str(src: &str) -> ::std::result::Result<Self, Self::Err> {
        match src.as_ref() {
            "black" => Ok(Color::Black),
            "dark_grey" => Ok(Color::DarkGrey),
            "red" => Ok(Color::Red),
            "dark_red" => Ok(Color::DarkRed),
            "green" => Ok(Color::Green),
            "dark_green" => Ok(Color::DarkGreen),
            "yellow" => Ok(Color::Yellow),
            "dark_yellow" => Ok(Color::DarkYellow),
            "blue" => Ok(Color::Blue),
            "dark_blue" => Ok(Color::DarkBlue),
            "magenta" => Ok(Color::Magenta),
            "dark_magenta" => Ok(Color::DarkMagenta),
            "cyan" => Ok(Color::Cyan),
            "dark_cyan" => Ok(Color::DarkCyan),
            "white" => Ok(Color::White),
            "grey" => Ok(Color::Grey),
            _ => Ok(Color::White),
        }
    }
}


pub fn enable_raw() -> Result<()> {
    #[cfg(unix)] {
        linux::_enable_raw()
    }

    #[cfg(windows)] {
        windows::_enable_raw()
    }
}

pub fn get_mode() -> Result<Termios> {
    #[cfg(unix)] {
        linux::_get_terminal_attr()
    }

    #[cfg(windows)] {
        windows::_get_terminal_attr()
    }
}

#[cfg(unix)]
pub fn set_mode(termios: &Termios) -> Result<()> {
    linux::_set_terminal_attr(termios)
}

#[cfg(windows)]
pub fn disable_raw() -> Result<()> {
    windows::_disable_raw()
}