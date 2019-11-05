extern crate tuitty;

use std::{ thread, time::Duration, sync::{ Arc, atomic::{ AtomicBool, Ordering }}};
use tuitty::common::enums::{ InputEvent, KeyEvent, Action::*, Color, Clear };


#[cfg(unix)]
fn main() {
    let mut dispatch = tuitty::terminal::dispatch::Dispatcher::init();
    dispatch.signal(EnableAlt);
    dispatch.signal(Raw);

    let listener = dispatch.listen();
    let counter = dispatch.spawn();
    let switch = Arc::new(AtomicBool::new(true));
    let switch_arc = switch.clone();
    let mut count = 0;

    let counter_handle = thread::spawn(move || {
        while switch_arc.load(Ordering::SeqCst) {
            let (col, row) = counter.coord();
            counter.signal(Goto(10, 10));
            counter.signal(Prints(format!("Counter: (")));
            counter.signal(SetFg(Color::Red));
            counter.signal(Prints(format!("{}", count)));
            counter.signal(SetFg(Color::Reset));
            counter.signal(Prints(format!(")")));
            counter.signal(Clear(Clear::NewLn));
            counter.signal(Goto(col, row));
            thread::sleep(Duration::from_millis(100));
            count += 1;
            if count > 200 { count = 0 }
        }
    });

    loop {
        if let Some(evt) = listener.poll_latest_async() {
            if let InputEvent::Keyboard(kv) = evt {
                match kv {
                    KeyEvent::Left => {
                        dispatch.signal(Left(1));
                    },
                    KeyEvent::Right => {
                        dispatch.signal(Right(1));
                    },
                    KeyEvent::Up => {
                        dispatch.signal(Up(1));
                    },
                    KeyEvent::Down => {
                        dispatch.signal(Down(1));
                    },
                    KeyEvent::Char(c) => if c == 'q' {
                        break switch.store(false, Ordering::SeqCst)
                    },
                    _ => (),
                }
            }
        }
        dispatch.signal(Flush);
        thread::sleep(Duration::from_millis(40));
    }

    let _ = counter_handle.join();

    dispatch.signal(Cook);
    dispatch.signal(DisableAlt);
}
