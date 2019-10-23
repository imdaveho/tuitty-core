extern crate tuitty;

use std::{thread, time};
use tuitty::common::DELAY;
use tuitty::common::enums::{ InputEvent, KeyEvent, Action::* };

fn main() {
    let mut dispatch = tuitty::terminal::dispatch::Dispatcher::init();
    let _stdin = dispatch.listen();
    dispatch.signal(Raw).expect("Error signaling dispatch - raw");
    dispatch.signal(EnableAlt).expect("Error signaling dispatch - alt");

    let listener = dispatch.spawn();
    let listener_handle = thread::spawn(move || loop {
        match listener.poll_latest_async() {
            Some(evt) => match evt {
                InputEvent::Keyboard(kv) => match kv {
                    KeyEvent::Char(c) => {
                        if c == 'q' {
                            break
                        }
                        listener.signal(Goto(0, 0));
                        listener.signal(Prints(format!("char: {}\n", c)));
                    },
                    _ => ()
                },
                _ => ()
            },
            None => (),
        }
        thread::sleep(time::Duration::from_millis(DELAY));
    });

    let counter = dispatch.spawn();
    let counter_handle = thread::spawn(move || {
        let mut i = 0;
        counter.signal(HideCursor);
        counter.lock();
        loop {
            counter.signal(Goto(10,10));
            let content = format!("count: {}", i);
            counter.signal(Printf(content));
            i += 1;
            match counter.poll_latest_async() {
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
            thread::sleep(time::Duration::from_millis(1000));
        }
        counter.unlock();
    });

    listener_handle.join().expect("Listener failed to join");
    counter_handle.join().expect("Counter failed to join");

    dispatch.signal(Cook).expect("Error signaling dispatch - cook");
    dispatch.signal(DisableAlt).expect("Error signaling dispatch - stdout");

    dispatch.shutdown().expect("Dispatch shutdown error on threads");

    // thread::sleep(time::Duration::from_millis(2000));
}
