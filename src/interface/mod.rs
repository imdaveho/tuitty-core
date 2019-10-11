//! TODO: placeholder for specific components to build a TUI.


use crate::terminal::Terminal;
use crate::common::{
    enums::{ InputEvent, KeyEvent, MouseEvent },
    traits::{ 
        TerminalFormatter, TerminalInput, TerminalCursor, TerminalWriter
    },
};


pub fn draw_sides(t: &mut Terminal) {
    let (w, h) = t.screen_size();
    let (mw, mh) = (w/2, h/2);
    let (vb, hb) = (4, 15);
    let top = mh - vb;
    let bot = mh + vb;
    let left = mw - hb;
    let right = mw + hb;
    
    let mut input = t.read_async();

    for col in left..right {
        t.goto(col, top);
        t.prints("-");
    }

    for col in left..right {
        t.goto(col, bot);
        t.prints("-");
    }

    for row in top..bot {
        t.goto(left, row);
        t.prints("|");
    }

    for row in top..bot {
        t.goto(right, row);
        t.prints("|");
    }

    t.flush();

    loop {
        if let Some(evt) = input.next() {
            match evt {
                InputEvent::Keyboard(k) => match k {
                    KeyEvent::Ctrl(c) => match c {
                        'q' => break,
                        _ => ()
                    },
                    _ => ()
                },
                InputEvent::Mouse(m) => match m {
                    _ => ()
                },
                _ => ()
            }
        }
    }
}