#[cfg(windows)]
extern crate tuitty;

#[cfg(windows)]
use tuitty::common::enums::{Color, foreground, background};

#[cfg(windows)]
extern crate winapi;

// #[cfg(windows)]
// use std::mem::zeroed;
#[cfg(windows)]
use std::thread;
#[cfg(windows)]
use std::time::Duration;


#[cfg(windows)]
fn main() {
    let handle = tuitty::terminal::actions::win32::Handle::stdout().unwrap();
    let mode = handle.get_mode().unwrap();
    let info = tuitty::terminal::actions::win32::ConsoleInfo::of(&handle).unwrap();

    let vte = tuitty::terminal::actions::win32::is_ansi_enabled();
    println!("{}", vte);

    let altern = tuitty::terminal::actions::win32::Handle::buffer().unwrap();
    tuitty::terminal::actions::win32::enable_alt(&altern, &mode, false);

    // let _ = tuitty::terminal::actions::win32::set_fg(Color::Red, attrs, false);
    // let attrs_mod = info.attributes();
    // let _ = tuitty::terminal::actions::win32::reset_styles(attrs, false);
    // let example_output = "qwertyuiopasd⚠️fghjkl😀園v明nmQWE👪RTY👨‍👩‍👧UIOPASDFGHJKLZXCVBNM";
    tuitty::terminal::actions::win32::goto(0, 0, false);
    // tuitty::terminal::actions::win32::printf(example_output, false);
    let size = info.terminal_size();
    // println!("X: {}, Y: {}", size.0, size.1);
    let size = (size.0 + 1, size.1 + 1);

    for i in 0..2 {
        let mut screen_str = String::new();
        let mut alpha = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","0","1","2","3","4","5","6","7","8","9","0"];
        if i > 0 {
            alpha.reverse();
        }
        for i in 0..size.1 {
            screen_str.push_str(&format!("{}", alpha[i as usize]).repeat(size.0 as usize));
        }
        tuitty::terminal::actions::win32::printf(&screen_str, false);

        // thread::sleep(Duration::from_millis(2000));

        tuitty::terminal::actions::win32::goto(0, 12, false);
        let emoji_string = "👪".repeat(size.0 as usize / 2);
        tuitty::terminal::actions::win32::printf(&emoji_string, false);

        // thread::sleep(Duration::from_millis(2000));

        let default_attr = info.attributes();
        let mut style_updates: Vec<(u16, usize, usize)> = vec![];

        // tuitty::terminal::actions::win32::set_fg(Color::Red, default_attr, false);
        // tuitty::terminal::actions::win32::set_bg(Color::Blue, default_attr, false);
        let mut current = default_attr;
        current = foreground(Color::Red, current, default_attr);
        current = background(Color::Blue, current, default_attr);
        let start: usize;
        let finish: usize;
        if i > 0 {
            start = 96;
            finish = 85 * 2 + 46;
        } else {
            start = 33;
            finish = 85 * 2 + 18;
        }
        style_updates.push((current, start, finish));
        tuitty::terminal::actions::win32::reset_styles(default_attr, false);
        current = default_attr;

        tuitty::terminal::actions::win32::set_bg(Color::Green, default_attr, false);
        current = background(Color::Green, current, default_attr);
        let start: usize;
        let finish: usize;
        if i > 0 {
            start = 85 * 9;
            finish = 85 * 12;
        } else {
            start = 85 * 4 + 2;
            finish = 85 * 6 + 8;
        }
        style_updates.push((current, start, finish));
        tuitty::terminal::actions::win32::reset_styles(default_attr, false);
        // current = default_attr;

        // loop to run through the style updates
        for s in style_updates {
            // let (attr, start, finish) = s;
            // let mut count = 0;
            // let length = finish - start;
            // let col = start as i16 % (size.0 - 1);
            // let row = start as i16 / (size.0 - 1);
            // let styles: Vec<WORD> = vec![attr; length];

            // let err = unsafe {
            //     WriteConsoleOutputAttribute(
            //         altern.0,
            //         styles.as_ptr() as *const WORD,
            //         length as u32,
            //         COORD { X: col, Y: row},
            //         &mut count
            //     )
            // };
            // if err == 0 {
            //     tuitty::terminal::actions::win32::cook();
            //     tuitty::terminal::actions::win32::disable_alt(false);
            //     panic!(format!("Something went wrong applying attr to buffer - response: {}", err));
            // }
            // // thread::sleep(Duration::from_millis(2000));

            let (word, start, finish) = s;
            let length = (finish - start) as u32;
            let coord = (
                finish as i16 % (size.0 - 1),
                finish as i16 / (size.0 - 1)
            );            
            tuitty::terminal::actions::win32::set_attrib(word, length, coord);
            thread::sleep(Duration::from_millis(2000));
        }
        // thread::sleep(Duration::from_millis(2000));
    }

    thread::sleep(Duration::from_millis(5000));

    
    tuitty::terminal::actions::win32::cook();
    tuitty::terminal::actions::win32::disable_alt(false);
}