// Shared code and abstractions that is leveraged by the other modules.
// * `ansi`, a simple macro to help with writing escape sequences to stdout.
// * `wincon`, wrappers for pointers to the Handle and ConsoleInfo objects.
// * `wcwidth`, clone of unicode-width returning how many cells a char occupies.
use super::screen::Clear;
use super::output::{Color, Effect, Effects, Style};


mod ansi;
pub use ansi::{ansi_write, ansi_flush};

#[cfg(windows)]
mod wincon;

#[cfg(windows)]
pub use wincon::{Handle, ConsoleInfo};

mod cell;
pub use cell::{CellBuffer, Cell, CellStyle};

mod wcwidth;
use wcwidth::{UnicodeWidthChar, UnicodeWidthStr};
