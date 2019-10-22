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
        .expect("Error spawning an event listener handle");

    listener.signal(Raw).expect("Error signaling dispatch - raw");
    listener.signal(EnableAlt).expect("Error signaling dispatch - alt");

    loop {
        match listener.poll_async() {
            Some(evt) => match evt {
                InputEvent::Keyboard(kv) => match kv {
                    KeyEvent::Char(c) => {
                        if c == 'q' {
                            // dispatch.shutdown();
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

    listener.signal(Cook).expect("Error signaling dispatch - cook");
    listener.signal(DisableAlt).expect("Error signaling dispatch - stdout");

    dispatch.shutdown();

    thread::sleep(time::Duration::from_millis(2000));
}