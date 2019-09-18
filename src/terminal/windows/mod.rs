// Windows specific modules.

mod input;
mod parser;
mod reader;
mod info;

pub use reader::{ SyncReader, AsyncReader };

use super::wincon::Handle;
use super::wincon::output;
use super::{ SystemTerminalApi, CommonTerminalApi, CommonTerminal };
use crate::common::runtime;


struct WindowsConsole {
    index: usize,
    state: Metadata,
    stash: Vec<Metadata>,
    common: CommonTerminal,
    original_mode: u32,
    alternate: Handle,
}

// (imdaveho) NOTE: This should be identical to UnixTerminal.
impl CommonTerminalApi for WindowsConsole {
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

impl SystemTerminalApi for WindowsConsole {
    pub fn init() -> WindowsConsole {
        WindowsConsole {
            index: 0,
            state: Metadata::new(),
            stash: Vec::with_capacity(5),
            original_mode: {
                if !runtime::is_wincon_enabled() {
                    Handle::conout().expect("Error fetching $CONOUT")
                        .get_mode().expect("Error fetching mode from $CONOUT")
                } else { output::get_mode()
                .expect("Error fetching mode from $STDOUT") }
            },
            alternate: Handle::buffer()
                .expect("Error creating alternate Console buffer"),
        }
    }

    pub fn raw(&mut self) {
        output::enable_raw().expect("Error enabling raw mode");
        self.state._raw();
    }

    pub fn cook(&mut self) {
        output::disable_raw().expect("Error disabling raw mode");
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

    pub fn screen_pos(&self) -> (i16, i16) {
        self.state.cache._screen_pos()
    }

    pub fn screen_size(&self) -> (i16, i16) {
        self.state.cache._screen_size()
    }

    pub fn prints(&mut self, content: &str) {
        self.common.prints(content);
    }

    pub fn flush(&self) {
        self.common.flush();
    }

    pub fn printf(&mut self, content: &str) {
        self.common.printf(content);
    }

    pub fn terminate(&mut self) {
        self.to_main();
        self.alternate.close()
            .expect("Error closing the alternate Console buffer");
            
        let stdout = if runtime::is_wincon_enabled() {
            Handle::stdout().expect("Error fetching $STDOUT")
        } else { Handle::conout().expect("Error fetching $CONOUT") };
        
        stdout.set_mode(&self.original_mode)
            .expect("Error reseting the Console mode to default");
        self.common.show_cursor();
        self.common.printf("\n\r");
        self.stash.clear();
    }
}

impl Drop for WindowsConsole {
    fn drop(&mut self) {
        self.terminate();
    }
}