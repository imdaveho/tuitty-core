extern crate tuitty;

use std::{ fs, thread, time::Duration, io::{ Read, BufReader } };
use std::sync::{
    Arc, mpsc::channel,
    atomic::{ AtomicBool, Ordering }
};

#[cfg(unix)]
use tuitty::{
    terminal::{
        dispatch::unix::parser,
        actions::ansi::{
            unix, AnsiAction,
            AnsiTerminal as Terminal,
        },
    },
    common::enums::{ InputEvent, KeyEvent },
};


fn main() {
    #[cfg(unix)] {
        let is_running = Arc::new(AtomicBool::new(true));
        let is_running_arc = is_running.clone();
        let original_mode = unix::get_mode().expect("Error fetching Termios");
        Terminal::enable_alt();
        Terminal::raw();
        let (input_tx, input_rx) = channel();

        let input_handle = thread::spawn(move || {
            while is_running_arc.load(Ordering::SeqCst) {
                let tty = BufReader::new(fs::OpenOptions::new()
                                         .read(true).write(true).open("/dev/tty")
                                         .expect("Error opening /dev/tty"));
                for byte in tty.bytes() {
                    if !is_running_arc.load(Ordering::SeqCst) { break }
                    let b = byte.expect("Error reading byte from /dev/tty");
                    // (imdaveho) TODO: Handle the Err state?
                    // Previous: break out of the loop. But might
                    // have caused weird conditions on .join() --
                    // further observation needed.
                    let _ = input_tx.send(b);
                }
                thread::sleep(Duration::from_millis(10));
            }
        });

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
                            Terminal::goto(0, 0);
                            Terminal::printf(&format!("Pressed: {}", c));
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
        Terminal::cook(&original_mode);
        Terminal::disable_alt();
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
