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
    dispatch.signal(EnableAlt).expect("Error -- enable_alt");
    dispatch.signal(Raw).expect("Error -- raw");

    dispatch.signal(Goto(0, 0)).expect("Error -- goto(0, 0)");
    dispatch.signal(Printf(format!("Hello, world!"))).expect("Error -- printf");
    thread::sleep(Duration::from_secs(2));

    dispatch.signal(Cook).expect("Error -- cook");
    dispatch.signal(DisableAlt).expect("Error -- disable_alt");
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    signal_scenario();
}
