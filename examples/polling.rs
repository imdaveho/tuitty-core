#[cfg(windows)]
fn main() {
    use tuitty_core::actions::wincon::{ output, mouse, handle };
    use tuitty_core::parser::windows;
    use tuitty_core::common::enums::{
        InputEvent, KeyEvent::*,
        MouseEvent::*, MouseButton
    };

    let mode = output::get_mode().unwrap();
    let conout = handle::Handle::conout().unwrap();
    let conin = hande::Handle::conin().unwrap();
    output::enable_raw(&conout).unwrap();
    mouse::enable_mouse_mode(&conin).unwrap();
    loop {
        let (i, evts) = windows::read_input_events();
        println!("evt#: {}\r", i);
        for evt in evts {
            match evt {
                InputEvent::Unsupported => (),
                InputEvent::Keyboard(k) => match k {
                    Char(c) => println!("Char({})\r", c),
                    Ctrl(c) => {
                        println!("Ctrl({})\r", c);
                        if c == 'q' { break }
                    },
                    Alt(c) => println!("Alt({})\r", c),
                    _ => ()
                },
                InputEvent::Mouse(m) => match m {
                    Press(btn, x, y) => match btn {
                        MouseButton::Left => println!("Left({}, {})\r", x, y),
                        MouseButton::Right => println!("Right({}, {})\r", x, y),
                        MouseButton::Middle => println!("Middle({}, {})\r", x, y),
                        MouseButton::WheelUp => println!("WheelUp({}, {})\r", x, y),
                        MouseButton::WheelDown => println!("WheelDown({}, {})\r", x, y),
                    },
                    _ => ()
                },
                InputEvent::WinResize(_, _) => println!("Resize\r"),
                _ => (),
            }
        }
    }
    output::disable_raw(&conout).unwrap();
    mouse::disable_mouse_mode(&conin).unwrap();
}


#[cfg(unix)]
fn main() {
    use tuitty_core::actions::ansi::{ output, mouse };
    use tuitty_core::parser::unix::UnixHandle;
    use tuitty_core::common::enums::{
        InputEvent, KeyEvent::*,
        MouseEvent::*, MouseButton
    };

    let mut stdinn = UnixHandle::new().unwrap();
    let mode = output::get_mode().unwrap();
    output::enable_raw().unwrap();
    output::printf(&mouse::enable_mouse_mode()).unwrap();
    loop {
        let inn = stdinn.read_input_events();
        match inn {
            InputEvent::Unsupported => (),
            InputEvent::Keyboard(k) => match k {
                Char(c) => println!("Char({})\r", c),
                Ctrl(c) => {
                    println!("Ctrl({})\r", c);
                    if c == 'q' { break }
                },
                Alt(c) => println!("Alt({})\r", c),
                _ => ()
            },
            InputEvent::Mouse(m) => match m {
                Press(btn, x, y) => match btn {
                    MouseButton::Left => println!("Left({}, {})\r", x, y),
                    MouseButton::Right => println!("Right({}, {})\r", x, y),
                    MouseButton::Middle => println!("Middle({}, {})\r", x, y),
                    MouseButton::WheelUp => println!("WheelUp({}, {})\r", x, y),
                    MouseButton::WheelDown => println!("WheelDown({}, {})\r", x, y),
                },
                _ => ()
            },
            InputEvent::WinResize(_, _) => println!("Resize\r"),
            _ => (),
        }
    }
    output::set_mode(&mode).unwrap();
    output::printf(&mouse::disable_mouse_mode()).unwrap();
}



// extern crate mio;


// use mio::{ unix::SourceFd, Events, Interest, Poll, Token };
// use std::{ fs, io, os::unix::io::{ IntoRawFd, RawFd }};
// use std::io::Result;
// use std::time::Duration;
// use tuitty_core::actions::ansi::{ output, mouse };


// #[derive(Debug)]
// struct FileDesc {
//     fd: RawFd,
//     close_on_drop: bool,
// }

// impl FileDesc {
//     fn new(fd: RawFd, close_on_drop: bool) -> Self {
//         Self { fd, close_on_drop }
//     }

//     fn read(&self, buffer: &mut [u8], size: usize) -> Result<usize> {
//         let result = unsafe {
//             libc::read(
//                 self.fd,
//                 buffer.as_mut_ptr() as *mut libc::c_void,
//                 size as libc::size_t,
//             ) as isize
//         };

//         if result < 0 {
//             // Err(ErrorKind::IoError(io::Error::last_os_error()))
//             Err(io::Error::last_os_error())
//         } else {
//             Ok(result as usize)
//         }
//     }

//     fn raw_fd(&self) -> RawFd {
//         self.fd
//     }
// }

// impl Drop for FileDesc {
//     fn drop(&mut self) {
//         if self.close_on_drop {
//             let _ = unsafe { libc::close(self.fd) };
//         }
//     }
// }

// fn tty_fd() -> Result<FileDesc> {
//     if unsafe { libc::isatty(libc::STDIN_FILENO) == 1 } {
//         Ok(FileDesc::new(libc::STDIN_FILENO, false))
//     } else {
//         Ok(FileDesc::new(
//             fs::OpenOptions::new()
//                 .read(true)
//                 .write(true)
//                 .open("/dev/tty")?
//             .into_raw_fd(), true
//         ))
//     }
// }


// const TTY_TOKEN: Token = Token(0);
// const LEN: usize = 12;

// struct UnixSource {
//     poll: Poll,
//     events: Events,
//     tty_buffer: [u8; LEN],
//     tty_fd: FileDesc,
// }

// impl UnixSource {
//     fn new() -> Result<Self> {
//         let poll = Poll::new()?;
//         let registry = poll.registry();

//         let input_fd = tty_fd()?;
//         let tty_raw_fd = input_fd.raw_fd();
//         let mut tty_ev = SourceFd(&tty_raw_fd);
//         registry.register(&mut tty_ev, TTY_TOKEN, Interest::READABLE)?;

//         Ok(Self {
//             poll,
//             events: Events::with_capacity(2),
//             tty_buffer: [0u8; LEN],
//             tty_fd: input_fd,
//         })
//     }

//     // fn read(&mut self) -> Result<[u8; 12]> {
//     fn read(&mut self) -> Result<&[u8]> {
//         let timeout = Some(Duration::from_millis(0));
//         // loop {
//             if let Err(e) = self.poll.poll(&mut self.events, timeout) {
//                 if e.kind() == io::ErrorKind::Interrupted {
//                     // continue;
//                     return Ok(&[0]);
//                 } else {
//                     return Err(e);
//                 }
//             };

//             if self.events.is_empty() {
//                 // return Ok([0; 12]);
//                 return Ok(&[0]);
//             }

//             for event in self.events.iter() {
//                 match event.token() {
//                     TTY_TOKEN => {
//                         loop {
//                             // self.tty_buffer = [0; LEN];
//                             match self.tty_fd.read(&mut self.tty_buffer, LEN) {
//                                 Ok(read_count) => {
//                                     if read_count > 0 {
//                                         // println!("rc: {}", read_count);
//                                         return Ok(&self.tty_buffer[..read_count])
//                                     }
//                                 },
//                                 // Err(ErrorKind::IoError(e)) => {
//                                 //     if e.kind() == io::ErrorKind::WouldBlock {
//                                 //         break;
//                                 //     }
//                                 //     else if e.kind() == io::ErrorKind::Interrupted {
//                                 //         continue;
//                                 //     }
//                                 // },
//                                 Err(e) => return Err(e)
//                             };
//                         }
//                     },
//                     _ => unreachable!("Unknown token detected")
//                 }
//             }
//         Ok(&[0])
//         }
//     // }
// }


// fn main() {

//     let mut stdinn = UnixSource::new().unwrap();
//     let mut count = 0;
//     let mode = output::get_mode().unwrap();
//     output::enable_raw().unwrap();
//     output::printf(&mouse::enable_mouse_mode()).unwrap();
//     // NOTE: looping example
//     loop {
//         let inn = stdinn.read().unwrap();
//         // if inn != [0; LEN] {
//         if inn != &[0] {
//             println!("{:?}\r", inn);
//             count += 1;
//         }
//         if count > 13 { break; }
//     }

//     // NOTE: poll w/ 1 sec wait
//     // let inn = stdinn.read().unwrap();
//     // println!("{:?}\r", inn);

//     output::set_mode(&mode).unwrap();
//     output::printf(&mouse::disable_mouse_mode()).unwrap();
// }
