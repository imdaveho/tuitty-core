
use std::io::{stdout, BufWriter, Write};

use crate::screen;
use crate::cursor;
use crate::output;
use crate::input;
use crate::{AsyncReader, SyncReader, Termios};

// Shared code and abstractions that is leveraged by the other modules.
// * `ansi`, a simple macro to help with writing escape sequences to stdout.
// * `wincon`, wrappers for pointers to the Handle and ConsoleInfo objects.

mod ansi;

#[cfg(windows)]
mod wincon;

#[cfg(windows)]
pub use wincon::{Handle, ConsoleInfo};


pub struct Tty {
    pub id: usize,
    pub meta: Vec<Metadata>,
    pub original_mode: Termios,
    ansi_supported: bool,

    #[cfg(windows)]
    altscreen: Option<Handle>,
    
    #[cfg(windows)]
    reset_attrs: u16,
}

pub struct Metadata {
    pub is_raw_enabled: bool,
    pub is_mouse_enabled: bool,
    
    #[cfg(windows)]
    saved_position: Option<(i16, i16)>,
}

impl Tty {

    /******************************
     * OS-specific public methods *
     *****************************/

    #[cfg(unix)]
    pub fn init() -> Tty {
        Tty {
            id: 0,
            meta: vec![Metadata {
                is_raw_enabled: false,
                is_mouse_enabled: false,
            }],
            original_mode: output::ansi::get_mode().unwrap(),
            ansi_supported: true,
        }
    }

    #[cfg(windows)]
    pub fn init() -> Tty {
        Tty {
            id: 0,
            meta: vec![Metadata {
                is_raw_enabled: false,
                is_mouse_enabled: false,
                saved_position: None,
            }],
            original_mode: {
                if !_is_wincon_supported() { 
                    Handle::conout().unwrap()
                    .get_mode().unwrap()
                 } else { 
                    output::wincon::get_mode().unwrap()
                }
            },
            ansi_supported: _is_ansi_supported(),

            altscreen: None,
            reset_attrs: ConsoleInfo::of(
                &Handle::conout().unwrap()
            ).unwrap().attributes(),
        }
    }

    #[cfg(unix)]
    pub fn exit(&mut self) {
        self.main();
        output::ansi::set_mode(&self.original_mode).unwrap();
        _write_ansi(cursor::ansi::show());
        _write_ansi("\n\r".to_string());
        self.meta.clear();
    }

    #[cfg(windows)]
    pub fn exit(&mut self) {
        let handle = match _is_wincon_supported() {
            true => Handle::stdout().unwrap(),
            false => Handle::conout().unwrap(),
        };

        if self.ansi_supported {
            handle.set_mode(&self.original_mode).unwrap();
            _write_ansi(cursor::ansi::show());
            _write_ansi("\n\r".to_string());
        } else {
            handle.set_mode(&self.original_mode).unwrap();
            if let Some(handle) = &self.altscreen {
                handle.close().unwrap();
            }
            self.altscreen = None;
            cursor::wincon::show().unwrap();
            self.write("\n\r");
        }
        self.meta.clear();

    }

    #[cfg(unix)]
    pub fn size(&self) -> (i16, i16) {
        screen::ansi::size()
    }

    #[cfg(windows)]
    pub fn size(&self) -> (i16, i16) {
        screen::wincon::size()
    }

    #[cfg(unix)]
    pub fn raw(&mut self) {
        let mut m = &mut self.meta[self.id];
        output::ansi::enable_raw().unwrap();
        m.is_raw_enabled = true;
    }

    #[cfg(windows)]
    pub fn raw(&mut self) {
        let mut m = &mut self.meta[self.id];
        output::wincon::enable_raw().unwrap();
        m.is_raw_enabled = true;
    }

    // "cooked" vs "raw" mode terminology from Wikipedia:
    // https://en.wikipedia.org/wiki/Terminal_mode
    // A terminal mode is one of a set of possible states of a terminal or 
    // pseudo terminal character device in Unix-like systems and determines how
    // characters written to the terminal are interpreted. In cooked mode data
    // is preprocessed before being given to a program, while raw mode passes 
    // data as-is to the program without interpreting any special characters.

    #[cfg(unix)]
    pub fn cook(&mut self) {
        let mut m = &mut self.meta[self.id];
        output::ansi::set_mode(&self.original_mode).unwrap();
        m.is_raw_enabled = false;
    }

    #[cfg(windows)]
    pub fn cook(&mut self) {
        let mut m = &mut self.meta[self.id];
        output::wincon::disable_raw().unwrap();
        m.is_raw_enabled = false;
    }

    #[cfg(unix)]
    pub fn enable_mouse(&mut self) {
        let mut m = &mut self.meta[self.id];
        _write_ansi(input::ansi::enable_mouse_mode());
        m.is_mouse_enabled = true;
    }

    #[cfg(windows)]
    pub fn enable_mouse(&mut self) {
        let mut m = &mut self.meta[self.id];
        input::wincon::enable_mouse_mode().unwrap();
        m.is_mouse_enabled = true;
    }

    #[cfg(unix)]
    pub fn disable_mouse(tty: &mut Tty) {
        let mut m = &mut self.meta[self.id];
        _write_ansi(input::ansi::disable_mouse_mode());
        m.is_mouse_enabled = false;
    }
    
    #[cfg(windows)]
    pub fn disable_mouse(&mut self) {
        let mut m = &mut self.meta[self.id];
        input::wincon::disable_mouse_mode().unwrap();
        m.is_mouse_enabled = false;
    }

    #[cfg(unix)]
    pub fn read_char(&self) -> char {
        input::ansi::read_char().unwrap()
    }

    #[cfg(windows)]
    pub fn read_char(&self) -> char {
        input::wincon::read_char().unwrap()
    }

    #[cfg(unix)]
    pub fn read_sync(&self) -> SyncReader {
        input::ansi::read_sync()
    }

    #[cfg(windows)]
    pub fn read_sync(&self) -> SyncReader {
        input::wincon::read_sync()
    }

    #[cfg(unix)]
    pub fn read_async(&self) -> AsyncReader {
        input::ansi::read_async()
    }

    #[cfg(windows)]
    pub fn read_async(&self) -> AsyncReader {
        input::wincon::read_async()
    }

    #[cfg(unix)]
    pub fn read_until_async(delimiter: u8) -> AsyncReader {
        input::ansi::read_until_async(delimiter)
    }

    #[cfg(windows)]
    pub fn read_until_async(delimiter: u8) -> AsyncReader {
        input::wincon::read_until_async(delimiter)
    }

    /*******************************
     * API-specific public methods *
     ******************************/

    pub fn clear(&mut self, method: &str) {
        match method {
            "all" => {
                if self.ansi_supported {
                    _write_ansi(screen::ansi::clear(screen::Clear::All));
                    self.goto(0, 0);
                } else {
                    screen::wincon::clear(screen::Clear::All).unwrap();
                    self.goto(0, 0);
                }
            }
            "newln" => {
                if self.ansi_supported {
                    _write_ansi(screen::ansi::clear(screen::Clear::NewLn));
                } else {
                    let (col, row) = cursor::wincon::pos().unwrap();
                    screen::wincon::clear(screen::Clear::NewLn).unwrap();
                    self.goto(col, row);
                }
            }
            "currentln" => {
                if self.ansi_supported {
                    _write_ansi(screen::ansi::clear(screen::Clear::CurrentLn));
                } else {
                    let (_, row) = cursor::wincon::pos().unwrap();
                    screen::wincon::clear(screen::Clear::CurrentLn).unwrap();
                    self.goto(0, row);
                }
            }
             "cursorup" => {
                if self.ansi_supported {
                    _write_ansi(screen::ansi::clear(screen::Clear::CursorUp));
                } else {
                    screen::wincon::clear(screen::Clear::CursorUp).unwrap();
                }
            }
            "cursordn" => {
                if self.ansi_supported {
                    _write_ansi(screen::ansi::clear(screen::Clear::CursorDn));
                } else {
                    screen::wincon::clear(screen::Clear::CursorDn).unwrap();
                }
            }
            _ => ()
        }
    }

    pub fn resize(&mut self, w: i16, h: i16) {
        if self.ansi_supported {
            _write_ansi(screen::ansi::resize(w, h));
        } else {
            screen::wincon::resize(w, h).unwrap();
        }
    }

    pub fn switch(&mut self) {
        // In order to support multiple "screens", this function creates a new
        // Metadata entry which stores any screen specific changes that a User
        // might want to be restored when switching between screens.
        if self.ansi_supported {
            if self.id == 0 {
                // There is no point to switch if you're on another screen
                // since ANSI terminals provide a single "alternate screen".
                _write_ansi(screen::ansi::enable_alt());
            }
            // Add new `Metadata` for the new screen. (OS-specific)
            self._add_metadata();
            self.id = self.meta.len() - 1;
            // Explicitly set default screen settings. (ANSI-only)
            self.cook();
            self.disable_mouse();
        } else {
            if self.altscreen.is_none() {
                self.altscreen = Some(Handle::buffer().unwrap());
            }
            if let Some(handle) = &self.altscreen {
                handle.set_mode(&self.original_mode).unwrap();
                if self.id == 0 {
                    // There is a single handle for the alternate screen buffer; so 
                    // only if you're on id == 0 or the main screen, do you need to
                    // enable the alternate.
                    handle.show().unwrap();
                }
                // Add new `Metadata` for the new screen. (OS-specific)
                self._add_metadata();
                self.id = self.meta.len() - 1;
            }
        }
    }

    pub fn main(&mut self) {
        // Only execute if the User is not on the main screen buffer.
        if self.id != 0 {
            if self.ansi_supported {
                let metas = &self.meta;
                let rstate = metas[0].is_raw_enabled;
                let mstate = metas[0].is_mouse_enabled;
                self.id = 0;
                _write_ansi(screen::ansi::disable_alt());

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
            } else {
                let metas = &self.meta;
                let rstate = metas[0].is_raw_enabled;
                let mstate = metas[0].is_mouse_enabled;
                let mode = &self.original_mode;
                let stdout = Handle::stdout().unwrap();
                stdout.set_mode(mode).unwrap();
                self.id = 0;
                screen::wincon::disable_alt().unwrap();

                if rstate {
                    self.raw();
                }

                if mstate {
                    self.enable_mouse();
                }
            }
        }
    }

    pub fn switch_to(&mut self, id: usize) {
        // NOTE: this only switches the screen buffer and updates the settings.
        //  Updating the content that will be passed in and rendered, that is 
        // up to the implementation.

        // If the id and the current id are the same, well, there is nothing to
        // do, you're already on the active screen buffer.
        if id != self.id {
            if id == 0 {
                // Switch to the main screen.
                self.main();
            } else {
                if self.ansi_supported {
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
                } else {
                    let metas = &self.meta;
                    let rstate = metas[id].is_raw_enabled;
                    let mstate = metas[id].is_mouse_enabled;
                    let mode = &self.original_mode;
                    if let Some(handle) = &self.altscreen {
                        handle.set_mode(mode).unwrap();
                        // Only show the altscreen handle if there is a switch
                        // from the main screen. Otherwise, the altscreen is
                        // already showing and no need to call `show()`.
                        if self.id == 0 {
                            handle.show().unwrap();
                        }
                        self.id = id;

                        if rstate {
                            self.raw();
                        }

                        if mstate {
                            self.enable_mouse();
                        }
                    }
                }
            }
        }
    }

    pub fn goto(&mut self, col: i16, row: i16) {
        if self.ansi_supported {
            _write_ansi(cursor::ansi::goto(col, row));
        } else {
            cursor::wincon::goto(col, row).unwrap();
        }
    }

    pub fn up(&mut self) {
        if self.ansi_supported {
            _write_ansi(cursor::ansi::move_up(1));
        } else {
            cursor::wincon::move_up(1).unwrap();
        }
    }

    pub fn dn(&mut self) {
        if self.ansi_supported {
            _write_ansi(cursor::ansi::move_down(1));
        } else {
            cursor::wincon::move_down(1).unwrap();
        }
    }

    pub fn left(&mut self) {
        if self.ansi_supported {
            _write_ansi(cursor::ansi::move_left(1));
        } else {
            cursor::wincon::move_left(1).unwrap();
        }
    }

    pub fn right(&mut self) {
        if self.ansi_supported {
            _write_ansi(cursor::ansi::move_right(1));
        } else {
            cursor::wincon::move_right(1).unwrap();
        }
    }

    pub fn dpad(&mut self, dir: &str, n: i16) {
        // Case-insensitive.
        let d = dir.to_lowercase();
        if n > 0 {
            match d.as_str() {
                "up" => {
                    if self.ansi_supported {
                        _write_ansi(cursor::ansi::move_up(n));
                    } else {
                        cursor::wincon::move_up(n).unwrap();
                    }
                },
                "dn" => {
                    if self.ansi_supported {
                        _write_ansi(cursor::ansi::move_down(n));
                    } else {
                        cursor::wincon::move_down(n).unwrap();
                    }
                },
                "left" => {
                    if self.ansi_supported {
                        _write_ansi(cursor::ansi::move_left(n));
                    } else {
                        cursor::wincon::move_left(n).unwrap();
                    }
                },
                "right" => {
                    if self.ansi_supported {
                        _write_ansi(cursor::ansi::move_right(n));
                    } else {
                        cursor::wincon::move_right(n).unwrap();
                    }
                },
                _ => ()
            };
        }
    }

    pub fn pos(&mut self) -> (i16, i16) {
        if self.ansi_supported {
            if self.meta[self.id].is_raw_enabled {
                cursor::ansi::pos_raw().unwrap()
            } else {
                self.raw();
                let (col, row) = cursor::ansi::pos_raw().unwrap();
                self.cook();
                return (col, row);
            }
        } else {
            cursor::wincon::pos().unwrap()
        }
    }

    pub fn mark(&mut self) {
        if self.ansi_supported {
            _write_ansi(cursor::ansi::save_pos());
        } else {
            self.meta[self.id].saved_position = Some(
                cursor::wincon::pos().unwrap()
            );
        }
    }

    pub fn load(&mut self) {
        if self.ansi_supported {
            _write_ansi(cursor::ansi::load_pos());
        } else {
            match self.meta[self.id].saved_position {
                Some(pos) => {
                    self.goto(pos.0, pos.1);
                }
                None => ()
            }
        }
    }

    pub fn hide_cursor(&mut self) {
        if self.ansi_supported {
            _write_ansi(cursor::ansi::hide());
        } else {
            cursor::wincon::hide().unwrap();
        }
    }

    pub fn show_cursor(&mut self) {
        if self.ansi_supported {
            _write_ansi(cursor::ansi::show());
        } else {
            cursor::wincon::show().unwrap();
        }
    }

    pub fn set_fg(&mut self, color: &str) {
        let fg_col = output::Color::from(color);
        if self.ansi_supported {
            _write_ansi(output::ansi::set_fg(fg_col));
        } else {
            output::wincon::set_fg(fg_col, self.reset_attrs).unwrap();
        }
    }

    pub fn set_bg(&mut self, color: &str) {
        let bg_col = output::Color::from(color);
        if self.ansi_supported {
            _write_ansi(output::ansi::set_bg(bg_col));
        } else {
            output::wincon::set_bg(bg_col, self.reset_attrs).unwrap();
        }
    }

    pub fn set_tx(&mut self, style: &str) {
        let tstyle = output::TextStyle::from(style);
        if self.ansi_supported {
            _write_ansi(output::ansi::set_tx(tstyle));
        } else {
            output::wincon::set_tx(tstyle).unwrap();
        }
    }

    pub fn set_fg_rgb(&mut self, r: u8, g:u8, b: u8) {
        let fg_col = output::Color::Rgb{
            r: r,
            g: g,
            b: b,
        };
        if self.ansi_supported {
            _write_ansi(output::ansi::set_fg(fg_col));
        } else {
            output::wincon::set_fg(fg_col, self.reset_attrs).unwrap();
        }
    }

    pub fn set_bg_rgb(&mut self, r: u8, g:u8, b: u8) {
        let bg_col = output::Color::Rgb{
            r: r,
            g: g,
            b: b,
        };
        if self.ansi_supported {
            _write_ansi(output::ansi::set_bg(bg_col));
        } else {
            output::wincon::set_bg(bg_col, self.reset_attrs).unwrap();
        }
    }

    pub fn set_fg_ansi(&mut self, v: u8) {
        let fg_col = output::Color::AnsiValue(v);
        if self.ansi_supported {
            _write_ansi(output::ansi::set_fg(fg_col));
        } else {
            output::wincon::set_fg(fg_col, self.reset_attrs).unwrap();
        }
    }

    pub fn set_bg_ansi(&mut self, v: u8) {
        let bg_col = output::Color::AnsiValue(v);
        if self.ansi_supported {
            _write_ansi(output::ansi::set_bg(bg_col));
        } else {
            output::wincon::set_bg(bg_col, self.reset_attrs).unwrap();
        }
    }

    pub fn set_style(&mut self, fg: &str, bg: &str, style: &str) {
        // The params fg is a single word, bg is also a single word, however
        // the tx param can be treated as a comma-separated list of words that
        // match the various text styles that are supported: "bold", "dim",
        // "underline", "reverse", "hide", and "reset".
        if self.ansi_supported {
            _write_ansi(output::ansi::set_all(fg, bg, style));
        } else {
            output::wincon::set_all(fg, bg, style, self.reset_attrs).unwrap();
        }
    }

    pub fn reset(&mut self) {
        if self.ansi_supported {
            _write_ansi(output::ansi::reset());
        } else {
            output::wincon::reset(self.reset_attrs).unwrap();
        }
    }

    pub fn write(&mut self, s: &str) {
        if self.ansi_supported {
            _write_ansi(output::ansi::writeout(s));
        } else {
            output::wincon::writeout(s).unwrap();
        }
    }

    pub fn flush(&mut self) {
        // ANSI-only
        if self.ansi_supported {
            let cout = stdout();
            let lock = cout.lock();
            let mut writer = BufWriter::new(lock);
            writer.flush().unwrap();
        }
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

    /****************************************
     * OS-specific helper / private methods *
     ***************************************/

    #[cfg(unix)]
    fn _add_metadata(&mut self) {
        let mut metas = &mut self.meta;
        let rstate = metas[self.id].is_raw_enabled;
        let mstate = metas[self.id].is_mouse_enabled;
        metas.push(Metadata{
            is_raw_enabled: rstate,
            is_mouse_enabled: mstate,
        });
        
    }

    #[cfg(windows)]
    fn _add_metadata(&mut self) {
        let metas = &mut self.meta;
        let rstate = metas[self.id].is_raw_enabled;
        let mstate = metas[self.id].is_mouse_enabled;
        metas.push(Metadata{
            is_raw_enabled: rstate,
            is_mouse_enabled: mstate,
            saved_position: None,
        });
    }
}


fn _write_ansi(s: String) {
    let cout = stdout();
    let lock = cout.lock();
    let mut writer = BufWriter::new(lock);
    writer.write(s.as_bytes()).unwrap();
}

#[cfg(windows)]
fn _is_ansi_supported() -> bool {
    const TERMS: [&'static str; 15] = [
        "xterm",  // xterm, PuTTY, Mintty
        "rxvt",   // RXVT
        "eterm",  // Eterm
        "screen", // GNU screen, tmux
        "tmux",   // tmux
        "vt100", "vt102", "vt220", "vt320",   // DEC VT series
        "ansi",    // ANSI
        "scoansi", // SCO ANSI
        "cygwin",  // Cygwin, MinGW
        "linux",   // Linux console
        "konsole", // Konsole
        "bvterm",  // Bitvise SSH Client
    ];

    let matched_terms = match std::env::var("TERM") {
        Ok(val) => val != "dumb" || TERMS.contains(&val.as_str()),
        Err(_) => false,
    };

    if matched_terms {
        return true
    } else {
        let enable_vt = 0x0004;
        let handle = match Handle::stdout() {
            Ok(h) => h,
            Err(_) => return false,
        };
        let mode = match handle.get_mode() {
            Ok(m) => m,
            Err(_) => return false,
        };
        match handle.set_mode(&(mode | enable_vt)) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}

#[cfg(windows)]
fn _is_wincon_supported() -> bool {
    // MinTTY (and alledgedly ConPTY) do not have common support for the native
    // Console API. The MinTTY instance used by `git-bash` emulates over MSYS2,
    // which supports ANSI sequences, but throws an error when tryiing to fetch
    // the default terminal mode from `Termios` (does not exist on Windows) or 
    // from the `Handle` (Console API not supported).
    //
    // MSYSTEM environment variable: (stackoverflow)
    // questions/37460073/msys-vs-mingw-internal-environment-variables
    //
    // MinTTY github issue: https://github.com/mintty/mintty/issues/56
    match std::env::var("MSYSTEM") {
        Ok(_) => false, // MSYS, MINGW64, MINGW32
        Err(_) => true, // 
    }
}