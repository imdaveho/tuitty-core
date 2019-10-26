extern crate tuitty;

use std::{ fs, thread, time::Duration, io::{ Read, BufReader } };
use std::sync::{
    Arc, mpsc::channel,
    atomic::{ AtomicBool, Ordering }
};

#[cfg(unix)]
use tuitty::{
    terminal::{ actions::posix, dispatch::unix::parser },
    common::enums::{ InputEvent, KeyEvent, MouseEvent },
};

fn main() {
    #[cfg(unix)] {
        let is_running = Arc::new(AtomicBool::new(true));
        let is_running_arc = is_running.clone();
        let original_mode = posix::get_mode();
        posix::enable_alt();
        posix::raw();
        posix::enable_mouse();
        posix::hide_cursor();
        posix::flush();
        let (input_tx, input_rx) = channel();

        let input_handle = thread::spawn(move || {
            // ORIGINAL - Hangs at end
            // while is_running_arc.load(Ordering::SeqCst) {
            //     let tty = BufReader::new(fs::OpenOptions::new()
            //                              .read(true).write(true).open("/dev/tty")
            //                              .expect("Error opening /dev/tty"));
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

            // TESTING MANUAL READS -- LOOKING GOOD!
            // let mut i = 0;
            // while i < 4 {
            //     let mut tty = BufReader::new(
            //         fs::OpenOptions::new()
            //             .read(true).write(true).open("/dev/tty")
            //             .expect("Error opening /dev/tty"));
            //     // DID NOT WORK
            //     // read_exact x2
            //     // let mut it1 = [0; 1];
            //     // let _ = tty.read_exact(&mut it1);
            //     // let mut it2 = [0; 1];
            //     // let _ = tty.read_exact(&mut it2);
            //     // println!("{:?}", it1);
            //     // println!("{:?}", it2);
            //     // DID WORK!
            //     let mut buf: [u8; 20] = [0; 20];
            //     let mut newr = tty.take(20);
            //     let _ = newr.read(&mut buf);
            //     let item = buf[0];
            //     let rest: Vec<u8> = buf[1..]
            //         .to_vec().into_iter()
            //         .filter(|x| x != &0).collect();
            //     let mut rest = rest.into_iter();
            //     let evt = parser::parse_event(item, &mut rest);
            //     // println!("{:?}", input);
            //     // println!("(item: {:?}, &mut iter: {:?})", item, iter);
            //     // println!("next: {:?}", iter.next());
            //     println!("{:?}", evt);
            //     i += 1;
            // }

            while is_running_arc.load(Ordering::SeqCst) {
                let tty = BufReader::new(
                    fs::OpenOptions::new()
                        .read(true).write(true).open("/dev/tty")
                        .expect("Error opening /dev/tty"));
                let (mut input, mut taken) = ([0; 12], tty.take(12));
                let _ = taken.read(&mut input);
                let item = input[0];
                // let rest: Vec<u8> = input[1..]
                //     .to_vec().into_iter()
                //     .filter(|x| x != &0).collect();
                // let mut rest = rest.into_iter();
                let mut rest = input[1..].to_vec().into_iter();
                let _ = input_tx.send(parser::parse_event(item, &mut rest));
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

            // Original
            // let evt = match iterator.next() {
            //     Some(ch) => {
            //         let parsed_evt = parser::parse_event(
            //             ch, &mut iterator);
            //         if let Ok(evt) = parsed_evt {
            //             Some(evt)
            //         } else { None }
            //     }
            //     None => None,
            // };

            // Scenario 1.
            let evt = iterator.next();

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

        // input_handle.join().expect("Error joining");
        // check_thread.join().expect("Error joining check_thread");

        posix::show_cursor();
        posix::disable_mouse();
        posix::cook(&original_mode);
        posix::disable_alt();
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
