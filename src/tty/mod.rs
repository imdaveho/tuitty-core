//! The `tty` module wraps the various components that make up a terminal. These
//! are represented by the sub-modules: `cursor`, `screen`, `input`, `output`.
//! The `Tty` struct is meant to be a thin abstraction to standardize between
//! operating systems and APIs (ANSI vs Windows Console).

#[cfg(unix)]
use libc::termios as Termios;

#[cfg(windows)]
type Termios = u32;

mod cursor;
mod input;
mod output;
mod screen;
mod shared;

#[cfg(windows)]
pub use shared::{Handle, ConsoleInfo};

pub use output::{Color, Effect};

#[cfg(windows)]
pub use output::ConsoleOutput;

pub use input::{InputEvent, KeyEvent, MouseEvent, MouseButton};

#[cfg(unix)]
pub use input::ansi::{AsyncReader, SyncReader};

#[cfg(windows)]
pub use input::wincon::{AsyncReader, SyncReader};

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::Tty;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::Tty;

#[cfg(test)]
mod tests;


trait Teletype {
    fn init();
    fn terminate();
    fn manual();
    fn automatic();
    fn screen_pos(); // cursor::pos
    fn screen_size(); // screen::size
    // screen
    fn clear();
    fn resize();
    // cursor
    fn goto();
    fn up();
    fn down();
    fn left();
    fn right();
    fn moves();
    fn mark_pos();
    fn load_pos();
    fn hide_cursor();
    fn show_cursor();
    // style
    fn set_fg(); // Style
    fn set_bg(); // STyle
    fn set_fx();
    fn set_styles(); // Color, u32
    fn reset_styles();
    // output
    fn prints();
    fn flush();
    fn printf();
}