//! # Cursor
//!
//! The `cursor` module provides a standardized way to control the position,
//! style, and visibility of the terminal cursor in both ANSI-supported
//! terminals and classic Windows consoles.
//!
//! Note that the differences between the ANSI vs the WinCon functionalities are
//! mainly between the way we fetch the current cursor position, which needs to
//! enable "raw mode" on ANSI-based terminals, and how we "bookmark" a cursor
//! location that a user can return to on demand, which isn't native to the Win
//! dows console.

pub mod ansi;

#[cfg(windows)]
pub mod wincon;
