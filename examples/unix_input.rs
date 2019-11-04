extern crate tuitty;

use std::{ thread, time::Duration };
use tuitty::common::DELAY;
use tuitty::common::enums::{ InputEvent, KeyEvent, Action::*, Color, Clear };

#[cfg(unix)]
use tuitty::terminal::actions::posix;


#[cfg(unix)]
fn main() {
    let mut dispatch = tuitty::terminal::dispatch::Dispatcher::init();
    let _ = dispatch.signal(EnableAlt);
    let _ = dispatch.signal(Raw);
    let _ = dispatch.signal(EnableMouse);
    let _ = dispatch.signal(HideCursor);
    // posix::enable_alt();
    // let initial = posix::get_mode();
    // posix::raw();

    let listener = dispatch.listen();
    let mut counter = 0;
    loop {
        let (col, row) = listener.coord();
        listener.signal(Goto(10, 10));
        listener.signal(Prints(format!("Counter: (")));
        listener.signal(SetFg(Color::Red));
        listener.signal(Prints(format!("{}", counter)));
        listener.signal(SetFg(Color::Reset));
        listener.signal(Prints(format!(")")));
        listener.signal(Clear(Clear::NewLn));
        listener.signal(Goto(col, row));
        thread::sleep(Duration::from_millis(DELAY));
        counter += 1;
        if counter > 5000 { counter = 0 }
        listener.signal(Refresh);
        if let Some(evt) = listener.poll_latest_async() {
            if let InputEvent::Keyboard(kv) = evt {
                match kv {
                    KeyEvent::Left => {
                        listener.signal(Left(1));
                    },
                    KeyEvent::Right => {
                        listener.signal(Right(1));
                    },
                    KeyEvent::Up => {
                        listener.signal(Up(1));
                    },
                    KeyEvent::Down => {
                        listener.signal(Down(1));
                    },
                    KeyEvent::Char(c) => if c == 'q' {
                        // posix::cook(&initial);
                        // posix::disable_alt();
                        let _ = listener.signal(Cook);
                        let _ = listener.signal(DisableAlt);
                        break
                    },
                    KeyEvent::Enter => listener.signal(Refresh),
                    _ => (),
                }
            }
        }
    }
}
