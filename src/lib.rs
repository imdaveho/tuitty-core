//! (imdaveho) TODO: ...
mod screen;
mod cursor;
mod output;
mod input;
mod shared;


#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::*;

#[cfg(windows)]
pub use shared::{Handle, ConsoleInfo};

mod ansi;
pub use ansi::*;

// use input::ansi::{AsyncAnsiReader, SyncAnsiReader};
// TODO:
// #[cfg(windows)]
// use input::{AsyncAnsiReader, SyncAnsiReader};




pub struct Tui(Tty);

#[no_mangle]
pub extern fn tuitty() -> *mut Tui {
    Box::into_raw(Box::new(Tui::init()))
}

// TODO: impl struct in system modules...
impl Tui {
    pub fn init() -> Tui {
        Tui(Tty::init())
    }
}
