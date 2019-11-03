extern crate tuitty;

use std::{thread, time};
use tuitty::common::DELAY;
use tuitty::common::enums::{ InputEvent, KeyEvent, Action::*, Clear };

#[cfg(unix)]
use tuitty::terminal::actions::posix;

fn main() {
    let mut dispatch = tuitty::terminal::dispatch::Dispatcher::init();

    // dispatch.signal(EnableAlt).expect("Error signaling dispatch - alt");
    dispatch.signal(Raw).expect("Error signaling dispatch - raw");
    // dispatch.signal(ShowCursor);
    // dispatch.signal(Flush);
    posix::show_cursor();
    posix::flush();


    dispatch.signal(Goto(10, 10));
    dispatch.signal(Prints(format!("count: 11")));
    dispatch.signal(Flush);

    let listener = dispatch.listen();

    loop {
        match listener.poll_latest_async() {
            Some(evt) => match evt {
                InputEvent::Keyboard(kv) => match kv {
                    KeyEvent::Char(c) => {
                        if c == 'q' {
                            break
                        }
                        listener.signal(Prints(format!("{}", c)));
                        listener.signal(Flush);
                        listener.signal(Goto(10, 11));
                    },
                    KeyEvent::Enter => {
                        listener.signal(Prints(format!("\t")));
                        listener.signal(Flush);
                    }
                    _ => ()
                },
                _ => ()
            },
            None => (),
        }
        thread::sleep(time::Duration::from_millis(DELAY));
    }

    dispatch.signal(Cook).expect("Error signaling dispatch - cook");
    // dispatch.signal(DisableAlt).expect("Error signaling dispatch - stdout");

    thread::sleep(time::Duration::from_millis(2000));
}
