// Unix specific modules.

mod input;
mod parser;
mod raw;

mod size;
pub use size::size;

mod pos;
pub use pos::pos;

mod reader;
pub use reader::{ SyncReader, AsyncReader };

use libc::termios as Termios;
use super::{ CommonTerminal, Clear, Color, Style, Style::* };

use crate::common::{
    meta::Metadata, enums::Direction,
    traits::*, cache::CacheUpdater,
};


pub struct UnixTerminal {
    index: usize,
    state: Metadata,
    stash: Vec<Metadata>,
    common: CommonTerminal,
    original_mode: Termios,
}

impl UnixTerminal {
    pub fn init() -> UnixTerminal {
        UnixTerminal {
            index: 0,
            state: Metadata::new(),
            stash: Vec::with_capacity(5),
            common: CommonTerminal::new(),
            original_mode: raw::get_mode()
                .expect("Error fetching Termios"),
        }
    }

    fn set_style(&mut self, style: Style) {
        self.common.set_style(style);
        self.state.cache._sync_style(style);
    }

    pub fn terminate(&mut self) {
        self.to_main();
        self.cook();
        self.show_cursor();
        self.common.printf("\n\r");
        self.stash.clear();
    }
}

impl TerminalCursor for UnixTerminal {
    fn goto(&mut self, col: i16, row: i16) {
        self.common.goto(col, row);
        self.state.cache._sync_pos(col, row);
    }

    fn up(&mut self) {
        self.common.up(1);
        self.state.cache._sync_up(1);
    }

    fn down(&mut self) {
        self.common.down(1);
        self.state.cache._sync_down(1);
    }

    fn left(&mut self) {
        self.common.left(1);
        self.state.cache._sync_left(1);
    }

    fn right(&mut self) {
        self.common.right(1);
        self.state.cache._sync_right(1);
    }

    fn pos(&mut self) -> (i16, i16) {
        let (col, row) = self.common.pos();
        self.state.cache._sync_pos(col, row);
        (col, row)
    }

    fn mark_pos(&mut self) {
        self.state._mark_position();
    }

    fn load_pos(&mut self) {
        let (col, row) = self.state._saved_position();
        self.goto(col, row);
    }

    fn moves(&mut self, direction: Direction) {
        match direction {
            Direction::Up(n) => {
                self.common.up(n);
                self.state.cache._sync_up(n);
            }
            Direction::Down(n) => {
                self.common.down(n);
                self.state.cache._sync_down(n);
            }
            Direction::Left(n) => {
                self.common.left(n);
                self.state.cache._sync_left(n);
            }
            Direction::Right(n) => {
                self.common.right(n);
                self.state.cache._sync_right(n);
            }
        }
    }
}

impl TerminalModifier for UnixTerminal {
    fn raw(&mut self) {
        raw::enable_raw().expect("Error enabling raw mode");
        self.state._raw();
    }

    fn cook(&mut self) {
        raw::set_mode(&self.original_mode).expect("Error disabling raw mode");
        self.state._cook();
    }

    fn hide_cursor(&mut self) {
        self.common.hide_cursor();
        self.state._hide_cursor();
    }

    fn show_cursor(&mut self) {
        self.common.show_cursor();
        self.state._show_cursor();
    }

    fn enable_mouse(&mut self) {
        self.common.enable_mouse();
        self.state._enable_mouse();
    }

    fn disable_mouse(&mut self) {
        self.common.disable_mouse();
        self.state._disable_mouse();
    }
}

impl TerminalFormatter for UnixTerminal {
    fn clear(&mut self, method: Clear) {
        self.common.clear(method);
        self.state.cache._clear_buffer(method);
        match method {
            Clear::All => self.goto(0, 0),
            Clear::CurrentLn => {
                let (_, row) = self.pos();
                self.goto(0, row);
            },
            _ => (),
        }
    }

    fn resize(&mut self, w: i16, h: i16) {
        self.common.resize(w, h);
        self.state.cache._sync_size(w, h);
    }

    fn set_fg(&mut self, color: Color) {
        self.set_style(Fg(color));
    }

    fn set_bg(&mut self, color: Color) {
        self.set_style(Bg(color));
    }

    fn set_fx(&mut self, effects: u32) {
        self.set_style(Fx(effects));
    }

    fn set_styles(&mut self, fg: Color, bg: Color, fx: u32) {
        self.common.set_styles(fg, bg, fx);
        self.state.cache._sync_styles(fg, bg, fx);
    }

    fn reset_styles(&mut self) {
        self.common.reset_styles();
        self.state.cache._reset_styles();
    }

    fn screen_pos(&self) -> (i16, i16) {
        self.state.cache._screen_pos()
    }

    fn screen_size(&self) -> (i16, i16) {
        self.state.cache._screen_size()
    }
}

impl TerminalWriter for UnixTerminal {
    fn prints(&mut self, content: &str) {
        self.state.cache._cache_content(content);
        self.common.prints(content);
    }

    fn flush(&mut self) {
        self.common.flush();
        // TODO: update cursor pos in cache on write or flush?
    }

    fn printf(&mut self, content: &str) {
        self.state.cache._cache_content(content);
        self.common.printf(content);
    }
}

impl TerminalInput for UnixTerminal {
    fn read_char(&self) -> char {
        input::read_char().expect("Error reading a character from stdin")
    }

    fn read_sync(&self) -> SyncReader {
        input::read_sync()
    }

    fn read_async(&self) -> AsyncReader {
        input::read_async()
    }

    fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        input::read_until_async(delimiter)
    }
}

impl TerminalSwitcher for UnixTerminal {
    fn switch(&mut self) {
        // In order to support multiple "screens", this function creates a new
        // Metadata entry which stores any screen specific changes that a User
        // might want to be restored when switching between screens.
        if self.index == 0 {
            // There is no point to switch if you're on another screen
            // since ANSI terminals provide a single "alternate screen".
            self.common.enable_alt();
        } else {
            // If this wasn't a switch to the alternate screen (ie. the current
            // screen is already the alternate screen), then we need to clear it
            // without modifying the cellbuffer.
            self.common.clear(Clear::All);
        }
        // Push current self.state `Metadata` to stash and increment the index.
        // Swap the current self.state for a new Metadata struct.
        self.stash.push(self.state.clone());
        self.state = Metadata::new();
        self.index = self.stash.len();
        // Explicitly set default screen settings.
        self.cook();
        self.disable_mouse();
        self.show_cursor();
        self.reset_styles();
        self.goto(0, 0);
        self.flush();
    }

    fn to_main(&mut self) {
        if self.index == 0 { return }
        self.switch_to(0);
    }

    fn switch_to(&mut self, index: usize) {
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
            self.common.disable_alt();
        } else {
            if self.index == 0 {
                // Enable as you are on the main screen switching to alternate.
                self.common.enable_alt();
            }
            self.common.clear(Clear::All);
            // Restore screen contents. Restore flushes.
            self.state.cache._flush_buffer();
            let (col, row) = self.state.cache._screen_pos();
            self.goto(col, row);
        }

        // Update `self.index` to the function argument `index`
        // (imdaveho) TODO: Confirm if main screen will have native buffer logs,
        // thereby not needing to restore content manually via library. Also,
        // because there is going to be output that is not from `tty` which is
        // not possible to save in the backbuf.
        self.index = index;

        let (raw, mouse, show) = (
            self.state._is_raw_on(),
            self.state._is_mouse_on(),
            self.state._is_cursor_on());

        // Restore settings based on metadata.
        if raw { self.raw() } else { self.cook() }
        if mouse { self.enable_mouse() } else { self.disable_mouse() }
        if show { self.show_cursor() } else { self.hide_cursor() }
        self.flush();
    }
}

impl Drop for UnixTerminal {
    fn drop(&mut self) {
        self.terminate();
    }
}
