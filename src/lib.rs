//! (imdaveho) TODO: ...

mod shared;

pub mod screen;
pub mod cursor;
pub mod output;
pub mod input;


#[cfg(unix)]
use libc::termios as Termios;

#[cfg(windows)]
use shared::Termios;


struct Tty<'t> {
    id: usize,
    meta: Vec<Metadata<'t>>,
    #[cfg(windows)]
    altscrn: Option<shared::Handle>,
}

struct Metadata<'m> {
    ORIGINAL_MODE: Termios,
    is_raw: bool,
    content: &'m str,
    #[cfg(windows)]
    markpos: Option<(i16, i16)>,
}

impl<'m> Metadata<'m> {
    fn new() -> Metadata<'m> {
        Metadata {
            ORIGINAL_MODE: output::get_mode().unwrap(),
            is_raw: false,
            content: &"",
            #[cfg(windows)]
            markpos: None,
        }
    }

    fn update_content(&mut self, content: &'m str) {
        self.content = content;
    }
}

// TODO: need to normalize between unix and windows
// with regards to Termios.
impl<'t> Tty<'t> {
    fn init() -> Tty<'t> {
        Tty {
            id: 0,
            meta: vec![Metadata::new()],
            #[cfg(windows)]
            altscrn: None,
        }
    }

    fn clear(&self, s: &str) {
        match s {
            "all" => {
                screen::clear(screen::Clear::All).unwrap();
                // (imdaveho) TODO: update self.
            },
            "currentln" => screen::clear(screen::Clear::CurrentLn).unwrap(),
            "cursorup" => screen::clear(screen::Clear::CursorUp).unwrap(),
            "cursordn" => screen::clear(screen::Clear::CursorDn).unwrap(),
            "newln" => screen::clear(screen::Clear::NewLn).unwrap(),
            _ => screen::clear(screen::Clear::All).unwrap(),
        }
    }

    fn size(&self) -> (u16, u16) {
        screen::size()
    }

    fn resize(&self, width: u16, height: u16) {
        screen::resize(width, height).unwrap();
    }

    fn switch(&mut self) {
        let m = Metadata::new();
        self.meta.push(m);
        self.id += 1;
        screen::enable_alt().unwrap();
    }

    fn main(&mut self) {
        screen::disable_alt().unwrap();
        self.id = 0;
        let m = &self.meta[0];
        // (imdaveho) TODO: implement a load buffer method...
    }

    fn switch_to(&mut self, id: usize) {
        self.id = id;
        let m = &self.meta[id];
        // (imdaveho) TODO: implement a load buffer method...
    }

    // (imdaveho) NOTE: removing scroll_up and scroll_down as
    // the unix native implementations are not really usable --
    // will consider adding it back in, if necessary.

    fn goto(&self, col: i16, row: i16) {
        cursor::goto(col, row).unwrap();
    }

    fn up(&self) {
        cursor::move_up(1).unwrap();
    }

    fn dn(&self) {
        cursor::move_down(1).unwrap();
    }

    fn left(&self) {
        cursor::move_left(1).unwrap();
    }

    fn right(&self) {
        cursor::move_right(1).unwrap();
    }

    fn dpad(&self, dir: &str, n: i16) {
        match dir {
            "up" => cursor::move_up(n).unwrap(),
            "dn" => cursor::move_down(n).unwrap(),
            "left" => cursor::move_left(n).unwrap(),
            "right" => cursor::move_right(n).unwrap(),
            _ => (),
        }
    }

    fn raw(&mut self) {
        output::enable_raw().unwrap();
        self.meta[self.id].is_raw = true;
    }

    fn cook(&mut self) {
        // "cooked" vs "raw" mode terminology from Wikipedia:
        // https://en.wikipedia.org/wiki/Terminal_mode
        // A terminal mode is one of a set of possible states of a
        // terminal or pseudo terminal character device in Unix-like
        // systems and determines how characters written to the terminal
        // are interpreted. In cooked mode data is preprocessed before
        // being given to a program, while raw mode passes the data as-is
        // to the program without interpreting any of the special characters.
        output::set_mode
            (&self.meta[self.id].ORIGINAL_MODE)
            .unwrap();
        self.meta[self.id].is_raw = false;
    }

    fn pos(&mut self) -> (i16, i16) {
        #[cfg(unix)] {
            if self.meta[self.id].is_raw {
                cursor::pos_raw().unwrap()
            } else {
                // unix needs to be raw to use pos()
                self.raw();
                let (col, row) = cursor::pos_raw().unwrap();
                // since the output was not in raw_mode before
                // we need to revert back to the cooked state
                self.cook();
                return (col, row);
            }
        }

        #[cfg(windows)] {
            cursor::pos().unwrap()
        }
    }

    #[cfg(unix)]
    fn mark(&self) {
        cursor::save_pos().unwrap()
    }

    #[cfg(unix)]
    fn load(&self) {
        cursor::load_pos().unwrap();
    }

    #[cfg(windows)]
    fn mark(&mut self) {
        let mut meta = self.meta[self.id];
        meta.markpos = Some(cursor::pos().unwrap());
    }

    #[cfg(windows)]
    fn load(&self) {
        let meta = self.meta[self.id];
        let (col, row) = meta.markpos.unwrap();
        self.goto(col, row);
    }

    fn hide_cursor(&self) {
        cursor::hide().unwrap();
    }

    fn show_cursor(&self) {
        cursor::show().unwrap();
    }

    fn read_char() {()}
    fn read_line() {()}
    fn read_async() {()}
    fn read_sync() {()}
    fn read_until_async() {()}
    fn enable_mouse_input() {()}
    fn disable_mouse_input() {()}
}
