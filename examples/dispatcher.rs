extern crate tuitty;

use std::{thread, time};
use tuitty::common::DELAY;
use tuitty::common::enums::{ InputEvent, KeyEvent, Action::* };

fn main() {
    let mut dispatch = tuitty::terminal::dispatch::Dispatcher::init();
    let dispatch = dispatch.listen();
    let listener = dispatch.spawn();

    dispatch.signal(Raw).expect("Error signaling dispatch - raw");
    dispatch.signal(EnableAlt).expect("Error signaling dispatch - alt");

    let listener_handle = thread::spawn(move || loop {
        match listener.poll_async() {
            Some(evt) => match evt {
                InputEvent::Keyboard(kv) => match kv {
                    KeyEvent::Char(c) => {
                        if c == 'q' {
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
        thread::sleep(time::Duration::from_millis(400));
    });

    let counter = dispatch.spawn();

    let counter_handle = thread::spawn(move || {
        let mut i = 0;
        counter.signal(HideCursor).expect("Error signaling dispatch - hidecursor");
        loop {
            counter.signal(Goto(10,10)).expect("Error signaling dispatch - goto");
            let content = format!("count: {}", i);
            counter.signal(Printf(content)).expect("Error signaling dispatch - printf");
            i += 1;
            match counter.poll_async() {
            // BECAUSE YOU ARE COPYING THE VALUES, AND THE DELAY IS GREATER,
            // THERE IS A BACKLOG OF EVTS THAT NEED TO BE CLEARED BEFORE
            // HITTING THIS BREAK IN ORDER TO JOIN().
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
            thread::sleep(time::Duration::from_millis(400));
        }
    });

    listener_handle.join().expect("Listener failed to join");
    counter_handle.join().expect("Counter failed to join");

    dispatch.signal(Cook).expect("Error signaling dispatch - cook");
    dispatch.signal(DisableAlt).expect("Error signaling dispatch - stdout");

    dispatch.shutdown().expect("Dispatch shutdown error on threads");

    // thread::sleep(time::Duration::from_millis(2000));
}
