extern crate tuitty;

use std::{ thread, time::Duration };

#[cfg(unix)]
use tuitty::terminal::actions::posix;

#[cfg(windows)]
use tuitty::terminal::actions::win32;


fn direct_scenario() {

    #[cfg(unix)] {
        let initial = posix::get_mode();

        posix::enable_alt();
        posix::raw();

        posix::goto(0, 0);
        posix::printf("Hello, world!");
        thread::sleep(Duration::from_secs(2));

        posix::cook(&initial);
        posix::disable_alt();
        thread::sleep(Duration::from_secs(1));
    }

    #[cfg(windows)] {
        let vte = win32::is_ansi_enabled();
        let initial = win32::get_mode();
        let screen = win32::Handle::buffer().unwrap();

        win32::enable_alt(&screen, &initial, vte);
        win32::raw();

        win32::goto(0, 0, vte);
        win32::printf("Hello, world!", vte);
        thread::sleep(Duration::from_secs(2));

        win32::cook();
        win32::disable_alt(vte);
        thread::sleep(Duration::from_secs(1));
    }

}

use tuitty::common::enums::Action::*;
use tuitty::terminal::dispatch::Dispatcher;

fn signal_scenario() {
    let dispatch = Dispatcher::init();
    dispatch.signal(EnableAlt);
    // dispatch.signal(Raw);

    // dispatch.signal(Goto(0, 0));
    // dispatch.signal(Printf(format!("Hello, world!")));
    thread::sleep(Duration::from_secs(2));

    // dispatch.signal(Cook);
    dispatch.signal(DisableAlt);
    // thread::sleep(Duration::from_secs(1));
}

#[cfg(windows)]
fn raw_scenario() {
    // let initial = win32::get_mode();
    let vte = win32::is_ansi_enabled();
    // let screen = win32::Handle::buffer()
    //     .expect("Error creating alternate Console buffer");
    // win32::printf("\x1B[?1049h", true);
    // win32::printf("Hello, alt.", true);
    // thread::sleep(Duration::from_secs(2));
    // win32::printf("\x1B[?1049l", true);

    // win32::enable_alt(&screen, &initial, vte);
    // win32::goto(0, 0, vte);
    // win32::prints("Hello, world", vte);
    // thread::sleep(Duration::from_secs(2));
    // win32::disable_alt(vte);
    // thread::sleep(Duration::from_secs(5));

    println!("{}", vte);
}

fn main() {
    // #[cfg(windows)]
    // raw_scenario();

    signal_scenario();
}
