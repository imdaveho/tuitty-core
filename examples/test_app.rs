extern crate tuitty;

use std::{thread, time};
// use tuitty::common::DELAY;
// use tuitty::common::enums::{ InputEvent, KeyEvent, Action::*, Clear::* };
use tuitty::common::enums::{ Color, Effect, Action::* };
use tuitty::terminal::dispatch::{ Dispatcher, EventHandle };


fn render_banner(handle: &EventHandle) {
    let (w, _) = handle.size();
    handle.signal(Goto(0, 0));
    handle.signal(Prints(format!("≡ ")));
    handle.signal(SetFx(Effect::Underline as u32));
    handle.signal(Prints(format!("David Ho")));
    handle.signal(SetFx(Effect::Reset as u32));
    handle.signal(Goto(w - 4, 0));
    handle.signal(SetFg(Color::Red));
    handle.signal(Prints(format!("[x]")));
    handle.signal(SetFg(Color::Reset));
    handle.signal(Goto(0, 1));
    handle.signal(SetFx(Effect::Dim as u32));
    handle.signal(Prints("─".repeat(w as usize)));
    handle.signal(SetFx(Effect::Reset as u32));
    handle.signal(Flush);
}


fn main() {
    let mut dispatch = Dispatcher::init();
    let listener = dispatch.listen();
    dispatch.signal(Switch);

    dispatch.signal(Raw);
    dispatch.signal(HideCursor);

    render_banner(&listener);

    dispatch.signal(ShowCursor);
    dispatch.signal(Cook);

    thread::sleep(time::Duration::from_secs(2));
    dispatch.signal(SwitchTo(0));
}
