//! `tuitty` is a cross platform library that is meant for FFI.

mod tty;
pub use tty::Tty;


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
