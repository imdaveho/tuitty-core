// Unix specific modules.

use libc::termios as Termios;

mod input;
mod parser;
mod reader;
mod size;
mod pos;
mod raw;

pub use size::size;
pub use pos::pos;
pub use raw::{ get_mode, set_mode, enable_raw };
pub use reader::{ SyncReader, AsyncReader };

use super::{ SystemTerminalApi, CommonTerminalApi, CommonTerminal };
use crate::common::{
    meta::Metadata,
    enums::{ Clear, Color, Effect, Style, Style::*, Direction },
};


struct UnixTerminal {
    index: usize,
    state: Metadata,
    stash: Vec<Metadata>,
    common: CommonTerminal,
    original_mode: Termios,
}

// (imdaveho) NOTE: This should be identical to WindowsConsole.
impl CommonTerminalApi for UnixTerminal {
    pub fn resize(&mut self, w: i16, h: i16) {
        self.common.resize(w, h);
        self.state.cache._sync_size(w, h);
    }

    pub fn goto(&mut self, col: i16, row: i16) {
        self.common.goto(col, row);
        self.state.cache._sync_pos(col, row);
    }

    pub fn up(&mut self) {
        self.common.up();
        self.state.cache._sync_up(1);
    }

    pub fn dn(&mut self) {
        self.common.dn();
        self.state.cache._sync_dn(1);
    }
    
    pub fn left(&mut self) {
        self.common.left();
        self.state.cache._sync_left(1);
    }
    
    pub fn right(&mut self) {
        self.common.right();
        self.state.cache._sync_right(1);
    }

    fn set_style(&mut self, style: Style) {
        self.common.set_style(style);
        self.state.cache._sync_style(style);
    }

    pub fn set_fg(&mut self, color: Color) {
        self.common.set_style(Fg(color));
        self.state.cache._sync_style(Fg(color);)
    }
    
    pub fn set_bg(&mut self, color: Color) {
        self.common.set_style(Bg(color));
        self.state.cache._sync_style(Bg(color);)
    }
    
    pub fn set_fx(&mut self, effects: Effect) {
        self.common.set_style(Fx(effects));
        self.state.cache._sync_style(Fx(effects);)
    }

    pub fn set_styles(&mut self, fg: Color, bg: Color, fx: u32) {
        self.set_styles(fg, bg, fx);
        self.state.cache._set_styles(fg, bg, fx);
    }

    pub fn reset_styles(&mut self) {
        self.common.reset_styles();
        self.state.cache._reset_styles();
    }

    pub fn clear(&mut self, method: Clear) {
        self.common.clear(method);
        self.state.cache._clear(method);
    }

    pub fn enable_mouse(&mut self) {
        self.common.enable_mouse();
        self.state._enable_mouse();
    }

    pub fn disable_mouse(&mut self) {
        self.common.disable_mouse();
        self.state._disable_mouse();
    }

    pub fn hide_cursor(&mut self) {
        self.common.hide_cursor();
        self.state._hide_cursor();
    }

    pub fn show_cursor(&mut self) {
        self.common.show_cursor();
        self.state._show_cursor();
    }

    pub fn pos(&mut self) -> (i16, i16) {
        let (col, row) = self.common.pos();
        self.state.cache._sync_pos(col, row);
        (col, row)
    }
}

impl SystemTerminalApi for UnixTerminal {
    pub fn init() -> UnixTerminal {
        UnixTerminal {
            index: 0,
            state: Metadata::new(),
            stash: Vec::with_capacity(5),
            original_mode: get_mode().expect("Error fetching Termios"),
        }
    }

    pub fn raw(&mut self) {
        enable_raw().expect("Error enabling raw mode");
        self.state._raw();
    }

    pub fn cook(&mut self) {
        set_mode(&self.original_mode).expect("Error disabling raw mode");
        self.state._cook();
    }

    pub fn read_char(&self) -> char {
        input::read_char().expect("Error reading a character from stdin")
    }

    pub fn read_sync(&self) -> SyncReader {
        input::read_sync()
    }

    pub fn read_async(&self) -> SyncReader {
        input::read_async()
    }

    pub fn read_until_sync(&self, delimiter: u8) -> SyncReader {
        input::read_sync(delimiter)
    }

    pub fn mark_pos(&mut self) {
        self.state._mark_position();
    }

    pub fn load_pos(&mut self) {
        let (col, row) = self.state._saved_position();
        self.goto(col, row);
    }

    pub fn screen_size(&self) -> (i16, i16) {
        self.state.cache._screen_size()
    }
    
    pub fn screen_pos(&self) -> (i16, i16) {
        self.state.cache._screen_pos()
    }

    pub fn moves(&mut self, direction: Direction) {
        match direction {
            Direction::Up(n) => {
                self.common.up(n));
                self.state.cache._sync_up(n);
            }
            Direction::Dn(n) => {
                self.common.dn(n));
                self.state.cache._sync_dn(n);
            }
            Direction::Left(n) => {
                self.common.left(n));
                self.state.cache._sync_left(n);
            }
            Direction::Right(n) => {
                self.common.right(n));
                self.state.cache._sync_right(n);
            }
        }
    }

    pub fn prints(&mut self, content: &str) {
        self.state.cache._sync_content(content);
        self.common.prints(content);
    }

    pub fn flush(&self) {
        self.common.flush();
    }

    pub fn printf(&mut self, content: &str) {
        self.state.cache._sync_content(content);
        self.common.printf(content);
    }

    pub fn terminate(&mut self) {
        self.to_main();
        self.cook();
        self.show_cursor();
        self.common.printf("\n\r");
        self.stash.clear();
    }
}

impl Drop for UnixTerminal {
    fn drop(&mut self) {
        self.terminate();
    }
}