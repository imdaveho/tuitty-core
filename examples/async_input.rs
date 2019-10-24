extern crate tuitty;

use std::{ fs, thread, time::Duration, io::{ Read, BufReader } };
use std::sync::{
    Arc, mpsc::channel,
    atomic::{ AtomicBool, Ordering }
};

// ATTEMPT 2
// #[cfg(unix)]
// use std::os::unix::io::{ IntoRawFd, FromRawFd };
// use std::io::Write;

#[cfg(unix)]
use tuitty::{
    terminal::{ dispatch::unix::parser, actions::posix },
    common::enums::{ InputEvent, KeyEvent, MouseEvent },
};


// pub struct Bytes<R> {
//     inner: R,
// }

// impl<R: Read> Iterator for Bytes<R> {
//     type Item = Result<u8>;

//     fn next(&mut self, cond: Arc<AtomicBool>) -> Option<Result<u8>> {
//         let mut byte = 0;
//         while cond.load(Ordering::SeqCst) {
//             return match self.inner.read(slice::from_mut(&mut byte)) {
//                 Ok(0) => None,
//                 Ok(..) => Some(Ok(byte)),
//                 Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
//                 Err(e) => Some(Err(e)),
//             };
//         }
//         Some(Ok(0))
//     }
// }

fn main() {
    #[cfg(unix)] {
        let is_running = Arc::new(AtomicBool::new(true));
        let is_running_arc = is_running.clone();
        let original_mode = posix::get_mode();
        posix::enable_alt();
        posix::raw();
        posix::enable_mouse();
        posix::hide_cursor();
        let (input_tx, input_rx) = channel();

        let input_handle = thread::spawn(move || {

            // ATTEMPT 2 - using FileFd to pass input later
            // Doesn't work since write..just prints out on the screen
            // Need to send EOT or some kind of process end to the thread...
            // For now...we just don't join the thread...
            // When the program exits...this should close the thread naturally.
            // let tty_fd = fs::OpenOptions::new()
            //     .read(true).write(true)
            //     .open("/dev/tty").expect("Error opening /dev/tty")
            //     .into_raw_fd();

            // while is_running_arc.load(Ordering::SeqCst) {
            //     let tty = unsafe {
            //         BufReader::new(fs::File::from_raw_fd(tty_fd))
            //     };
            //     for byte in tty.bytes() {
            //         if !is_running_arc.load(Ordering::SeqCst) { break }
            //         let b = byte.expect("Error reading byte from /dev/tty");
            //         // (imdaveho) TODO: Handle the Err state?
            //         // Previous: break out of the loop. But might
            //         // have caused weird conditions on .join() --
            //         // further observation needed.
            //         let _ = input_tx.send(b);
            //     }
            //     thread::sleep(Duration::from_millis(10));
            // }
            // ATTEMPT 1 - Works only for single byte sequences
            // while is_running_arc.load(Ordering::SeqCst) {
            //     let mut tty = BufReader::new(fs::OpenOptions::new()
            //                              .read(true).write(true).open("/dev/tty")
            //                              .expect("Error opening /dev/tty"));
            //     let mut buf: [u8; 1] = [0];
            //     let _ = tty.read_exact(&mut buf);
            //     let _ = input_tx.send(buf[0]);

            //     thread::sleep(Duration::from_millis(10));
            // }

            // ORIGINAL - Hangs at end
            while is_running_arc.load(Ordering::SeqCst) {
                let tty = BufReader::new(fs::OpenOptions::new()
                                         .read(true).write(true).open("/dev/tty")
                                         .expect("Error opening /dev/tty"));
                // for byte in tty.bytes() {
                //     if !is_running_arc.load(Ordering::SeqCst) { break }
                //     let b = byte.expect("Error reading byte from /dev/tty");
                //     // (imdaveho) TODO: Handle the Err state?
                //     // Previous: break out of the loop. But might
                //     // have caused weird conditions on .join() --
                //     // further observation needed.
                //     let _ = input_tx.send(b);
                // }
                let t = tty.bytes();
                for _b in t {
                    if !is_running_arc.load(Ordering::SeqCst) { break }
                    let _ = input_tx.send(59);
                }
                thread::sleep(Duration::from_millis(10));
            }
        });

        // let is_running_chk = is_running.clone();
        // let check_thread = thread::spawn(move || {
        //     let mut counter = 0;
        //     while is_running_chk.load(Ordering::SeqCst) {
        //         Terminal::goto(0, 5);
        //         Terminal::printf(&format!("Iter: {}", counter));
        //         thread::sleep(Duration::from_millis(100));
        //         counter += 1;
        //     }
        // });

        loop {
            let mut iterator = input_rx.try_iter();
            let evt = match iterator.next() {
                Some(ch) => {
                    let parsed_evt = parser::parse_event(
                        ch, &mut iterator);
                    if let Ok(evt) = parsed_evt {
                        Some(evt)
                    } else { None }
                }
                None => None,
            };
            match evt {
                Some(ev) => match ev {
                    InputEvent::Keyboard(kv) => match kv {
                        KeyEvent::Char(c) => {
                            if c == ';' {
                                is_running.store(false, Ordering::SeqCst);
                                break
                            }
                            posix::goto(0, 0);
                            posix::printf(&format!("Event: Char({}){:<16}", c, " "));
                        },
                        KeyEvent::Ctrl(c) => {
                            posix::goto(0, 0);
                        posix::printf(&format!("Event: Ctrl({}){:<16}", c, " "));
                        },
                        KeyEvent::Alt(c) => {
                            posix::goto(0, 0);
                            posix::printf(&format!("Event: Alt({}){:<16}", c, " "));
                        },
                        KeyEvent::CtrlLeft => {
                            posix::goto(0, 0);
                            posix::printf(&format!("Event: CtrlLeft{:<16}", " "));
                        },
                        KeyEvent::CtrlRight => {
                            posix::goto(0, 0);
                            posix::printf(&format!("Event: CtrlRight{:<16}", " "));
                        },
                        _ => (),
                    },
                    InputEvent::Mouse(mv) => match mv {
                        MouseEvent::Press(_, col, row) => {
                            posix::goto(0, 0);
                            posix::printf(&format!("Event: MousePress({},{}){:<16}", col, row, " "));
                        },
                        _ => (),
                    },
                    _ => (),
                },
                None => (),
            }
            thread::sleep(Duration::from_millis(10));
        }

        input_handle.join().expect("Error joining");
        // check_thread.join().expect("Error joining check_thread");

        posix::show_cursor();
        posix::disable_mouse();
        posix::cook(&original_mode);
        posix::disable_alt();

        thread::sleep(Duration::from_secs(2));
    }
}

// extern crate tuitty;

// use std::thread;
// use std::time::Duration;

// use tuitty::{
//     common::{
//         traits::*,
//         enums::{ InputEvent, KeyEvent, Clear },
//     },
//     terminal::{ Terminal, CommonTerminal },
// };


// fn main() {
//     let mut tty = Terminal::init();
//     tty.switch();
//     tty.raw();

//     let t = thread::spawn(|| {
//         let cty = CommonTerminal::new();
//         let mut input = Terminal::read_async();

//         'out: loop {
//             let evt = input.next();
//             match evt {
//                 Some(e) => match e {
//                     InputEvent::Keyboard(k) => match k {
//                         KeyEvent::Esc => (),
//                         KeyEvent::Char(c) => match c {
//                             'q' => break 'out,
//                             _ => cty.printf("Some keyboard event!"),
//                         },
//                         _ => (),
//                     },
//                     _ => (),
//                 },
//                 None => ()
//             }
//         }
//         return
//     });
//     t.join().unwrap();

//     tty.clear(Clear::All);
//     tty.flush();

//     let g = thread::spawn(|| {
//         let cty = CommonTerminal::new();
//         let mut input = Terminal::read_async();

//         'out: loop {
//             let evt = input.next();
//             match evt {
//                 Some(e) => match e {
//                     InputEvent::Keyboard(k) => match k {
//                         KeyEvent::Esc => (),
//                         KeyEvent::Char(c) => match c {
//                             'q' => break 'out,
//                             _ => cty.printf("Some keyboard event!"),
//                         },
//                         _ => (),
//                     },
//                     _ => (),
//                 },
//                 None => ()
//             }
//         }
//         return
//     });
//     g.join().unwrap();
// }
