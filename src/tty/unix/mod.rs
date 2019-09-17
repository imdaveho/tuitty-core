//! This module consolidates the parts that make up a TTY into a single struct
//! and exposes the methods that work specifically on Unix systems.

use super::cursor;
use super::input;
use super::screen::{self, Clear::*};
use super::output::{self, Color, Effects, Style::*};
use super::shared::{ansi_write, ansi_flush, CellBuffer, Metadata};
use super::{AsyncReader, SyncReader, Termios};

// mod metadata;
// use metadata::Metadata;


pub struct Tty {
    index: usize,
    state: Metadata,
    stash: Vec<Metadata>,
    original_mode: Termios,
    flush_if_auto: bool,
}

impl Tty {
    pub fn init() -> Tty {
        Tty {
            index: 0,
            state: Metadata::new(),
            stash: Vec::with_capacity(5),
            original_mode: output::ansi::get_mode()
                .expect("Error fetching Termios"),
            flush_if_auto: false,
        }
    }

    // (imdaveho) NOTE: size in this context is more for internal use.
    // See: CellBuffer::new()
    // pub fn size(&self) -> (i16, i16) {
    //     // Unix specific dependency on `libc`.
    //     screen::ansi::size()
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
        // Unix specific: depends on `libc::Termios`.
        output::ansi::enable_raw()
            .expect("Error enabling raw mode");
        self.state._raw();
    }

    pub fn cook(&mut self) {
        // Unix specific: depends on `libc::Termios`.
        output::ansi::set_mode(&self.original_mode)
            .expect("Error disabling raw mode");
        self.state._cook();
    }

    // Input module functions are OS-specific.
    // * enable/disable mouse
    // * read_char/sync/async/until_async

    pub fn enable_mouse(&mut self) {
        ansi_write(
            &input::ansi::enable_mouse_mode(),
            self.flush_if_auto);
        self.state._enable_mouse();
    }

    pub fn disable_mouse(&mut self) {
        ansi_write(
            &input::ansi::disable_mouse_mode(),
            self.flush_if_auto);
        self.state._disable_mouse();
    }

    pub fn read_char(&self) -> char {
        input::ansi::read_char()
            .expect("Error reading a character from stdin")
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
                ansi_write(
                    &screen::ansi::clear(All),
                    self.flush_if_auto);
                self.state.cellbuf._clear(All);
                // (imdaveho) NOTE: might be too many "side effects" in a single
                // method call...this should be explicit for the user to batch
                // operations together; eg. clear all, goto(0,0), flush()
                // self.goto(0, 0);
                // (imdaveho) NOTE: there might be differences between ansi and
                // wincon that perhaps this "goto" normalizes...TEST this.
            }
            "newln" => {
                ansi_write(
                    &screen::ansi::clear(NewLn),
                    self.flush_if_auto);
                self.state.cellbuf._clear(NewLn);
            }
            "currentln" => {
                ansi_write(
                    &screen::ansi::clear(CurrentLn),
                    self.flush_if_auto);
                self.state.cellbuf._clear(CurrentLn);
                // self.goto(0, row); ??
            }
            "cursorup" => {
                ansi_write(
                    &screen::ansi::clear(CursorUp),
                    self.flush_if_auto);
                self.state.cellbuf._clear(CursorUp)
            }
            "cursordn" => {
                ansi_write(
                    &screen::ansi::clear(CursorDn),
                    self.flush_if_auto);
                self.state.cellbuf._clear(CursorDn);
            }
            // (imdaveho) TODO: CursorAt
            _ => ()
        }
    }

    pub fn resize(&mut self, w: i16, h: i16) {
        // NOTE (imdaveho): this method must call `flush`
        // otherwise nothing happens.
        ansi_write(
            &screen::ansi::resize(w, h),
            self.flush_if_auto);
        // let meta = &mut self.metas[self.index];
        // meta.screen_size = (w, h);
        // meta.backbuf.resize((w * h) as usize, None);
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
            // There is no point to switch if you're on another screen
            // since ANSI terminals provide a single "alternate screen".
            ansi_write(&screen::ansi::enable_alt(), self.flush_if_auto);
        } else {
            // If this wasn't a switch to the alternate screen (ie. the current
            // screen is already the alternate screen), then we need to clear it
            // without modifying the cellbuffer.
            ansi_write(&screen::ansi::clear(All), self.flush_if_auto);
        }
        // Push current self.state `Metadata` to stash and increment the index.
        // Swap the current self.state for a new Metadata struct.
        self.stash.push(self.state.clone());
        self.state = Metadata::new();
        self.index = self.stash.len();
        // Prevent multiple `flush()` calls due to `flush_if_auto` setting.
        let auto = self.flush_if_auto;
        if auto { self.manual() }
        // Explicitly set default screen settings.
        self.reset();
        self.cook();
        self.disable_mouse();
        self.show_cursor();
        self.goto(0, 0);
        // Revert back to previous `autoflush` configuration.
        if auto { self.automatic(); self.flush(); }
    }


    pub fn to_main(&mut self) {
        // Only execute if the User is not on the main screen buffer.
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
            ansi_write(&screen::ansi::disable_alt(), self.flush_if_auto)
        } else {
            if self.index == 0 {
                // Enable if you are already on main switching to an altscreen.
                ansi_write(&screen::ansi::enable_alt(), self.flush_if_auto)
            }
            ansi_write(&screen::ansi::clear(All), self.flush_if_auto);
            ansi_write(&cursor::ansi::goto(0, 0), self.flush_if_auto);
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
        
        // Prevent multiple `flush()` calls due to `autoflush` setting.
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
    }

    pub fn goto(&mut self, col: i16, row: i16) {
        // (imdaveho) NOTE: Disallow negative values.
        if col < 0 || row < 0 { return }
        ansi_write(
            &cursor::ansi::goto(col, row),
            self.flush_if_auto);
        self.state.cellbuf._reposition(col, row);
    }

    pub fn up(&mut self) {
        ansi_write(
            &cursor::ansi::move_up(1),
            self.flush_if_auto);
        self.state.cellbuf._sync_up(1);
    }

    pub fn dn(&mut self) {
        ansi_write(
            &cursor::ansi::move_down(1),
            self.flush_if_auto);
        self.state.cellbuf._sync_dn(1);
    }

    pub fn left(&mut self) {
        ansi_write(
            &cursor::ansi::move_left(1),
            self.flush_if_auto);
        self.state.cellbuf._sync_left(1);
    }

    pub fn right(&mut self) {
        ansi_write(
            &cursor::ansi::move_right(1),
            self.flush_if_auto);
        self.state.cellbuf._sync_right(1);
    }

    pub fn dpad(&mut self, dir: &str, n: i16) {
        // (imdaveho) NOTE: Only deal with non-negative `n`. We use i16 to
        // mirror types for getting cursor position and returning terminal size.
        if n < 0 { return }
        // Case-insensitive.
        match dir.to_lowercase().as_str() {
            "up" => {
                ansi_write(
                    &cursor::ansi::move_up(n),
                    self.flush_if_auto);
                self.state.cellbuf._sync_up(n);
            },
            "dn" => {
                ansi_write(
                    &cursor::ansi::move_down(n),
                    self.flush_if_auto);
                self.state.cellbuf._sync_dn(n);
            },
            "left" => {
                ansi_write(
                    &cursor::ansi::move_left(n),
                    self.flush_if_auto);
                self.state.cellbuf._sync_left(n);
            },
            "right" => {
                ansi_write(
                    &cursor::ansi::move_right(n),
                    self.flush_if_auto);
                self.state.cellbuf._sync_right(n);
            },
            _ => ()
        }
    }

    pub fn pos(&mut self) -> (i16, i16) {
        let err_message = "Error reading cursor position (I/O related)";
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
    }

    pub fn screen_pos(&mut self) -> (i16, i16) {
        self.state.cellbuf._screen_pos()
    }

    pub fn mark(&mut self) {
        // ansi_write(
        //     &cursor::ansi::save_pos(),
        //     self.flush_if_auto);
        self.state._mark_position();
    }

    pub fn load(&mut self) {
        // ansi_write(
        //     &cursor::ansi::load_pos(),
        //     self.flush_if_auto);
        // // TODO: On ANSI do we need to flush immediately after load so that the
        // // call to `pos()` can properly reposition the cellbuffer?
        let (col, row) = self.state._saved_position();
        self.goto(col, row);
    }

    pub fn hide_cursor(&mut self) {
        ansi_write(
            &cursor::ansi::hide(),
            self.flush_if_auto);
        self.state._hide_cursor();
    }

    pub fn show_cursor(&mut self) {
        ansi_write(
            &cursor::ansi::show(),
            self.flush_if_auto);
        self.state._show_cursor();
    }

    pub fn set_fg(&mut self, color: Color) {
        ansi_write(
            &output::ansi::set_style(Fg(color)),
            self.flush_if_auto);
        self.state.cellbuf._sync_style(Fg(color));
    }

    pub fn set_bg(&mut self, color: Color) {
        ansi_write(
            &output::ansi::set_style(Bg(color)),
            self.flush_if_auto);
        self.state.cellbuf._sync_style(Bg(color));
    }

    pub fn set_fx(&mut self, effects: Effects) {
        ansi_write(
            &output::ansi::set_style(Fx(effects)),
            self.flush_if_auto);
        self.state.cellbuf._sync_style(Fx(effects));
    }

    pub fn set_styles(&mut self, fgcol: Color, bgcol: Color, effects: Effects) {
        ansi_write(
            &output::ansi::set_styles(fgcol, bgcol, effects),
            self.flush_if_auto);
        self.state.cellbuf._sync_style(Fg(fgcol));
        self.state.cellbuf._sync_style(Bg(bgcol));
        self.state.cellbuf._sync_style(Fx(effects));
    }

    pub fn reset(&mut self) {
        ansi_write(
            &output::ansi::reset(),
            self.flush_if_auto);
        self.state.cellbuf._reset_style();
    }

    pub fn prints(&mut self, content: &str) {
        self.state.cellbuf._sync_buffer(content);
        ansi_write(&output::ansi::prints(content), self.flush_if_auto);
    }

    pub fn flush(&mut self) {
        ansi_flush();
    }

    pub fn printf(&mut self, content: &str) {
        self.prints(content);
        ansi_flush();
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
        self.to_main();
        self.cook();
        self.show_cursor();
        ansi_write("\n\r", true);
        self.stash.clear();
    }

}


impl Drop for Tty {
    fn drop(&mut self) {
        self.terminate();
    }
}
