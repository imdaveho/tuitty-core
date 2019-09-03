//! This module consolidates the parts that make up a TTY into a single struct
//! and exposes the methods that work specifically on Unix systems.

use super::screen;
use super::cursor;
use super::output;
use super::input;
use super::shared::{write_ansi, flush_ansi};
use super::{AsyncReader, SyncReader, Termios};


pub struct Tty {
    id: usize,
    meta: Vec<Metadata>,
    original_mode: Termios,
}

pub struct Metadata {
    is_raw_enabled: bool,
    is_mouse_enabled: bool,
}

impl Tty {

    pub fn init() -> Tty {
        Tty {
            id: 0,
            meta: vec![Metadata {
                is_raw_enabled: false,
                is_mouse_enabled: false,
            }],
            original_mode: output::ansi::get_mode().unwrap(),
        }
    }

    pub fn exit(&mut self) {
        self.to_main();
        output::ansi::set_mode(&self.original_mode).unwrap();
        write_ansi(&cursor::ansi::show());
        write_ansi("\n\r");
        self.meta.clear();
    }

    pub fn size(&self) -> (i16, i16) {
        // Unix specific: depends on `libc`.
        screen::ansi::size()
    }

    // "cooked" vs "raw" mode terminology from Wikipedia:
    // https://en.wikipedia.org/wiki/Terminal_mode
    // A terminal mode is one of a set of possible states of a terminal or
    // pseudo terminal character device in Unix-like systems and determines how
    // characters written to the terminal are interpreted. In cooked mode data
    // is preprocessed before being given to a program, while raw mode passes
    // data as-is to the program without interpreting any special characters.
    pub fn raw(&mut self) {
        // Unix specific: depends on `libc::Termios`.
        let mut m = &mut self.meta[self.id];
        output::ansi::enable_raw().unwrap();
        m.is_raw_enabled = true;
    }

    pub fn cook(&mut self) {
        // Unix specific: depends on `libc::Termios`.
        let mut m = &mut self.meta[self.id];
        output::ansi::set_mode(&self.original_mode).unwrap();
        m.is_raw_enabled = false;
    }

    // Input module functions are OS-specific.
    // * enable/disable mouse
    // * read_char/sync/async/until_async

    pub fn enable_mouse(&mut self) {
        let mut m = &mut self.meta[self.id];
        write_ansi(&input::ansi::enable_mouse_mode());
        m.is_mouse_enabled = true;
    }

    pub fn disable_mouse(&mut self) {
        let mut m = &mut self.meta[self.id];
        write_ansi(&input::ansi::disable_mouse_mode());
        m.is_mouse_enabled = false;
    }

    pub fn read_char(&self) -> char {
        input::ansi::read_char().unwrap()
    }

    pub fn read_sync(&self) -> SyncReader {
        input::ansi::read_sync()
    }

    pub fn read_async(&self) -> AsyncReader {
        input::ansi::read_async()
    }

    pub fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        input::ansi::read_until_async(delimiter)
    }


    pub fn clear(&mut self, method: &str) {
        match method {
            "all" => {
                write_ansi(&screen::ansi::clear(screen::Clear::All));
                self.goto(0, 0);
            }
            "newln" => {
                write_ansi(&screen::ansi::clear(screen::Clear::NewLn));
            }
            "currentln" => {
                write_ansi(&screen::ansi::clear(screen::Clear::CurrentLn));
            }
            "cursorup" => {
                write_ansi(&screen::ansi::clear(screen::Clear::CursorUp));
            }
            "cursordn" => {
                write_ansi(&screen::ansi::clear(screen::Clear::CursorDn));
            }
            _ => ()
        }
    }

    pub fn resize(&mut self, w: i16, h: i16) {
        write_ansi(&screen::ansi::resize(w, h));
    }


    pub fn switch(&mut self) {
        // In order to support multiple "screens", this function creates a new
        // Metadata entry which stores any screen specific changes that a User
        // might want to be restored when switching between screens.
        if self.id == 0 {
            // There is no point to switch if you're on another screen
            // since ANSI terminals provide a single "alternate screen".
            write_ansi(&screen::ansi::enable_alt());
        }
        // Add new `Metadata` for the new screen.
        self._add_metadata();
        self.id = self.meta.len() - 1;
        // Explicitly set default screen settings.
        self.cook();
        self.disable_mouse();
    }


    pub fn to_main(&mut self) {
        // Only execute if the User is not on the main screen buffer.
        if self.id != 0 {
            let metas = &self.meta;
            let rstate = metas[0].is_raw_enabled;
            let mstate = metas[0].is_mouse_enabled;
            self.id = 0;
            write_ansi(&screen::ansi::disable_alt());

            if rstate {
                self.raw();
            } else {
                self.cook();
            }

            if mstate {
                self.enable_mouse();
            } else {
                self.disable_mouse();
            }
        }
    }


    pub fn switch_to(&mut self, id: usize) {
        // NOTE: this only switches the screen buffer and updates the settings.
        // Updating the content that will be passed in and rendered, that is
        // up to the implementation.

        // If the id and the current id are the same, well, there is nothing to
        // do, you're already on the active screen buffer.
        if id != self.id {
            if id == 0 {
                // Switch to the main screen.
                self.to_main();
            } else {
                let metas = &self.meta;
                let rstate = metas[id].is_raw_enabled;
                let mstate = metas[id].is_mouse_enabled;
                self.id = id;
                if rstate {
                    self.raw();
                } else {
                    self.cook();
                }

                if mstate {
                    self.enable_mouse();
                } else {
                    self.disable_mouse();
                }
            }
        }
    }

    pub fn goto(&mut self, col: i16, row: i16) {
        write_ansi(&cursor::ansi::goto(col, row));
    }

    pub fn up(&mut self) {
        write_ansi(&cursor::ansi::move_up(1));
    }

    pub fn dn(&mut self) {
        write_ansi(&cursor::ansi::move_down(1));
    }

    pub fn left(&mut self) {
        write_ansi(&cursor::ansi::move_left(1));
    }

    pub fn right(&mut self) {
        write_ansi(&cursor::ansi::move_right(1));
    }

    pub fn dpad(&mut self, dir: &str, n: i16) {
        // Case-insensitive.
        let d = dir.to_lowercase();
        if n > 0 {
            match d.as_str() {
                "up" => {
                    write_ansi(&cursor::ansi::move_up(n));
                },
                "dn" => {
                    write_ansi(&cursor::ansi::move_down(n));
                },
                "left" => {
                    write_ansi(&cursor::ansi::move_left(n));
                },
                "right" => {
                    write_ansi(&cursor::ansi::move_right(n));
                },
                _ => ()
            };
        }
    }

    pub fn pos(&mut self) -> (i16, i16) {
        if self.meta[self.id].is_raw_enabled {
            cursor::ansi::pos_raw().unwrap()
        } else {
            self.raw();
            let (col, row) = cursor::ansi::pos_raw().unwrap();
            self.cook();
            return (col, row);
        }
    }

    pub fn mark(&mut self) {
        write_ansi(&cursor::ansi::save_pos());
    }

    pub fn load(&mut self) {
        write_ansi(&cursor::ansi::load_pos());
    }

    pub fn hide_cursor(&mut self) {
        write_ansi(&cursor::ansi::hide());
    }

    pub fn show_cursor(&mut self) {
        write_ansi(&cursor::ansi::show());
    }

    pub fn set_fg(&mut self, color: &str) {
        let fg_col = output::Color::from(color);
        write_ansi(&output::ansi::set_fg(fg_col));
    }

    pub fn set_bg(&mut self, color: &str) {
        let bg_col = output::Color::from(color);
        write_ansi(&output::ansi::set_bg(bg_col));
    }

    pub fn set_tx(&mut self, style: &str) {
        let tstyle = output::TextStyle::from(style);
        write_ansi(&output::ansi::set_tx(tstyle));
    }

    pub fn set_fg_rgb(&mut self, r: u8, g:u8, b: u8) {
        let fg_col = output::Color::Rgb{
            r: r,
            g: g,
            b: b,
        };
        write_ansi(&output::ansi::set_fg(fg_col));
    }

    pub fn set_bg_rgb(&mut self, r: u8, g:u8, b: u8) {
        let bg_col = output::Color::Rgb{
            r: r,
            g: g,
            b: b,
        };
        write_ansi(&output::ansi::set_bg(bg_col));
    }

    pub fn set_fg_ansi(&mut self, v: u8) {
        let fg_col = output::Color::AnsiValue(v);
        write_ansi(&output::ansi::set_fg(fg_col));
    }

    pub fn set_bg_ansi(&mut self, v: u8) {
        let bg_col = output::Color::AnsiValue(v);
        write_ansi(&output::ansi::set_bg(bg_col));
    }

    pub fn set_style(&mut self, fg: &str, bg: &str, style: &str) {
        // The params fg is a single word, bg is also a single word, however
        // the tx param can be treated as a comma-separated list of words that
        // match the various text styles that are supported: "bold", "dim",
        // "underline", "reverse", "hide", and "reset".
        write_ansi(&output::ansi::set_all(fg, bg, style));
    }

    pub fn reset(&mut self) {
        write_ansi(&output::ansi::reset());
    }

    pub fn write(&mut self, s: &str) {
        write_ansi(&output::ansi::writeout(s));
    }

    pub fn flush(&mut self) {
        // ANSI-only
        flush_ansi();
    }

    // pub fn paint() {
    //     // write with colors and styles
    // }

    // pub fn render() {
    //     // write from a template
    // }

    // pub fn intellisense() {
    //     // write from a set of rules
    //     // eg. syntax highlighting
    // }



    fn _add_metadata(&mut self) {
        let metas = &mut self.meta;
        let rstate = metas[self.id].is_raw_enabled;
        let mstate = metas[self.id].is_mouse_enabled;
        metas.push(Metadata{
            is_raw_enabled: rstate,
            is_mouse_enabled: mstate,
        });
    }
}
