extern crate tuitty;

use std::thread;
use std::time::Duration;

use tuitty::common::{
    traits::{
        TerminalCursor, TerminalFormatter, TerminalInput,
        TerminalModifier, TerminalSwitcher, TerminalWriter
    }, enums::{ Color, Effect },
    unicode::{ grapheme::*, wcwidth::* }
};

use tuitty::terminal;
use tuitty::interface;

use std::io::{ stdin, stdout, Result, BufRead, Write };

#[cfg(windows)]
use tuitty::terminal::wincon::Handle;

fn main() {
    let content = "👨‍👩‍👧|👨‍🚀|🤦‍♀️|褐色|क्‍ष|👧🏿|☆|\u{200d}\u{fe0f}|寬\u{2060}帶|fa\x00mily|family|";
    let groupe = UnicodeGraphemes::graphemes(content, true).collect::<Vec<&str>>();
    println!("{:?}", groupe);
    let mut t = terminal::Terminal::init();
    t.printf(content);
    let wsize = t.screen_size();
    t.printf(&format!("\n{}, {}\n", wsize.0, wsize.1));
    t.resize(86, 30);
    let wsizea = t.screen_size();
    t.printf(&format!("{}, {}", wsizea.0, wsizea.1));

    t.switch();
    t.hide_cursor();
    t.raw();
    t.enable_mouse();
    let mut alertbox = interface::AlertBox::new(&mut t,
        "We should now be able to create functions that accept strings whether they are &str, String or event reference counted. We are also able to create structs that are able to have variables that are references. The lifetime of the struct is linked to those referenced variables to make sure that the struct does not outlive the referenced variable and caused bad things to happen in our program. We also have a initial und".to_string());
    alertbox.render();
    let res = alertbox.handle();
    // t.cook();
    t.to_main();
    t.printf(&format!("{}", res));

    // let stdout = Handle::stdout().expect("Error with Stdout");
    // let mode = stdout.get_mode().expect("Error getting mode with Stdout");
    // let mask = 0x0002 | 0x0002;
    // thread::sleep(Duration::from_millis(5000));
}