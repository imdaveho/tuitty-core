// #[cfg(unix)]
// extern crate tuitty;

// #[cfg(unix)]
// use tuitty::terminal::actions::posix;

// #[cfg(unix)]
// fn main() {
//     let initial = posix::get_mode();
//     posix::raw();
//     let mut count = 0;
//     while count < 10 {
//         let tty = match std::fs::OpenOptions::new()
//             .read(true).write(true).open("/dev/tty")
//         {
//             Ok(f) => std::io::BufReader::new(f),
//             Err(_) => continue
//         };
//         let (mut input, mut taken) = (
//             [0; 12], std::io::Read::take(tty, 12));
//         let _ = std::io::Read::read(&mut taken, &mut input);

//         // Parse the user input from /dev/tty.
//         let item = input[0];
//         let rest = input[1..].to_vec().into_iter();
//         posix::goto(0, 27);
//         println!("{:?}", item);
//         posix::goto(0, 28);
//         println!("{:?}", rest);
//         count += 1;
//     }
//     posix::goto(0, 29);
//     posix::cook(&initial);
// }


extern crate tuitty;

use std::{ thread, time::Duration, sync::{ Arc, atomic::{ AtomicBool, Ordering }}};
use tuitty::common::enums::{ InputEvent, KeyEvent, Action::*, Color, Clear };
use tuitty::common::enums::{ MouseEvent::*, MouseButton as Btn };


fn main() {
    let mut dispatch = tuitty::terminal::dispatch::Dispatcher::init();
    dispatch.signal(EnableAlt);
    dispatch.signal(Raw);
    dispatch.signal(EnableMouse);

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
            thread::sleep(Duration::from_millis(200));
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
                        dispatch.signal(Flush);
                    },
                    KeyEvent::Right => {
                        dispatch.signal(Right(1));
                        dispatch.signal(Flush);
                    },
                    KeyEvent::Up => {
                        dispatch.signal(Up(1));
                        dispatch.signal(Flush);
                    },
                    KeyEvent::Down => {
                        dispatch.signal(Down(1));
                        dispatch.signal(Flush);
                    },
                    KeyEvent::Esc => {
                        dispatch.signal(Printf(format!("Esc")))
                    },
                    KeyEvent::Alt(c) => {
                        dispatch.signal(Printf(format!("alt({})", c)))
                    },
                    KeyEvent::Char(c) => if c == 'q' {
                        break switch.store(false, Ordering::SeqCst)
                    },
                    _ => (),
                }
            } else if let InputEvent::Mouse(mv) = evt {
                // NOTE: checking https://github.com/imdaveho/tuitty/issues/2
                match mv {
                    Press(btn, col, row) => match btn {
                        Btn::Left => dispatch.signal(Printf(format!("mbLeft: {}, {}", col, row))),
                        _ => (),
                    },
                    _ => ()
                }
            }
        }

        thread::sleep(Duration::from_millis(40));
    }

    let _ = counter_handle.join();

    dispatch.signal(DisableMouse);
    dispatch.signal(Cook);
    dispatch.signal(DisableAlt);
}
