// use std::mem;
// use libc::c_int;
// use std::io::Error; // TODO: move to shared? rename shared to platform
// use shared::Termios;


// pub fn get_mode() -> Result<Termios> {
//     extern "C" {
//         pub fn tcgetattr(fd: c_int, termpt: *mut Termios) -> c_int;
//     }
//     unsafe {
//         let mut termios = mem::zeroed();
//         if tcgetattr(0, &mut termios) == -1 {
//             Err(Error::last_os_error())
//         } else {
//             Ok(termios)
//         }
//     }
// }

// pub fn set_mode(termios: &Termios) -> Result<()> {
//     extern "C" {
//         pub fn tcsetattr(fd: c_int, opt: c_int, termpt: *const Termios) -> c_int;
//     }
//     if unsafe { tcsetattr(0, 0, termios) } == -1 {
//         Err(Error::last_os_error())
//     } else {
//         Ok(())
//     }
// }