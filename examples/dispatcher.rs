extern crate tuitty;

use std::{thread, time};
use tuitty::common::DELAY;
use tuitty::common::enums::{ InputEvent, KeyEvent, Action::*, Clear::* };


fn main() {
    let mut dispatch = tuitty::terminal::dispatch::Dispatcher::init();

    dispatch.signal(EnableAlt).expect("Error signaling dispatch - alt");
    dispatch.signal(Raw).expect("Error signaling dispatch - raw");

    let listener = dispatch.listen();
    let listener_handle = thread::spawn(move || {
        loop {
            match listener.poll_latest_async() {
                Some(evt) => match evt {
                    InputEvent::Keyboard(kv) => match kv {
                        KeyEvent::Char(c) => {
                            if c == 'q' {
                                break
                            }
                            listener.signal(Goto(0, 0));
                            listener.signal(Printf(format!("char: {}\n\nhello\r\n0\t\t㓘7", c)));
                        },
                        KeyEvent::Left => {
                            listener.signal(Left(1));
                            listener.signal(Flush);
                            let (col, row) = listener.coord();
                            listener.signal(Goto(0, 11));
                            listener.signal(Clear(NewLn));
                            listener.signal(Printf(format!("col: {}, row: {}", col, row)));
                            listener.signal(Goto(col, row));
                            listener.signal(Flush);
                        },
                        KeyEvent::Right => {
                            listener.signal(Right(1));
                            listener.signal(Flush);
                            let (col, row) = listener.coord();
                            listener.signal(Goto(0, 11));
                            listener.signal(Clear(NewLn));
                            listener.signal(Printf(format!("col: {}, row: {}", col, row)));
                            listener.signal(Goto(col, row));
                            listener.signal(Flush);
                        },
                        KeyEvent::Up => {
                            listener.signal(Up(1));
                            listener.signal(Flush);
                            let (col, row) = listener.coord();
                            listener.signal(Goto(0, 11));
                            listener.signal(Clear(NewLn));
                            listener.signal(Printf(format!("col: {}, row: {}", col, row)));
                            listener.signal(Goto(col, row));
                            listener.signal(Flush);
                        },
                        KeyEvent::Down => {
                            listener.signal(Down(1));
                            listener.signal(Flush);
                            let (col, row) = listener.coord();
                            listener.signal(Goto(0, 11));
                            listener.signal(Clear(NewLn));
                            listener.signal(Printf(format!("col: {}, row: {}", col, row)));
                            listener.signal(Goto(col, row));
                            listener.signal(Flush);
                        }
                        KeyEvent::Enter => {
                            let c = listener.getch();
                            listener.signal(Goto(30, 10));
                            listener.signal(Clear(NewLn));
                            listener.signal(Printf(format!("ch: {}", c)));
                        }
                        _ => ()
                    },
                    _ => ()
                },
                None => (),
            }
            thread::sleep(time::Duration::from_millis(DELAY));
        }
        // listener.signal(ShowCursor);
    });

    let counter = dispatch.listen(); // works as spawn or listen!
    let counter_handle = thread::spawn(move || {
        let mut i = 0;
        // counter.signal(HideCursor);
        counter.lock();
        loop {
            counter.signal(Goto(10,10));
            let content = format!("count: {}", i);
            counter.signal(Printf(content));
            counter.signal(Flush);
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
            thread::sleep(time::Duration::from_millis(400));
        }
        counter.unlock();
    });

    listener_handle.join().expect("Listener failed to join");
    counter_handle.join().expect("Counter failed to join");

    dispatch.signal(Goto(10, 10)).expect("Error goto");
    dispatch.signal(Printf("Hello, World".to_string())).expect("Error printf");

    thread::sleep(time::Duration::from_millis(2000));

    dispatch.signal(Cook).expect("Error signaling dispatch - cook");
    dispatch.signal(DisableAlt).expect("Error signaling dispatch - stdout");

    thread::sleep(time::Duration::from_millis(2000));
}
