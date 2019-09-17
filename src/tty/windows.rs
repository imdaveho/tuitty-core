//! This module consolidates the parts that make up a TTY into a single struct
//! and exposes the methods that work specifically on Windows systems.
//!
//! Additionally, it provides logic to identify the type of terminal being used
//! on the Windows system (eg. xterm, MinTTY, Cygwin, etc.) as well as check if
//! the terminal has support for ANSI sequences.

use super::cursor;
use super::input;
use super::screen::{self, Clear::*};
use super::output::{self, Color, Style::*};
use super::shared::{ansi_write, ansi_flush, Metadata};
use super::{AsyncReader, SyncReader, Termios};

#[cfg(windows)]
use super::{Handle, ConsoleInfo, ConsoleOutput};


pub struct Tty {
    index: usize,
    state: Metadata,
    stash: Vec<Metadata>,
    original_mode: Termios,
    ansi_supported: bool,
    altscreen: Option<Handle>,
    console_output: ConsoleOutput,
    flush_if_auto: bool,
}

impl Tty {

    pub fn init() -> Tty {
        Tty {
            index: 0,
            state: Metadata::new(),
            stash: Vec::with_capacity(5),
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
            console_output: ConsoleOutput(
                ConsoleInfo::of(&Handle::conout().unwrap())
                .unwrap().attributes()
            ),
            flush_if_auto: false,
        }
    }

    // (imdaveho) NOTE: size in this context is more for internal use.
    // See: CellBuffer::new()
    // pub fn size(&self) -> (i16, i16) {
    //     // Windows Console API only (no ANSI equivalent).
    //     screen::wincon::size()
    // }

    pub fn screen_size(&self) -> (i16, i16) {
        self.state.cellbuf._screen_size()
    }

    // "cooked" vs "raw" mode terminology from Wikipedia:
    // https://en.wikipedia.org/wiki/Terminal_mode
    // A terminal mode is one of a set of possible states of a terminal or
    // pseudo terminal character device in Unix-like systems and determines how
    // characters written to the terminal are interpreted. In cooked mode data
    // is preprocessed before being given to a program, while raw mode passes
    // data as-is to the program without interpreting any special characters.
    pub fn raw(&mut self) {
        // Windows Console API only (no ANSI equivalent).
        output::wincon::enable_raw().unwrap();
        self.state._raw();
    }

    pub fn cook(&mut self) {
        // Windows Console API only (no ANSI equivalent).
        output::wincon::disable_raw().unwrap();
        self.state._cook();
    }

    // Input module functions are OS-specific.
    // * enable/disable mouse
    // * read_char/sync/async/until_async

    pub fn enable_mouse(&mut self) {
        input::wincon::enable_mouse_mode().unwrap();
        self.state._enable_mouse();
    }

    pub fn disable_mouse(&mut self) {
        input::wincon::disable_mouse_mode().unwrap();
        self.state._disable_mouse();
    }

    pub fn read_char(&self) -> char {
        input::wincon::read_char().unwrap()
    }

    pub fn read_sync(&self) -> SyncReader {
        input::wincon::read_sync()
    }

    pub fn read_async(&self) -> AsyncReader {
        input::wincon::read_async()
    }

    pub fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        input::wincon::read_until_async(delimiter)
    }


    pub fn clear(&mut self, method: &str) {
        match method {
            "all" => {
                if self.ansi_supported {
                    ansi_write(
                        &screen::ansi::clear(All),
                        self.flush_if_auto);
                } else {
                    screen::wincon::clear(screen::Clear::All).unwrap();
                    self.goto(0, 0);
                }
                self.state.cellbuf._clear(All);
                // (imdaveho) NOTE: might be too many "side effects" in a single
                // method call...this should be explicit for the user to batch
                // operations together; eg. clear all, goto(0,0), flush()
                // self.goto(0, 0);
                // (imdaveho) NOTE: there might be differences between ansi and
                // wincon that perhaps this "goto" normalizes...
                // TODO: TEST this.
            }
            "newln" => {
                if self.ansi_supported {
                    ansi_write(
                        &screen::ansi::clear(NewLn),
                        self.flush_if_auto);
                } else {
                    let (col, row) = cursor::wincon::pos().unwrap();
                    screen::wincon::clear(screen::Clear::NewLn).unwrap();
                    self.goto(col, row);
                }
                self.state.cellbuf._clear(NewLn);
            }
            "currentln" => {
                if self.ansi_supported {
                    ansi_write(
                        &screen::ansi::clear(CurrentLn),
                        self.flush_if_auto);
                    let (_, row) = self.pos();
                    self.goto(0, row);
                } else {
                    let (_, row) = cursor::wincon::pos().unwrap();
                    screen::wincon::clear(screen::Clear::CurrentLn).unwrap();
                    self.goto(0, row);
                }
                self.state.cellbuf._clear(CurrentLn);
            }
            "cursorup" => {
                if self.ansi_supported {
                    ansi_write(
                        &screen::ansi::clear(CursorUp),
                        self.flush_if_auto);
                } else {
                    screen::wincon::clear(screen::Clear::CursorUp).unwrap();
                }
                self.state.cellbuf._clear(CursorUp)
            }
            "cursordn" => {
                if self.ansi_supported {
                    ansi_write(
                        &screen::ansi::clear(CursorDn),
                        self.flush_if_auto);
                } else {
                    screen::wincon::clear(screen::Clear::CursorDn).unwrap();
                }
                self.state.cellbuf._clear(CursorDn);
            }
            _ => ()
        }
    }

    pub fn resize(&mut self, w: i16, h: i16) {
        if self.ansi_supported {
            ansi_write(
                &screen::ansi::resize(w, h),
                self.flush_if_auto);
        } else {
            screen::wincon::resize(w, h).unwrap();
        }
        self.state.cellbuf._resize(w, h);
    }

    pub fn manual(&mut self) {
        self.flush_if_auto = false;
    }

    pub fn automatic(&mut self) {
        self.flush_if_auto = true;
    }

    pub fn switch(&mut self) {
        // In order to support multiple "screens", this function creates a new
        // Metadata entry which stores any screen specific changes that a User
        // might want to be restored when switching between screens.
        if self.index == 0 {
            if self.ansi_supported {
                // // There is no point to switch if you're on another screen
                // // since ANSI terminals provide a single "alternate screen".
                ansi_write(&screen::ansi::enable_alt(), self.flush_if_auto);
                ansi_write(&screen::ansi::clear(All), self.flush_if_auto);
            } else {
                if self.altscreen.is_none() {
                    self.altscreen = Some(Handle::buffer().unwrap());
                }
                if let Some(handle) = &self.altscreen {
                    handle.set_mode(&self.original_mode).unwrap();
                    // There is a single handle for the alternate screen buffer;
                    // so only if you're on index == 0 or the main screen, do you
                    // need to enable the alternate.
                    handle.show().unwrap();
                }
            }
        } else {
            // If this wasn't a switch to the alternate screen (ie. the current
            // screen is already the alternate screen), then we need to clear it
            // without modifying the cellbuffer.
            if self.ansi_supported {
                ansi_write(&screen::ansi::clear(All), self.flush_if_auto);
            } else {
                screen::wincon::clear(screen::Clear::All).unwrap();
            }
        }
        // Push current self.state `Metadata` to stash and increment the index.
        // Swap the current self.state for a new Metadata struct.
        self.stash.push(self.state.clone());
        self.state = Metadata::new();
        self.index = self.stash.len();

        if self.ansi_supported {
            // Prevent multiple `flush()` calls due to `autoflush` setting.
            let auto = self.flush_if_auto;
            if auto { self.manual() }
            // Explicitly set default screen settings. (ANSI-only)
            self.cook();
            self.disable_mouse();
            self.show_cursor();
            self.goto(0, 0);
            // Revert back to previous `autoflush` configuration.
            if auto { self.automatic(); self.flush(); }
        } else {
            self.show_cursor();
            self.goto(0, 0);
        }
    }

    pub fn to_main(&mut self) {
        if self.index == 0 { return }
        self.switch_to(0);
    }

    pub fn switch_to(&mut self, index: usize) {
        // If the id and the current id are the same, well, there is nothing to
        // do, you're already on the active screen buffer.
        if index == self.index { return }
        // The below is to handle cases where `switch()` created a `Metadata`
        // state that has not yet been pushed to self.stash. If it has already
        // been pushed, update the stash at the current `self.index` before
        // getting the Metadata at the switched to (function argument) `index`.
        if self.stash.len() - 1 < self.index {
            self.stash.push(self.state.clone())
        } else {
            self.stash[self.index] = self.state.clone();
        }
        // After updating the stash, clone the Metadata at the switch_to index.
        self.state = self.stash[index].clone();
        // Enable/Disable alternate screen based on current and target indices.
        if index == 0 {
            // Disable if you are reverting back to main.
            if self.ansi_supported {
                ansi_write(&screen::ansi::disable_alt(), self.flush_if_auto)
            } else {
                screen::wincon::disable_alt().unwrap();
                let handle = Handle::stdout().unwrap();
                handle.set_mode(&self.original_mode).unwrap();
            }
        } else {
            if self.ansi_supported {
                if self.index == 0 {
                    // Enable if you are already on main switching to an
                    // alternate screen.
                    ansi_write(&screen::ansi::enable_alt(), self.flush_if_auto)
                }
                ansi_write(&screen::ansi::clear(All), self.flush_if_auto);
                ansi_write(&cursor::ansi::goto(0, 0), self.flush_if_auto);
            } else {
                if self.index == 0 {
                    // Enable if you are already on main switching to an
                    // alternate altscreen.
                    if let Some(handle) = &self.altscreen {
                        handle.set_mode(&self.original_mode).unwrap();
                        handle.show().unwrap();
                    }
                }
                screen::wincon::clear(All).unwrap();
                cursor::wincon::goto(0, 0).unwrap();
            }
            // Restore screen contents. Restore flushes.
            self.state.cellbuf._restore_buffer();
            let (col, row) = self.state.cellbuf._screen_pos();
            self.goto(col, row);
        }
        // Update `self.index` to the function argument `index`
        // (imdaveho) TODO: Confirm if main screen will have native buffer logs,
        // thereby not needing to restore content manually via library. Also,
        // because there is going to be output that is not from `tty` which is
        // not possible to save in the backbuf.
        self.index = index;
        
        if self.ansi_supported {
            let auto = self.flush_if_auto;
            if auto { self.manual() }
            let (raw, mouse, show) = (
                self.state._is_raw(),
                self.state._is_mouse(),
                self.state._is_cursor());
            // Restore settings based on metadata.
            if raw { self.raw() } else { self.cook() }
            if mouse { self.enable_mouse() } else { self.disable_mouse() }
            if show { self.show_cursor() } else { self.hide_cursor() }

            // Revert back to previous `autoflush` configuration.
            // (imdaveho) NOTE: `_flush_backbuf` always calls `flush` so there
            // is no need to call it again below as `switch()` does.
            if auto { self.automatic() }
        } else {
            let (raw, mouse, show) = (
                self.state._is_raw(),
                self.state._is_mouse(),
                self.state._is_cursor());
            if raw { self.raw() } 
            if mouse { self.enable_mouse() }
            if show { self.show_cursor() } else { self.hide_cursor() }
        }
    }

    pub fn goto(&mut self, col: i16, row: i16) {
        // (imdaveho) NOTE: Disallow negative values.
        if col < 0 || row < 0 { return }
        if self.ansi_supported {
            ansi_write(
                &cursor::ansi::goto(col, row),
                self.flush_if_auto);
        } else {
            cursor::wincon::goto(col, row).unwrap();
        }
        self.state.cellbuf._reposition(col, row);
    }

    pub fn up(&mut self) {
        if self.ansi_supported {
            ansi_write(
                &cursor::ansi::move_up(1),
                self.flush_if_auto);
        } else {
            cursor::wincon::move_up(1).unwrap();
        }
        self.state.cellbuf._sync_up(1);
    }

    pub fn dn(&mut self) {
        if self.ansi_supported {
            ansi_write(
                &cursor::ansi::move_down(1),
                self.flush_if_auto);
        } else {
            cursor::wincon::move_down(1).unwrap();
        }
        self.state.cellbuf._sync_dn(1);
    }

    pub fn left(&mut self) {
        if self.ansi_supported {
            ansi_write(
                &cursor::ansi::move_left(1),
                self.flush_if_auto);
        } else {
            cursor::wincon::move_left(1).unwrap();
        }
        self.state.cellbuf._sync_left(1);
    }

    pub fn right(&mut self) {
        if self.ansi_supported {
            ansi_write(
                &cursor::ansi::move_right(1),
                self.flush_if_auto);
        } else {
            cursor::wincon::move_right(1).unwrap();
        }
        self.state.cellbuf._sync_right(1);
    }

    pub fn dpad(&mut self, dir: &str, n: i16) {
        // (imdaveho) NOTE: Only deal with non-negative `n`. We use i16 to
        // mirror types for getting cursor position and returning terminal size.
        if n < 0 { return }
        // Case-insensitive.
        match dir.to_lowercase().as_str() {
            "up" => {
                if self.ansi_supported {
                    ansi_write(
                        &cursor::ansi::move_up(n),
                        self.flush_if_auto);
                } else {
                    cursor::wincon::move_up(n).unwrap();
                }
                self.state.cellbuf._sync_up(n);
            },
            "dn" => {
                if self.ansi_supported {
                    ansi_write(
                        &cursor::ansi::move_down(n),
                        self.flush_if_auto);
                } else {
                    cursor::wincon::move_down(n).unwrap();
                }
                self.state.cellbuf._sync_dn(n);
            },
            "left" => {
                if self.ansi_supported {
                    ansi_write(
                        &cursor::ansi::move_left(n),
                        self.flush_if_auto);
                } else {
                    cursor::wincon::move_left(n).unwrap();
                }
                self.state.cellbuf._sync_left(n);
            },
            "right" => {
                if self.ansi_supported {
                    ansi_write(
                        &cursor::ansi::move_right(n),
                        self.flush_if_auto);
                } else {
                    cursor::wincon::move_right(n).unwrap();
                }
                self.state.cellbuf._sync_right(n);
            },
            _ => ()
        }
    }

    pub fn pos(&mut self) -> (i16, i16) {
        let err_message = "Error reading cursor position (I/O related)";
        if self.ansi_supported {
            let (col, row) = if self.state._is_raw() {
                cursor::ansi::pos_raw()
                    .expect(err_message) // TODO: .unwrap_or((0, 0))
            } else {
                self.raw();
                let (col, row) = cursor::ansi::pos_raw()
                    .expect(err_message);
                self.cook();
                (col, row)
            };
            self.state.cellbuf._reposition(col, row);
            (col, row)
        } else {
            cursor::wincon::pos().expect(err_message) // TODO .unwrap_or((0, 0))
        }
    }

    pub fn screen_pos(&mut self) -> (i16, i16) {
        self.state.cellbuf._screen_pos()
    }

    pub fn mark(&mut self) {
        // if self.ansi_supported {
        //     write_ansi(&cursor::ansi::save_pos());
        //     if self.autoflush { self.flush() }
        // } else {
        //     self.metas[self.index].saved_position = Some(
        //         cursor::wincon::pos().unwrap()
        //     );
        // }
        self.state._mark_position();
    }

    pub fn load(&mut self) {
        // if self.ansi_supported {
        //     write_ansi(&cursor::ansi::load_pos());
        //     if self.autoflush { self.flush() }
        // } else {
        //     match self.metas[self.index].saved_position {
        //         Some(pos) => {
        //             self.goto(pos.0, pos.1);
        //         }
        //         None => ()
        //     }
        // }
        let (col, row) = self.state._saved_position();
        self.goto(col, row);
    }

    pub fn hide_cursor(&mut self) {
        if self.ansi_supported {
            ansi_write(
                &cursor::ansi::hide(),
                self.flush_if_auto);
        } else {
            cursor::wincon::hide().unwrap();
        }
        self.state._hide_cursor();
    }

    pub fn show_cursor(&mut self) {
        if self.ansi_supported {
            ansi_write(
                &cursor::ansi::show(),
                self.flush_if_auto);
        } else {
            cursor::wincon::show().unwrap();
        }
        self.state._show_cursor();
    }

    pub fn set_fg(&mut self, color: Color) {
        if self.ansi_supported {
            ansi_write(
                &output::ansi::set_style(Fg(color)),
                self.flush_if_auto);
        
        } else {
            self.console_output.set_style(Fg(color)).unwrap()
        }
        self.state.cellbuf._sync_style(Fg(color));
    }

    pub fn set_bg(&mut self, color: Color) {
        if self.ansi_supported {
            ansi_write(
                &output::ansi::set_style(Bg(color)),
                self.flush_if_auto);
        } else {
            self.console_output.set_style(Bg(color)).unwrap()
        }
        self.state.cellbuf._sync_style(Bg(color));
    }

    pub fn set_fx(&mut self, effects: u32) {
        if self.ansi_supported {
            ansi_write(
                &output::ansi::set_style(Fx(effects)),
                self.flush_if_auto);
        } else {
            self.console_output.set_style(Fx(effects)).unwrap()
        }
        self.state.cellbuf._sync_style(Fx(effects));
    }

    pub fn set_styles(&mut self, fgcol: Color, bgcol: Color, effects: u32) {
        if self.ansi_supported {
            ansi_write(
                &output::ansi::set_styles(fgcol, bgcol, effects),
                self.flush_if_auto);
            self.state.cellbuf._sync_style(Fg(fgcol));
            self.state.cellbuf._sync_style(Bg(bgcol));
            self.state.cellbuf._sync_style(Fx(effects));
        } else {
            self.console_output.set_styles(fgcol, bgcol, effects).unwrap()
        }
    }

    pub fn reset(&mut self) {
        if self.ansi_supported {
            ansi_write(
                &output::ansi::reset(),
                self.flush_if_auto);
        } else {
            self.console_output.reset().unwrap();
        }
        self.state.cellbuf._reset_style();
    }

    pub fn prints(&mut self, content: &str) {
        self.state.cellbuf._sync_buffer(content);
        if self.ansi_supported {
            ansi_write(&output::ansi::prints(content), self.flush_if_auto);
        } else {
            output::wincon::prints(content).unwrap();
        }
    }

    pub fn flush(&mut self) {
        // ANSI-only
        if self.ansi_supported {
            ansi_flush();
        }
    }

    pub fn printf(&mut self, content: &str) {
        self.prints(content);
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


    pub fn terminate(&mut self) {
        let stdout = match _is_wincon_supported() {
            true => Handle::stdout().unwrap(),
            false => Handle::conout().unwrap(),
        };

        self.to_main();

        if self.ansi_supported {
            stdout.set_mode(&self.original_mode).unwrap();
            self.show_cursor();
            ansi_write("\n\r", true);
            self.flush();
        } else {
            stdout.set_mode(&self.original_mode).unwrap();
            if let Some(handle) = &self.altscreen {
                handle.close().unwrap();
            }
            self.altscreen = None;
            cursor::wincon::show().unwrap();
            self.prints("\n\r");
        }
        self.stash.clear();
    }

    // pub fn is_ansi(&self) -> bool {
    //     self.ansi_supported
    // }
}


impl Drop for Tty {
    fn drop(&mut self) {
        self.terminate()
    }
}


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
