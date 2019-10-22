extern crate tuitty;

use std::{thread, time};
// use std::sync::atomic::Ordering;
use tuitty::common::DELAY;
use tuitty::common::enums::{ InputEvent, KeyEvent };
// use tuitty::terminal::actions::*;

fn main() {

    let alternate = tuitty::terminal::actions::wincon::windows::Handle::buffer().unwrap();
    let original_mode = tuitty::terminal::actions::wincon::windows::get_mode().unwrap();
    let _ = tuitty::terminal::actions::wincon::windows::enable_raw();

    alternate.set_mode(&original_mode).unwrap();
    alternate.show().unwrap();

    let mut dispatch = tuitty::terminal::dispatch::Dispatcher::new();
    let dispatch = dispatch.listen().dispatch();
    let listener = dispatch.spawn()
        .expect("Error spawning an event listener handle");

    loop {
        match listener.poll_async() {
            Some(evt) => match evt {
                InputEvent::Keyboard(kv) => match kv {
                    KeyEvent::Char(c) => {
                        if c == 'q' {
                            dispatch.shutdown();
                            break
                        }
                        let _ = tuitty::terminal::actions::wincon::windows::prints(
                            &format!("char: {}\n", c));
                    },
                    _ => ()
                },
                _ => ()
            },
            None => (),
        }
        thread::sleep(time::Duration::from_millis(DELAY));
    }

    let _ = tuitty::terminal::actions::wincon::windows::disable_raw();
    let stdout = tuitty::terminal::actions::wincon::windows::Handle::stdout().unwrap();
    stdout.set_mode(&original_mode).unwrap();
    stdout.show().unwrap();

    // // loop {
    // //     match dispatch.listener.try_recv() {
    // //         Ok(evt) => match evt {
    // //             InputEvent::Keyboard(kv) => match kv {
    // //                 KeyEvent::Char(c) => {
    // //                     if c == 'q' {
    // //                         dispatch.shutdown.store(true, Ordering::SeqCst);
    // //                         break
    // //                     }
    // //                     tuitty::terminal::actions::wincon::windows::prints(&format!("char: {}", c));
    // //                 },
    // //                 _ => ()
    // //             },
    // //             _ => ()
    // //         },
    // //         Err(_) => ()
    // //     }
    // //     thread::sleep(time::Duration::from_millis(100));
    // // }

    // let stopsig = dispatch.stop();
    // let listener = dispatch.listen();
    // // dispatch.broadcast();
    // let handle = thread::spawn(move || {
    //    dispatch.broadcast(); // needs to be in separate thread...so launching 2 threads??? coulnd't we just launch 1?
    // });

    // loop {
    //     match listener.poll_async() {
    //         Some(evt) => match evt {
    //             InputEvent::Keyboard(kevt) => match kevt {
    //                 KeyEvent::Char(c) => {
    //                     if c == 'q' {
    //                         stopsig.store(true, Ordering::SeqCst);
    //                         break
    //                     }
    //                     tuitty::terminal::actions::wincon::windows::prints(&format!("char: {}", c));
    //                 },
    //                 KeyEvent::Ctrl(c) => {
    //                     tuitty::terminal::actions::wincon::windows::prints(&format!("ctrl + {}", c));
    //                 },
    //                 _ => (),
    //             },
    //             _ => (),
    //         }
    //         None => (),
    //     }
    //     // let event = listener.poll_async();
    //     // match event {
    //     //     Some(e) => tuitty::terminal::actions::wincon::windows::prints("Some "),
    //     //     None => tuitty::terminal::actions::wincon::windows::prints("None "),
    //     // };

    //     thread::sleep(time::Duration::from_millis(100));
    // }

    // // thread::sleep(time::Duration::from_millis(2000));

    // // match listener.poll_async() {
    // //     Some(evt) => match evt {
    // //         InputEvent::Keyboard(kevt) => match kevt {
    // //             KeyEvent::Char(c) => {
    // //                 if c == 'q' {
    // //                     dispatch.shutdown();
    // //                 }
    // //                 println!("char: {}", c);
    // //             },
    // //             KeyEvent::Ctrl(c) => {
    // //                 println!("ctrl + {}", c);
    // //             },
    // //             _ => (),
    // //         },
    // //         _ => (),
    // //     }
    // //     None => (),
    // // }

    // handle.join();
    
    // // match term {
    // //     Ansi(_) => (),
    // //     Win32(_) => {
    // //         let tty = wincon::Win32Console;
    // //         tty::cook();
    // //         tty::disable_alt();
    // //     }
    // // }
}