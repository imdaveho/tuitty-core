#[cfg(windows)]
extern crate tuitty;

#[cfg(windows)]
use tuitty::common::unicode::{grapheme::*, wcwidth::*};

#[cfg(windows)]
use tuitty::common::enums::{Style, Color};

#[cfg(windows)]
extern crate winapi;

// #[cfg(windows)]
// use std::mem::zeroed;
#[cfg(windows)]
use std::thread;
#[cfg(windows)]
use std::time::Duration;
#[cfg(windows)]
use std::ffi::OsStr;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

#[cfg(windows)]
use winapi::um::wincon::{
    WriteConsoleOutputAttribute,
    CHAR_INFO, COORD, SMALL_RECT, FOREGROUND_RED, BACKGROUND_BLUE,BACKGROUND_GREEN
};

#[cfg(windows)]
use winapi::shared::minwindef::WORD;

#[cfg(windows)]
fn main() {
    let handle = tuitty::terminal::actions::win32::Handle::stdout().unwrap();
    let mode = handle.get_mode().unwrap();
    let info = tuitty::terminal::actions::win32::ConsoleInfo::of(&handle).unwrap();
    let attrs = info.attributes();

    let altern = tuitty::terminal::actions::win32::Handle::buffer().unwrap();
    tuitty::terminal::actions::win32::enable_alt(&altern, &mode, false);

    let _ = tuitty::terminal::actions::win32::set_fg(Color::Red, attrs, false);

    let attrs_mod = info.attributes();

    let _ = tuitty::terminal::actions::win32::reset_styles(attrs, false);



    let example_output = "qwertyuiopasd⚠️fghjkl😀園v明nmQWE👪RTY👨‍👩‍👧UIOPASDFGHJKLZXCVBNM";

    tuitty::terminal::actions::win32::goto(0, 0, false);
    // tuitty::terminal::actions::win32::printf(example_output, false);
    // Size within sub-terminal (eg. windows terminal or cmder) is incorrect...
    let size = info.terminal_size();
    println!("X: {}, Y: {}", size.0, size.1);
    let size = (size.0 + 1, size.1 + 1);

    let mut screen_str = String::new();
    let alpha = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","0","1","2","3","4","5","6","7","8","9","0"];
    for i in 0..size.1 {
        screen_str.push_str(&format!("{}", alpha[i as usize]).repeat(size.0 as usize));
    }
    tuitty::terminal::actions::win32::printf(&screen_str, false);

    thread::sleep(Duration::from_millis(2000));

    let mut innerbuf: Vec<Option<Vec<u16>>> = vec![None; (size.0 * size.1) as usize];

    let first_row = 0;
    let first_col = size.0 as usize;
    for i in first_row..first_col {
        innerbuf[i] = Some(vec!['a' as u16]);
    }

    let last_row = (size.0 * (size.1 - 1)) as usize;
    let last_col = last_row + size.0 as usize;
    for i in last_row..last_col {
        innerbuf[i] = Some(vec!['z' as u16]);
    }

    let index = size.0 as usize / 2;
    innerbuf[index] = Some(OsStr::new("👪").encode_wide().collect());
    innerbuf[index + 1] = Some(vec![]);

    let mut output: Vec<u16> = vec![];
    for v in &innerbuf {
        match v {
            Some(u) => { for i in u { output.push(*i) }},
            None => output.push('-' as u16),
        }
    }

    tuitty::terminal::actions::win32::goto(0, 0, false);
    let str_output = String::from_utf16(&output).unwrap();
    tuitty::terminal::actions::win32::printf(&str_output, false);

    // thread::sleep(Duration::from_millis(2000));

    let mut attr_cnt = 0;
    let attr_res = unsafe {
        // let mut attrss: [WORD; 2] = [0, 0];
        // attrss[0] = FOREGROUND_RED | BACKGROUND_BLUE;
        // attrss[1] = BACKGROUND_BLUE | FOREGROUND_RED;
        let attrss = vec![(0x0020 as WORD | 0x0004 as WORD), attrs, (0x0020 as WORD | 0x0004 as WORD)];
        WriteConsoleOutputAttribute(
            altern.0,
            attrss.as_ptr() as *const WORD,
            // &(BACKGROUND_GREEN | FOREGROUND_RED),
            3,
            COORD {X: 2, Y: 3},
            &mut attr_cnt
        )
    };

    if attr_res == 0 {
        tuitty::terminal::actions::win32::cook();
        tuitty::terminal::actions::win32::disable_alt(false);
        panic!(format!("Something went wrong applying attr to buffer - response: {}", attr_res));
    }


    thread::sleep(Duration::from_millis(5000));

    tuitty::terminal::actions::win32::cook();
    tuitty::terminal::actions::win32::disable_alt(false);
}