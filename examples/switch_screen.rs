extern crate tuitty;

use std::{ thread, time::Duration };
use tuitty::common::DELAY;
use tuitty::common::enums::{
    InputEvent, KeyEvent, Action::*, Clear::*, Color
};

#[cfg(unix)]
use tuitty::terminal::actions::posix;

fn main() {
    let mut dispatch = tuitty::terminal::dispatch::Dispatcher::init();
    let listener = dispatch.listen();
    let (col, row) = listener.coord();
    dispatch.signal(SetBg(Color::Yellow));
    dispatch.signal(Printf(format!("Main screen check at: {}, {}", col, row)));
    dispatch.signal(SetBg(Color::Reset));
    dispatch.signal(Raw);
    loop {
        if let Some(evt) = listener.poll_latest_async() {
            if let InputEvent::Keyboard(kv) = evt {
                match kv {
                    KeyEvent::Left => {
                        listener.signal(Left(1));
                        listener.signal(Flush);
                    },
                    KeyEvent::Right => {
                        listener.signal(Right(1));
                        listener.signal(Flush);
                    },
                    KeyEvent::Up => {
                        listener.signal(Up(1));
                        listener.signal(Flush);
                    },
                    KeyEvent::Down => {
                        listener.signal(Down(1));
                        listener.signal(Flush);
                    },
                    KeyEvent::Enter => {
                        let (col, row) = listener.coord();
                        let c = listener.getch();
                        listener.signal(Goto(0, 0));
                        listener.signal(Clear(NewLn));
                        listener.signal(Printf(format!("ch: ({}), col: {}, row: {}", c, col, row)));
                        listener.signal(Goto(col, row));
                        listener.signal(Flush);
                    },
                    KeyEvent::Char(c) => if c == 'q' { break },
                    _ => (),
                }
            }
        }
        thread::sleep(Duration::from_millis(DELAY));
    }

    dispatch.signal(Switch);
    let _ = dispatch.signal(Raw);
    let (col, row) = listener.coord();
    dispatch.signal(Printf(format!("Alternate Screen 1 @ {}, {}", col, row)));
    loop {
        if let Some(evt) = listener.poll_latest_async() {
            if let InputEvent::Keyboard(kv) = evt {
                match kv {
                    KeyEvent::Left => {
                        listener.signal(Left(1));
                        listener.signal(Flush);
                    },
                    KeyEvent::Right => {
                        listener.signal(Right(1));
                        listener.signal(Flush);
                    },
                    KeyEvent::Up => {
                        listener.signal(Up(1));
                        listener.signal(Flush);
                    },
                    KeyEvent::Down => {
                        listener.signal(Down(1));
                        listener.signal(Flush);
                    },
                    KeyEvent::Enter => {
                        let (col, row) = listener.coord();
                        let c = listener.getch();
                        listener.signal(Goto(0, 1));
                        listener.signal(Clear(NewLn));
                        listener.signal(Printf(format!("ch: ({}), col: {}, row: {}", c, col, row)));
                        listener.signal(Goto(col, row));
                        listener.signal(Flush);
                    },
                    KeyEvent::Char(c) => {
                        if c == 'q' { break }
                        if c == '!' {
                            listener.signal(Switch);
                            let _ = listener.signal(Raw);
                            listener.signal(Goto(10, 3));
                            listener.signal(Flush);
                            let (col, row) = listener.coord();
                            dispatch.signal(SetFg(Color::Green));
                            listener.signal(Printf(format!("Alternate Screen 2 @ {}, {}", col, row)));
                            dispatch.signal(SetFg(Color::Reset));
                            listener.signal(Goto(col, row));
                            listener.signal(Flush);
                        }
                        if c == '1' { listener.signal(SwitchTo(1)) }
                        if c == '2' { listener.signal(SwitchTo(2)) }
                        if c == '0' { listener.signal(SwitchTo(0)) }
                    },
                    _ => (),
                }
            }
        }
        thread::sleep(Duration::from_millis(DELAY));
    }

    listener.signal(SwitchTo(0));
    dispatch.signal(Cook);

    thread::sleep(Duration::from_millis(2000));
}
