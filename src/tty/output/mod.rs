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
use crate::csi;
use crate::tty::Termios;

pub mod ansi;
pub use ansi::{Color, Effect, Effects};

#[cfg(windows)]
use crate::tty::{Handle, ConsoleInfo};

#[cfg(windows)]
pub mod wincon;


pub enum Style {
    Fg(Color),
    Bg(Color),
    Fx(Effects),
}
