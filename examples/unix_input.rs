extern crate tuitty;

use std::{ thread, time::Duration };
use tuitty::common::enums::{ InputEvent, KeyEvent };

// #[cfg(unix)]
use tuitty::terminal::actions::posix;

#[cfg(unix)]
fn main() {
    let mut dispatch = tuitty::terminal::dispatch::Dispatcher::init();

    let original_mode = posix::get_mode();
    posix::enable_alt();
    posix::raw();
    posix::hide_cursor();
    posix::flush();

    // let _ = dispatch.signal(EnableAlt);
    // let _ = dispatch.signal(Raw);
    // let _ = dispatch.signal(HideCursor);
    // let _ = dispatch.signal(Flush);

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
                            // listener.signal(Goto(0, 0));
                            // listener.signal(Prints(format!("char: {}\n", c)));
                            posix::goto(0, 0);
                            posix::prints(&format!("char: {}\n", c));
                        },
                        _ => ()
                    },
                    _ => ()
                },
                None => (),
            }
            thread::sleep(Duration::from_millis(10));
        }
        // listener.signal(ShowCursor);
        posix::show_cursor();
    });

    let counter = dispatch.listen(); // works as spawn or listen!
    let counter_handle = thread::spawn(move || {
        let mut i = 0;
        loop {
            // counter.signal(Goto(10,10));
            posix::goto(10, 10);
            let content = format!("count: {}", i);
            // counter.signal(Printf(content));
            posix::printf(&content);
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
            thread::sleep(Duration::from_millis(400));
        }
    });

    // let _ = dispatch.signal(Goto(14, 16));
    // let _ = dispatch.signal(Flush);
    // let (col, row) = listener.pos();
    // let _ = dispatch.signal(Goto(5, 3));
    // let _ = dispatch.signal(Printf(format!()));

    let listen = dispatch.listen();
    let listen_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(2000));
        posix::goto(14, 16);
        posix::flush();
        // listen.signal(Goto(14, 16));
        // listen.signal(Flush);
        let (col, row) = listen.pos();
        thread::sleep(Duration::from_millis(200));
        posix::goto(5, 3);
        posix::printf(&format!("col: {}, row: {}", col, row));
        // listen.signal(Goto(5, 3));
        // let st = format!("col: {}, row: {}", col, row);
        // listen.signal(Printf(st));
        thread::sleep(Duration::from_millis(1000));
        posix::goto(14, 16);
        posix::flush();
        let (w, h) = listen.size();
        thread::sleep(Duration::from_millis(200));
        posix::goto(5, 4);
        posix::printf(&format!("w: {}, h: {}", w, h));
    });


    listen_handle.join().expect("Counter failed to join");
    counter_handle.join().expect("Counter failed to join");
    listener_handle.join().expect("Counter failed to join");

    posix::cook(&original_mode);
    posix::disable_alt();
    posix::flush();

    // let _ = dispatch.signal(ShowCursor);
    // let _ = dispatch.signal(Cook);
    // let _ = dispatch.signal(DisableAlt);
    // let _ = dispatch.signal(Flush);

    thread::sleep(Duration::from_secs(2));
}
