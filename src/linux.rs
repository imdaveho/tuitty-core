use crate::screen;
use crate::cursor;
use crate::output;
use crate::input;
use crate::shared::{ConsoleInfo, Handle};
use crate::{AsyncReader, SyncReader};

use libc::termios as Termios;


struct Tty {
    id: usize,
    meta: Vec<Metadata>,
    original_mode: Termios,
}

struct Metadata {
    is_raw_enabled: bool,
    is_mouse_enabled: bool,
}

impl Tty {
    pub fn init() {
        Tty {
            id: 0,
            meta: vec![Metadata {
                is_raw_enabled: false,
                is_mouse_enabled: false,
            }],
            original_mode: output::get_mode().unwrap(),
        }
    }
}

pub fn clear(method: &str) {
    match method {
        "all" => {
            screen::clear(screen::Clear::All).unwrap();
            cursor::goto(0, 0);
        }
        "newln" => {
            screen::clear(screen::Clear::NewLn).unwrap();
        }
        "currentln" => {
            screen::clear(screen::Clear::CurrentLn).unwrap();
        }
        "cursorup" => {
            screen::clear(screen::Clear::CursorUp).unwrap();
        }
        "cursordn" => {
            screen::clear(screen::Clear::CursorDn).unwrap();
        }
        _ => ()
    }
}

pub fn size() -> (i16, i16) {
    screen::size()
}

pub fn resize(w: i16, h: i16) {
    screen::resize(w, h).unwrap();
}

pub fn switch(tty: &mut Tty) {
    // This function is used primarily to create
    // a new "screen" by creating some Metadata
    // that reflects any changes in the mode as
    // with enabling raw input or mouse events.
    // On Unix, we will need to reset the screen
    // to disable raw mode and mouse events.
    if tty.id == 0 {
        // There is no point to switch
        // if you're on another screen
        // since Unix systems only have
        // a single "alternate screen".
        screen::enable_alt().unwrap();
    }
    // Create the new `Metadata` to describe the
    // new screen.
    let metas = &mut tty.meta;
    let rstate = metas[tty.id].is_raw_enabled;
    let mstate = metas[tty.id].is_mouse_enabled;
    metas.push(Metadata{
        is_raw_enabled: rstate,
        is_mouse_enabled: mstate,
        saved_position: None,
    });
    tty.id = tty.meta.len() - 1;
    // Ensure that raw and mouse modes are disabled.
    cook(tty);
    input::disable_mouse_input().unwrap();
}

pub fn main(tty: &mut Tty) {
    if tty.id != 0 {
        // This function only works if the
        // User is not already on the main
        // screen buffer.
        let m = &tty.meta[0];
        tty.id = 0;
        screen::disable_alt().unwrap();

        if m.is_raw_enabled {
            output::enable_raw().unwrap();
        } else {
            cook(tty);
        }

        if m.is_mouse_enabled {
            input::enable_mouse_input().unwrap();
        } else {
            input::disable_mouse_input().unwrap();
        }
    }
}

pub fn switch_to(tty: &mut Tty, id: usize) {
    // If the id and the current id are the same, well,
    // there is nothing more to do, you're already on
    // the active screen buffer.
    if id != tty.id {
        if id == 0 {
            // Switch to the main screen.
            main(tty);
        } else {
            // Restore the mode of the alternate
            // screen that you're switching to.
            let m = &tty.meta[id];
            tty.id = id;
            if m.is_raw_enabled {
                output::enable_raw().unwrap();
            } else {
                cook(tty);
            }

            if m.is_mouse_enabled {
                input::enable_mouse_input().unwrap();
            } else {
                input::disable_mouse_input().unwrap();
            }
        }
    }
    // NOTE: this only switches the screen buffer and updates
    // the settings. Updating the content that will be passed
    // in and rendered, that is up to the implementation.
}

pub fn raw(tty: &mut Tty) {
    let mut m = &mut tty.meta[tty.id];
    output::enable_raw().unwrap();
    m.is_raw_enabled = true;
}

pub fn cook(tty: &mut Tty) {
    // "cooked" vs "raw" mode terminology from Wikipedia:
    // https://en.wikipedia.org/wiki/Terminal_mode
    // A terminal mode is one of a set of possible states of a
    // terminal or pseudo terminal character device in Unix-like
    // systems and determines how characters written to the terminal
    // are interpreted. In cooked mode data is preprocessed before
    // being given to a program, while raw mode passes the data as-is
    // to the program without interpreting any of the special characters.
    let mut m = &mut tty.meta[tty.id];
    output::set_mode(m.mode).unwrap();
    m.is_raw_enabled = false;
}

pub fn enable_mouse(tty: &mut Tty) {
    let mut m = &mut tty.meta[tty.id];
    input::enable_mouse_input().unwrap();
    m.is_mouse_enabled = true;
}

pub fn disable_mouse(tty: &mut Tty) {
    let mut m = &mut tty.meta[tty.id];
    input::disable_mouse_input().unwrap();
    m.is_mouse_enabled = false;
}

pub fn goto(col: i16, row: i16) {
    cursor::goto(col, row).unwrap();
}

pub fn up() {
    cursor::move_up(1).unwrap();
}

pub fn dn() {
    cursor::move_down(1).unwrap();
}

pub fn left() {
    cursor::move_left(1).unwrap();
}

pub fn right() {
    cursor::move_right(1).unwrap();
}

pub fn dpad(dir: &str, n: i16) {
    // Case-insensitive.
    let d = dir.to_lowercase();
    if n > 0 {
        match d.as_str() {
            "up" => cursor::move_up(n).unwrap(),
            "dn" => cursor::move_down(n).unwrap(),
            "left" => cursor::move_left(n).unwrap(),
            "right" => cursor::move_right(n).unwrap(),
            _ => ()
        }
    } 
}

pub fn pos(tty: &mut Tty) {
    if &tty.meta[tty.id].is_raw {
        cursor::pos_raw().unwrap()
    } else {
        // Unix needs to be raw to use pos().
        raw(tty);
        let (col, row) = cursor::pos_raw().unwrap();
        // Since the output was not in raw_mode before
        // we need to revert back to the cooked state.
        cook(tty);
        return (col, row);
    }
}

pub fn mark() {
    cursor::save_pos().unwrap()
}

pub fn load() {
    cursor::load_pos().unwrap()
}

pub fn hide_cursor() {
    cursor::hide().unwrap();
}

pub fn show_cursor() {
    cursor::show().unwrap();
}

pub fn read_char() {
    input::read_char().unwrap()
}

pub fn read_sync() -> SyncReader {
    input::read_sync()
}

pub fn read_async() -> AsyncReader {
    input::read_async()
}

pub fn read_until_async(delimiter: u8) -> AsyncReader {
    input::read_until_async(delimiter)
}

pub fn set_fg(col: &str) {
    let fg_col = output::Color::from(col);
    output::fg(fg_col).unwrap();
}

pub fn set_bg(col: &str) {
    let bg_col = output::Color::from(col);
    output::bg(bg_col).unwrap();
}

pub fn set_txsty(tx: &str) {
    let tx_sty = output::TextStyle::from(tx);
    output::txsty(tx_sty).unwrap();
}

pub fn set_fg_rgb(r: u8, g: u8, b: u8) {
    let fg_col = output::Color::Rgb{
        r: r,
        g: g,
        b: b,
    };
    output::fg(fg_col).unwrap();
}

pub fn set_bg_rgb(r: u8, g: u8, b: u8) {
    let bg_col = output::Color::Rgb{
        r: r,
        g: g,
        b: b,
    };
    output::bg(bg_col).unwrap();
}

pub fn set_fg_ansi(v: u8) {
    let fg_col = output::Color::AnsiValue(v);
    output::fg(fg_col).unwrap();
}

pub fn set_bg_ansi(v: u8) {
    let bg_col = output::Color::AnsiValue(v);
    output::bg(bg_col).unwrap();
}

pub fn set_style(fg: &str, bg: &str, tx: &str) {
    // The params fg is a single word, bg is 
    // also a single word, however the tx
    // param can be treated as a comma-separated
    // list of words that match the various text
    // styles that are supported: "bold", "dim",
    // "underline", "reverse", "hide", and "reset".
    output::set_style(fg, bg, tx).unwrap();
}

pub fn reset() {
    output::reset().unwrap();
}

pub fn writeout(s: &str) {
    output::writeout(s).unwrap();
}