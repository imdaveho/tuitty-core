extern crate tuitty;

use std::{thread, time};
use tuitty::common::DELAY;
use tuitty::common::enums::{ InputEvent, KeyEvent, Action::* };

fn main() {
    let mut dispatch = tuitty::terminal::dispatch::Dispatcher::new();
    let dispatch = dispatch
        .listen()
        .dispatch();
    
    let listener = dispatch.spawn()
        .expect("Error spawning");

    listener.signal(Raw).expect("Error signaling dispatch - raw");
    listener.signal(EnableAlt).expect("Error signaling dispatch - alt");

    let listener_handle = thread::spawn(move || loop {
        match listener.poll_async() {
            Some(evt) => match evt {
                InputEvent::Keyboard(kv) => match kv {
                    KeyEvent::Char(c) => {
                        if c == 'q' {
                            // dispatch.shutdown();
                            break
                        }
                        listener.signal(Goto(0, 0)).expect("Error signaling dispatch - goto");
                        listener.signal(Prints(format!("char: {}\n", c))).expect("Error signaling dispatch - prints");
                    },
                    _ => ()
                },
                _ => ()
            },
            None => (),
        }
        thread::sleep(time::Duration::from_millis(DELAY));
    });
    
    let counter = dispatch.spawn().expect("Error spawning");

    let counter_handle = thread::spawn(move || {
        let mut i = 0;
        counter.signal(HideCursor).expect("Error signaling dispatch - hidecursor");
        loop {
            counter.signal(Goto(10,10)).expect("Error signaling dispatch - goto");
            let content = format!("count: {}", i);
            counter.signal(Printf(content)).expect("Error signaling dispatch - printf");
            i += 1;
            thread::sleep(time::Duration::from_millis(1200));
            match counter.poll_async() {
                Some(evt) => match evt {
                    InputEvent::Keyboard(kv) => match kv {
                        KeyEvent::Char(c) => {
                            if c == ';' { break }
                        },
                        _ => (),
                    },
                    _ => (),
                },
                None => (),
            }
        }
    });

    let _ = listener_handle.join();
    let _ = counter_handle.join();

    let listener = dispatch.spawn().expect("Error spawning");
    listener.signal(Cook).expect("Error signaling dispatch - cook");
    listener.signal(DisableAlt).expect("Error signaling dispatch - stdout");

    dispatch.shutdown();

    thread::sleep(time::Duration::from_millis(2000));
}