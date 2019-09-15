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
use std::io::{Error, Result};
use std::ops::BitOr;

use crate::csi;
use crate::tty::Termios;

pub mod ansi;

mod color;
pub use color::Color;

#[cfg(windows)]
use color::{Foreground, Background, RESET, IGNORE};

#[cfg(windows)]
use crate::tty::{Handle, ConsoleInfo};

#[cfg(windows)]
pub mod wincon;
pub use wincon::ConsoleOutput;

pub enum Style {
    Fg(Color),
    Bg(Color),
    Fx(u32),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Effect {
    Reset = 1 << (0 + 9),
    Bold = 1 << (1 + 9),
    Dim = 1 << (2 + 9),
    Underline = 1 << (4 + 9),
    Reverse = 1 << (7 + 9),
    Hide = 1 << (8 + 9),
}

impl BitOr for Effect {
    type Output = u32;

    fn bitor(self, rhs: Self) -> u32 {
        self as u32 | rhs as u32
    }
}

impl BitOr<Effect> for u32 {
    type Output = Self;

    fn bitor(self, rhs: Effect) -> Self {
        self as u32 | rhs as u32
    }
}


// #[derive(Clone, Copy, PartialEq)]
// pub enum Color {
//     Reset,
//     Black,
//     DarkGrey,
//     Red,
//     DarkRed,
//     Green,
//     DarkGreen,
//     Yellow,
//     DarkYellow,
//     Blue,
//     DarkBlue,
//     Magenta,
//     DarkMagenta,
//     Cyan,
//     DarkCyan,
//     White,
//     Grey,
//     Rgb {
//         r: u8,
//         g: u8,
//         b: u8,
//     },
//     AnsiValue(u8),
// }