//! This module consolidates the parts that make up a TTY into a single struct
//! and exposes the methods that work specifically on Unix systems.

use super::screen;
use super::cursor;
use super::output;
use super::input;
use super::shared::{write_ansi, flush_ansi};
use super::{AsyncReader, SyncReader, Termios};


pub struct Tty {
    index: usize,
    metas: Vec<Metadata>,
    original_mode: Termios,
}

pub struct Metadata {
    is_raw_enabled: bool,
    is_mouse_enabled: bool,
    is_cursor_visible: bool,
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
        }
    }

    pub fn terminate(&mut self) {
        self.to_main();
        output::ansi::set_mode(&self.original_mode).unwrap();
        write_ansi(&cursor::ansi::show());
        write_ansi("\n\r");
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
        write_ansi(&input::ansi::enable_mouse_mode());
        m.is_mouse_enabled = true;
    }

    pub fn disable_mouse(&mut self) {
        let mut m = &mut self.metas[self.index];
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
                let (_, row) = self.pos();
                self.goto(0, row);
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
        self.flush();
    }


    pub fn switch(&mut self) {
        // In order to support multiple "screens", this function creates a new
        // Metadata entry which stores any screen specific changes that a User
        // might want to be restored when switching between screens.
        if self.index == 0 {
            // There is no point to switch if you're on another screen
            // since ANSI terminals provide a single "alternate screen".
            write_ansi(&screen::ansi::enable_alt());
        }
        // Add new `Metadata` for the new screen.
        self._add_metadata();
        self.index = self.metas.len() - 1;
        // Explicitly set default screen settings.
        self.cook();
        self.disable_mouse();
        self.show_cursor();
        self.goto(0, 0)
    }


    pub fn to_main(&mut self) {
        // Only execute if the User is not on the main screen buffer.
        if self.index != 0 {
            let metas = &self.metas;
            let rstate = metas[0].is_raw_enabled;
            let mstate = metas[0].is_mouse_enabled;
            let cstate = metas[0].is_cursor_visible;
            self.index = 0;
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

            if cstate {
                self.show_cursor();
            } else {
                self.hide_cursor();
            }
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

                if cstate {
                    self.show_cursor();
                } else {
                    self.hide_cursor();
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
        write_ansi(&cursor::ansi::save_pos());
    }

    pub fn load(&mut self) {
        write_ansi(&cursor::ansi::load_pos());
    }

    pub fn hide_cursor(&mut self) {
        let mut m = &mut self.metas[self.index];
        write_ansi(&cursor::ansi::hide());
        m.is_cursor_visible = false;
    }

    pub fn show_cursor(&mut self) {
        let mut m = &mut self.metas[self.index];
        write_ansi(&cursor::ansi::show());
        m.is_cursor_visible = true;
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
        // NOTE: `style` will be `reset` if the passed in
        // `&str` contains multiple values (eg. "bold, underline").
        let style = output::TextStyle::from(style);
        write_ansi(&output::ansi::set_tx(style));
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

    pub fn prints(&mut self, s: &str) {
        write_ansi(&output::ansi::prints(s));
    }

    pub fn flush(&mut self) {
        // ANSI-only
        flush_ansi();
    }

    pub fn printf(&mut self, s: &str) {
        self.prints(s);
        self.flush();
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
