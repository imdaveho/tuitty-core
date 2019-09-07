//! # Output
//!
//! The `output` module contains the functions that will influence how and what
//! gets displayed into the screen on both ANSI-supported terminals and Windows
//! consoles. The module handles writing to the current output buffer, styling
//! the foreground and background colors, setting text styles (eg. underline),
//! and toggling raw mode.
//!
//! The main difference between the ANSI and WinCon for this module is that the
//! `wincon` module does not export a set_mode function because this is already
//! an existing method for the `Handle` wrapper at the top level.

use std::fmt::Display;
use std::str::FromStr;
use std::io::{Error, Result};
use crate::csi;
use crate::tty::Termios;

pub mod ansi;

#[cfg(windows)]
use crate::tty::{Handle, ConsoleInfo};

#[cfg(windows)]
pub mod wincon;


enum Style {
    Fg(Color),
    Bg(Color),
    Fmt(Format),
}

// Enum with the different colors to color your test and terminal.
// #[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy, PartialEq)]
pub enum Color {
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

    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    AnsiValue(u8),
}

// In order to do something like this: `Color::from("blue")`
impl From<&str> for Color {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

// The FromStr Trait provides the .parse() method
impl FromStr for Color {
    type Err = ();
    fn from_str(src: &str) -> ::std::result::Result<Self, Self::Err> {
        match src.as_ref() {
            "black" => Ok(Color::Black),
            "dark_grey" | "darkgrey" => Ok(Color::DarkGrey),
            "red" => Ok(Color::Red),
            "dark_red" | "darkred" => Ok(Color::DarkRed),
            "green" => Ok(Color::Green),
            "dark_green" | "darkgreen" => Ok(Color::DarkGreen),
            "yellow" => Ok(Color::Yellow),
            "dark_yellow" | "darkyellow" => Ok(Color::DarkYellow),
            "blue" => Ok(Color::Blue),
            "dark_blue" | "darkblue" => Ok(Color::DarkBlue),
            "magenta" => Ok(Color::Magenta),
            "dark_magenta" | "darkmagenta" => Ok(Color::DarkMagenta),
            "cyan" => Ok(Color::Cyan),
            "dark_cyan" | "darkcyan" => Ok(Color::DarkCyan),
            "white" => Ok(Color::White),
            "grey" => Ok(Color::Grey),
            "reset" => Ok(Color::Reset),
            _ => Ok(Color::Reset),
        }
    }
}


#[derive(Clone, Copy, PartialEq)]
pub enum Format {
    Reset = 0,
    Bold = 1,
    Dim = 2,
    Underline = 4,
    Reverse = 7,
    Hide = 8,
}

impl Display for Format {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl From<&str> for Format {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Format::Reset)
    }
}

impl From<u8> for Format {
    fn from(src: u8) -> Self {
        match src {
            0 => Format::Reset,
            1 => Format::Bold,
            2 => Format::Dim,
            4 => Format::Underline,
            7 => Format::Reverse,
            8 => Format::Hide,
            _ => Format::Reset,
        }
    }
}

impl FromStr for Format {
    type Err = ();
    fn from_str(src: &str) -> ::std::result::Result<Self, Self::Err> {
        match src.as_ref() {
            "bold" => Ok(Format::Bold),
            "dim" => Ok(Format::Dim),
            "underline" => Ok(Format::Underline),
            "reverse" => Ok(Format::Reverse),
            "hide" => Ok(Format::Hide),
            _ => Ok(Format::Reset),
        }
    }
}
