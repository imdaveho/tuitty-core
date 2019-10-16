extern crate tuitty;

use std::thread;
use std::time::Duration;

use tuitty::{
    common::{
        traits::*,
        enums::{ InputEvent, KeyEvent, Clear },
    },
    terminal::{ Terminal, CommonTerminal },
};


fn main() {
    let mut tty = Terminal::init();
    tty.switch();
    tty.raw();

    let t = thread::spawn(|| {
        let cty = CommonTerminal::new();
        let mut input = Terminal::read_async();

        'out: loop {
            let evt = input.next();
            match evt {
                Some(e) => match e {
                    InputEvent::Keyboard(k) => match k {
                        KeyEvent::Esc => (),
                        KeyEvent::Char(c) => match c {
                            'q' => break 'out,
                            _ => cty.printf("Some keyboard event!"),
                        },
                        _ => (),
                    },
                    _ => (),
                },
                None => ()
            }
        }
        return
    });
    t.join().unwrap();

    tty.clear(Clear::All);
    tty.flush();

    let g = thread::spawn(|| {
        let cty = CommonTerminal::new();
        let mut input = Terminal::read_async();

        'out: loop {
            let evt = input.next();
            match evt {
                Some(e) => match e {
                    InputEvent::Keyboard(k) => match k {
                        KeyEvent::Esc => (),
                        KeyEvent::Char(c) => match c {
                            'q' => break 'out,
                            _ => cty.printf("Some keyboard event!"),
                        },
                        _ => (),
                    },
                    _ => (),
                },
                None => ()
            }
        }
        return
    });
    g.join().unwrap();
}
