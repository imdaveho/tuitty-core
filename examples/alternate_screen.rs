extern crate tuitty;

use std::{ thread, time::Duration };

#[cfg(unix)]
use tuitty::terminal::actions::unix;

#[cfg(windows)]
use tuitty::terminal::actions::windows;


fn direct_scenario() {

    #[cfg(unix)] {
        let initial = unix::get_mode();

        unix::enable_alt();
        unix::raw();

        unix::goto(0, 0);
        unix::printf("Hello, world!");
        thread::sleep(Duration::from_secs(2));

        unix::cook(&initial);
        unix::disable_alt();
        thread::sleep(Duration::from_secs(1));
    }

    #[cfg(windows)] {
        let vte = windows::is_ansi_enabled();
        let initial = windows::get_mode();
        let screen = windows::Handle::buffer().unwrap();

        windows::enable_alt(&screen, &initial, vte);
        windows::raw();

        windows::goto(0, 0, vte);
        windows::printf("Hello, world!", vte);
        thread::sleep(Duration::from_secs(2));

        windows::cook();
        windows::disable_alt(vte);
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
