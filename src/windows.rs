use crate::screen;
use crate::cursor;
use crate::output;
use crate::input;
use crate::shared::{ConsoleInfo, Handle};
use crate::{AsyncReader, SyncReader};

pub type Termios = u32;


pub struct Tty {
    id: usize,
    meta: Vec<Metadata>,
    original_mode: Termios,
    pub altscreen: Option<Handle>, // TODO: implement exit() where it cleans up before ending program
    reset_attrs: u16,
}

struct Metadata {
    is_raw_enabled: bool,
    is_mouse_enabled: bool,
    saved_position: Option<(i16, i16)>,
}

impl Tty {
    pub fn init() -> Tty {
        Tty {
            id: 0,
            meta: vec![Metadata {
                is_raw_enabled: false,
                is_mouse_enabled: false,
                saved_position: None,
            }],
            original_mode: output::get_mode().unwrap(),
            altscreen: None,
            reset_attrs: ConsoleInfo::of(
                &Handle::conout().unwrap()
            ).unwrap().attributes(),
        }
    }
}

pub fn clear(method: &str) {
    match method {
        "all" => {
            screen::clear(screen::Clear::All).unwrap();
            goto(0, 0);
        }
        "newln" => {
            let (col, row) = cursor::pos().unwrap();
            screen::clear(screen::Clear::NewLn).unwrap();
            goto(col, row);
        }
        "currentln" => {
            let (_, row) = cursor::pos().unwrap();
            screen::clear(screen::Clear::CurrentLn).unwrap();
            goto(0, row);
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
    // On Windows, the new buffer that is created
    // is a fresh instance to the defaults: where
    // raw mode and mouse mode are disabled.
    if tty.altscreen.is_none() {
        tty.altscreen = Some(
            Handle::buffer().unwrap());
    }

    if let Some(handle) = &tty.altscreen {
        handle.set_mode(&tty.original_mode).unwrap();
        if tty.id == 0 {
            // There is a single handle for the
            // alternate screen buffer; so only if 
            // you're on id == 0 or the main screen, 
            // do you need to enable the alternate.
            handle.show().unwrap();
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
    }
}

pub fn main(tty: &mut Tty) {
    if tty.id != 0 {
        // This function only works if the
        // User is not already on the main
        // screen buffer.
        let m = &tty.meta[0];
        let mode = &tty.original_mode;
        let stdout = Handle::stdout().unwrap();
        stdout.set_mode(mode).unwrap();
        tty.id = 0;
        screen::disable_alt().unwrap();
        
        if m.is_raw_enabled {
            output::enable_raw().unwrap();
        }

        if m.is_mouse_enabled {
            input::enable_mouse_input().unwrap();
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
            let mode = &tty.original_mode;
            if let Some(handle) = &tty.altscreen {
                handle.set_mode(mode).unwrap();
                tty.id = id;

                if m.is_raw_enabled {
                    output::enable_raw().unwrap();
                }

                if m.is_mouse_enabled {
                    input::enable_mouse_input().unwrap();
                }
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
    output::disable_raw().unwrap();
    m.is_raw_enabled = false;
}

pub fn enable_mouse(tty: &mut Tty) {
    let mut m = &mut tty.meta[tty.id];
    input::enable_mouse_input().unwrap();
    m.is_mouse_enabled = true;
}

pub fn disable_mouse(tty: &mut Tty) {
    let mut m = &mut tty.meta[tty.id];
    let handle = Handle::conin().unwrap();
    let mode = &tty.original_mode;
    handle.set_mode(mode).unwrap();
    if m.is_raw_enabled {
        output::enable_raw().unwrap();   
    }
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

pub fn pos() -> (i16, i16) {
    cursor::pos().unwrap()
}

pub fn mark(tty: &mut Tty) {
    tty.meta[tty.id].saved_position = Some(
        cursor::pos().unwrap()
    );
}

pub fn load(tty: &&Tty) {
    match tty.meta[tty.id].saved_position {
        Some(pos) => {
            goto(pos.0, pos.1);
        }
        None => ()
    }
}

pub fn hide_cursor() {
    cursor::hide().unwrap();
}

pub fn show_cursor() {
    cursor::show().unwrap();
}

pub fn read_char() -> char {
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

pub fn set_fg(tty: &Tty, col: &str) {
    let fg_col = output::Color::from(col);
    output::fg(fg_col, tty.reset_attrs).unwrap();
}

pub fn set_bg(tty: &Tty, col: &str) {
    let bg_col = output::Color::from(col);
    output::bg(bg_col, tty.reset_attrs).unwrap();
}

pub fn set_txsty(tx: &str) {
    let tx_sty = output::TextStyle::from(tx);
    output::txsty(tx_sty).unwrap();
}

pub fn set_fg_rgb(tty: &Tty, r: u8, g: u8, b: u8) {
    let fg_col = output::Color::Rgb{
        r: r,
        g: g,
        b: b,
    };
    output::fg(fg_col, tty.reset_attrs).unwrap();
}

pub fn set_bg_rgb(tty: &Tty, r: u8, g: u8, b: u8) {
    let bg_col = output::Color::Rgb{
        r: r,
        g: g,
        b: b,
    };
    output::bg(bg_col, tty.reset_attrs).unwrap();
}

pub fn set_fg_ansi(tty: &Tty, v: u8) {
    let fg_col = output::Color::AnsiValue(v);
    output::fg(fg_col, tty.reset_attrs).unwrap();
}

pub fn set_bg_ansi(tty: &Tty, v: u8) {
    let bg_col = output::Color::AnsiValue(v);
    output::bg(bg_col, tty.reset_attrs).unwrap();
}

pub fn set_style(tty: &Tty, fg: &str, bg: &str, tx: &str) {
    // The params fg is a single word, bg is 
    // also a single word, however the tx
    // param can be treated as a comma-separated
    // list of words that match the various text
    // styles that are supported: "bold", "dim",
    // "underline", "reverse", "hide", and "reset".
    output::set_style(fg, bg, tx, tty.reset_attrs).unwrap();
}

pub fn reset(tty: &Tty) {
    output::reset(tty.reset_attrs).unwrap();
}

pub fn writeout(s: &str) {
    output::writeout(s).unwrap();
}