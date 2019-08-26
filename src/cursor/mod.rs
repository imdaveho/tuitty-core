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

// pub use ansi::{
//     _goto as goto,
//     _hide as hide,
//     _show as show,
//     _move_up as move_up,
//     _move_down as move_down,
//     _move_left as move_left,
//     _move_right as move_right,
//     // Differences with wincon:
//     _pos_raw as pos_raw,
//     _save_pos as save_pos,
//     _load_pos as load_pos,
// };

// #[cfg(windows)]
// pub use console::{
//     _goto as goto,
//     _move_up as move_up,
//     _move_down as move_down,
//     _move_left as move_left,
//     _move_right as move_right,
//     _hide as hide,
//     _show as show,
//     // Differences with wincon:
//     _pos as pos,
// };
