// This module provides the Store which synchronizes application state with
// dispatched user actions and maintains settings across each "screen".

// mod buffer;
mod cell;
use cell::ScreenBuffer;

use crate::common::enums::{ Clear, Color, Style };


struct Screen {
    // Screen mode settings
    is_raw_enabled: bool,
    is_mouse_enabled: bool,
    is_cursor_visible: bool,
    // Screen buffer
    buffer: ScreenBuffer,
}


impl Screen {
    pub fn new() -> Screen {
        Screen {
            is_raw_enabled: false,
            is_mouse_enabled: false,
            is_cursor_visible: true,
            buffer: ScreenBuffer::new(),
        }
    }
}


pub struct Store {
    id: usize,
    data: Vec<Screen>,
}

impl Store {
    pub fn new() -> Store {
        Store { id: 0, data: vec![Screen::new()] }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn exists(&self, id: usize) -> bool {
        self.data.get(id).is_some()
    }

    pub fn set(&mut self, id: usize) {
        if let Some(_) = self.data.get(id) {
            self.id = id
        }
    }

    pub fn new_screen(&mut self) {
        self.data.push(Screen::new());
        self.id = self.data.len() - 1;
    }

    pub fn coord(&self) -> (i16, i16) {
        if let Some(s) = self.data.get(self.id) {
            s.buffer.coord()
        } else { (0, 0) }
    }

    pub fn size(&self) -> (i16, i16) {
        if let Some(s) = self.data.get(self.id) {
            s.buffer.size()
        } else { (0, 0) }
    }

    #[cfg(unix)]
    pub fn render(&self) {
        if let Some(s) = self.data.get(self.id) {
            s.buffer.render()
        }
    }

    #[cfg(windows)]
    pub fn render(&self, reset: u16, vte: bool) {
        if let Some(s) = self.data.get(self.id) {
            s.buffer.render(reset, vte)
        }
    }

    pub fn getch(&self) -> String {
        if let Some(s) = self.data.get(self.id) {
            s.buffer.getch()
        } else { String::new() }
    }

    pub fn is_raw(&self) -> bool {
        if let Some(s) = self.data.get(self.id) {
            s.is_raw_enabled
        } else { false }
    }

    pub fn sync_raw(&mut self, state: bool) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.is_raw_enabled = state;
        }
    }

    pub fn is_cursor(&self) -> bool {
        if let Some(s) = self.data.get(self.id) {
            s.is_cursor_visible
        } else { false }
    }

    pub fn sync_cursor(&mut self, state: bool) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.is_cursor_visible = state;
        }
    }

    pub fn is_mouse(&self) -> bool {
        if let Some(s) = self.data.get(self.id) {
            s.is_mouse_enabled
        } else { false }
    }

    pub fn sync_mouse(&mut self, state: bool) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.is_mouse_enabled = state;
        }
    }

    pub fn sync_goto(&mut self, col: i16, row: i16) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.buffer.sync_coord(col, row);
        }
    }

    pub fn sync_left(&mut self, n: i16) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.buffer.sync_left(n);
        }
    }

    pub fn sync_right(&mut self, n: i16) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.buffer.sync_right(n);
        }
    }

    pub fn sync_up(&mut self, n: i16) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.buffer.sync_up(n);
        }
    }

    pub fn sync_down(&mut self, n: i16) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.buffer.sync_down(n);
        }
    }

    pub fn jump(&mut self) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.buffer.jump();
        }
    }

    pub fn sync_marker(&mut self, col: i16, row: i16) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.buffer.sync_marker(col, row);
        }
    }

    pub fn sync_size(&mut self, w: i16, h: i16) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.buffer.sync_size(w, h);
        }
    }

    pub fn sync_tab_size(&mut self, n: usize) {
        // TODO: include a process Command into tabs
        // to ensure that system tabs is aligned.
        if let Some(s) = self.data.get_mut(self.id) {
            s.buffer.sync_tab_size(n);
        }
    }

    pub fn sync_content(&mut self, content: &str) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.buffer.sync_content(content);
        }
    }

    pub fn sync_style(&mut self, style: Style) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.buffer.sync_style(style);
        }
    }

    pub fn sync_styles(&mut self, f: Color, b: Color, fx: u32) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.buffer.sync_styles(f, b, fx);
        }
    }

    pub fn sync_clear(&mut self, clr: Clear) {
        if let Some(s) = self.data.get_mut(self.id) {
            s.buffer.sync_clear(clr);
        }
    }
}
