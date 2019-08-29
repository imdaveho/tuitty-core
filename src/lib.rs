//! (imdaveho) TODO: ...
mod screen;
mod cursor;
mod output;
mod input;


#[cfg(unix)]
use libc::termios as Termios;

#[cfg(windows)]
pub type Termios = u32;


#[cfg(unix)]
use input::ansi::{AsyncReader, SyncReader};

#[cfg(windows)]
use input::wincon::{AsyncReader, SyncReader};


mod tty;

#[cfg(windows)]
use tty::{Handle, ConsoleInfo};




// pub struct Tui(Tty);


// #[no_mangle]
// pub extern fn tuitty() -> *mut Tui {
//     Box::into_raw(Box::new(Tui::init()))
// }

// // TODO: impl struct in system modules...
// impl Tui {
//     pub fn init() -> Tui {
//         Tui(Tty::init())
//     }
// }
