//! This module consolidates the parts that make up a TTY into a single struct
//! and exposes the methods that work specifically on Unix systems.

use super::cursor;
use super::input;
use super::screen::{self, Clear::*};
use super::output::{self, Color, Format};
use super::shared::{ansi_write, ansi_flush};
use super::{AsyncReader, SyncReader, Termios};


pub struct Tty {
    index: usize,
    metas: Vec<Metadata>,
    original_mode: Termios,
    autoflush: bool,
}

pub struct Metadata {
    is_raw_enabled: bool,
    is_mouse_enabled: bool,
    is_cursor_visible: bool,
}

struct Cell {
    ch: char,
    fg: Color,
    bg: Color,
    fmts: Vec<Format>,
}


impl Tty {
    pub fn init() -> Tty {
        Tty {
            index: 0,
            metas: vec![Metadata {
                is_raw_enabled: false,
                is_mouse_enabled: false,
                is_cursor_visible: true,
            }],
            original_mode: output::ansi::get_mode().unwrap(),
            autoflush: false,
        }
    }

    pub fn terminate(&mut self) {
        self.to_main();
        output::ansi::set_mode(&self.original_mode).unwrap();
        ansi_write(&cursor::ansi::show(), false);
        ansi_write("\r", true);
        self.metas.clear();
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
        let mut m = &mut self.metas[self.index];
        output::ansi::enable_raw().unwrap();
        m.is_raw_enabled = true;
    }

    pub fn cook(&mut self) {
        // Unix specific: depends on `libc::Termios`.
        let mut m = &mut self.metas[self.index];
        output::ansi::set_mode(&self.original_mode).unwrap();
        m.is_raw_enabled = false;
    }

    // Input module functions are OS-specific.
    // * enable/disable mouse
    // * read_char/sync/async/until_async

    pub fn enable_mouse(&mut self) {
        let mut m = &mut self.metas[self.index];
        ansi_write(&input::ansi::enable_mouse_mode(), self.autoflush);
        m.is_mouse_enabled = true;
    }

    pub fn disable_mouse(&mut self) {
        let mut m = &mut self.metas[self.index];
        ansi_write(&input::ansi::disable_mouse_mode(), self.autoflush);
        m.is_mouse_enabled = false;
        if self.autoflush { self.flush() }
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
                ansi_write(&screen::ansi::clear(All), self.autoflush);
                self.goto(0, 0);
            }
            "newln" => {
                ansi_write(&screen::ansi::clear(NewLn), self.autoflush);
            }
            "currentln" => {
                ansi_write(&screen::ansi::clear(CurrentLn), self.autoflush);
                let (_, row) = self.pos();
                self.goto(0, row);
            }
            "cursorup" => {
                ansi_write(&screen::ansi::clear(CursorUp), self.autoflush);
            }
            "cursordn" => {
                ansi_write(&screen::ansi::clear(CursorDn), self.autoflush);
            }
            _ => ()
        }
    }

    pub fn resize(&mut self, w: i16, h: i16) {
        // NOTE (imdaveho): this method must call `flush`
        // otherwise nothing happens.
        ansi_write(&screen::ansi::resize(w, h), true);
    }

    pub fn manual(&mut self) {
        self.autoflush = false;
    }

    pub fn automatic(&mut self) {
        self.autoflush = true;
    }

    pub fn switch(&mut self) {
        // In order to support multiple "screens", this function creates a new
        // Metadata entry which stores any screen specific changes that a User
        // might want to be restored when switching between screens.
        if self.index == 0 {
            // There is no point to switch if you're on another screen
            // since ANSI terminals provide a single "alternate screen".
            ansi_write(&screen::ansi::enable_alt(), false);
        }
        // Add new `Metadata` for the new screen.
        self._add_metadata();
        self.index = self.metas.len() - 1;
        // Prevent multiple `flush()` calls due to `autoflush` setting.
        let autoflush = self.autoflush;
        if self.autoflush { self.manual() }
        // Explicitly set default screen settings.
        self.cook();
        self.disable_mouse();
        self.show_cursor();
        self.goto(0, 0);
        // Revert back to previous `autoflush` configuration.
        if autoflush { self.flush(); self.automatic() }
    }


    pub fn to_main(&mut self) {
        // Only execute if the User is not on the main screen buffer.
        if self.index != 0 {
            let metas = &self.metas;
            let rstate = metas[0].is_raw_enabled;
            let mstate = metas[0].is_mouse_enabled;
            let cstate = metas[0].is_cursor_visible;
            self.index = 0;
            ansi_write(&screen::ansi::disable_alt(), false);
            // Prevent multiple `flush()` calls due to `autoflush` setting.
            let autoflush = self.autoflush;
            if self.autoflush { self.manual() }

            if rstate { self.raw() }
            else { self.cook() }

            if mstate { self.enable_mouse() }
            else { self.disable_mouse() }

            if cstate { self.show_cursor() }
            else { self.hide_cursor() }

            // Revert back to previous `autoflush` configuration.
            if autoflush { self.flush(); self.automatic() }
        }
    }


    pub fn switch_to(&mut self, index: usize) {
        // NOTE: this only switches the screen buffer and updates the settings.
        // Updating the content that will be passed in and rendered, that is
        // up to the implementation.

        // If the id and the current id are the same, well, there is nothing to
        // do, you're already on the active screen buffer.
        if index != self.index {
            if index == 0 {
                // Switch to the main screen.
                self.to_main();
            } else {
                let metas = &self.metas;
                let rstate = metas[index].is_raw_enabled;
                let mstate = metas[index].is_mouse_enabled;
                let cstate = metas[index].is_cursor_visible;
                self.index = index;
                // Prevent multiple `flush()` calls due to `autoflush` setting.
                let autoflush = self.autoflush;
                if self.autoflush { self.manual() }

                if rstate { self.raw() }
                else { self.cook() }

                if mstate { self.enable_mouse() }
                else { self.disable_mouse() }

                if cstate { self.show_cursor() }
                else { self.hide_cursor() }

                // Revert back to previous `autoflush` configuration.
                if autoflush { self.flush(); self.automatic() }
            }
        }
    }

    pub fn goto(&mut self, col: i16, row: i16) {
        ansi_write(&cursor::ansi::goto(col, row), self.autoflush);
    }

    pub fn up(&mut self) {
        ansi_write(&cursor::ansi::move_up(1), self.autoflush);
    }

    pub fn dn(&mut self) {
        ansi_write(&cursor::ansi::move_down(1), self.autoflush);
    }

    pub fn left(&mut self) {
        ansi_write(&cursor::ansi::move_left(1), self.autoflush);
    }

    pub fn right(&mut self) {
        ansi_write(&cursor::ansi::move_right(1), self.autoflush);
    }

    pub fn dpad(&mut self, dir: &str, n: i16) {
        // Case-insensitive.
        let d = dir.to_lowercase();
        if n > 0 {
            match d.as_str() {
                "up" => {
                    ansi_write(&cursor::ansi::move_up(n), self.autoflush);
                },
                "dn" => {
                    ansi_write(&cursor::ansi::move_down(n), self.autoflush);
                },
                "left" => {
                    ansi_write(&cursor::ansi::move_left(n), self.autoflush);
                },
                "right" => {
                    ansi_write(&cursor::ansi::move_right(n), self.autoflush);
                },
                _ => ()
            };
        }
    }

    pub fn pos(&mut self) -> (i16, i16) {
        if self.metas[self.index].is_raw_enabled {
            cursor::ansi::pos_raw().unwrap()
        } else {
            self.raw();
            let (col, row) = cursor::ansi::pos_raw().unwrap();
            self.cook();
            return (col, row);
        }
    }

    pub fn mark(&mut self) {
        ansi_write(&cursor::ansi::save_pos(), self.autoflush);
    }

    pub fn load(&mut self) {
        ansi_write(&cursor::ansi::load_pos(), self.autoflush);
    }

    pub fn hide_cursor(&mut self) {
        let mut m = &mut self.metas[self.index];
        ansi_write(&cursor::ansi::hide(), self.autoflush);
        m.is_cursor_visible = false;
    }

    pub fn show_cursor(&mut self) {
        let mut m = &mut self.metas[self.index];
        ansi_write(&cursor::ansi::show(), self.autoflush);
        m.is_cursor_visible = true;
    }

    pub fn set_fg(&mut self, color: &str) {
        let fg = Color::from(color);
        ansi_write(&output::ansi::set_fg(fg), self.autoflush);
    }

    pub fn set_bg(&mut self, color: &str) {
        let bg = Color::from(color);
        ansi_write(&output::ansi::set_bg(bg), self.autoflush);
    }

    pub fn set_fmt(&mut self, format: &str) {
        // NOTE: `format` will be `reset` if the passed in
        // `&str` contains multiple values (eg. "bold, underline").
        let fmt = Format::from(format);
        ansi_write(&output::ansi::set_fmt(fmt), self.autoflush);
    }

    pub fn set_fg_rgb(&mut self, r: u8, g:u8, b: u8) {
        let fg = output::Color::Rgb{
            r: r,
            g: g,
            b: b,
        };
        ansi_write(&output::ansi::set_fg(fg), self.autoflush);
    }

    pub fn set_bg_rgb(&mut self, r: u8, g:u8, b: u8) {
        let bg = output::Color::Rgb{
            r: r,
            g: g,
            b: b,
        };
        ansi_write(&output::ansi::set_bg(bg), self.autoflush);
    }

    pub fn set_fg_ansi(&mut self, value: u8) {
        let fg = output::Color::AnsiValue(value);
        ansi_write(&output::ansi::set_fg(fg), self.autoflush);
    }

    pub fn set_bg_ansi(&mut self, value: u8) {
        let bg = output::Color::AnsiValue(value);
        ansi_write(&output::ansi::set_bg(bg), self.autoflush);
    }

    pub fn set_style(&mut self, fg: &str, bg: &str, fmts: &str) {
        // The params fg is a single word, bg is also a single word, however
        // the tx param can be treated as a comma-separated list of words that
        // match the various text styles that are supported: "bold", "dim",
        // "underline", "reverse", "hide", and "reset".
        ansi_write(&output::ansi::set_all(fg, bg, fmts), self.autoflush);
    }

    pub fn reset(&mut self) {
        ansi_write(&output::ansi::reset(), self.autoflush);
    }

    pub fn prints(&mut self, string: &str) {
        ansi_write(&output::ansi::prints(string), false);
    }

    pub fn flush(&mut self) {
        ansi_flush();
    }

    pub fn printf(&mut self, string: &str) {
        ansi_write(&output::ansi::prints(string), true);
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
        let metas = &mut self.metas;
        let rstate = metas[self.index].is_raw_enabled;
        let mstate = metas[self.index].is_mouse_enabled;
        let cstate = metas[self.index].is_cursor_visible;
        metas.push(Metadata{
            is_raw_enabled: rstate,
            is_mouse_enabled: mstate,
            is_cursor_visible: cstate,
        });
    }
}

impl Drop for Tty {
    fn drop(&mut self) {
        self.terminate()
    }
}
