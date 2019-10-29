// This module provides the Store which synchronizes application state with
// dispatched user actions and maintains settings across each "screen".

use std::collections::VecDeque;
use crate::common::{
    unicode::{grapheme::*, wcwidth::*},
    enums::{ Color::{*, self}, Effect, Style, Clear }
};

#[cfg(unix)]
use crate::terminal::actions::posix;

#[cfg(windows)]
use crate::terminal::actions::win32;


#[derive(Clone)]
struct Cell {
    glyph: Option<Vec<char>>,
    width: usize,
    style: (Color, Color, u32),
    index: usize,
}


pub struct Store {
    // Screen configuration
    tabsize: i16,
    width: i16,
    height: i16,
    // Screen mode settings
    is_raw_enabled: bool,
    is_mouse_enabled: bool,
    is_cursor_visible: bool,
    is_insert_mode: bool,
    // Current updates
    current_ch: Vec<char>,
    current_pos: (i16, i16),
    current_style: (Color, Color, u32),
    marked_pos: (i16, i16),
    // Screen buffer
    cell_buffer: VecDeque<Option<Cell>>
}

impl Store {
    pub fn new() -> Store {
        #[cfg(unix)]
        let (w, h) = posix::size();
        #[cfg(windows)]
        let (w, h) = win32::size();
        let capacity = (w * h) as usize;
        Store {
            tabsize: 4,
            width: w,
            height: h,
            is_raw_enabled: false,
            is_mouse_enabled: false,
            is_cursor_visible: true,
            is_insert_mode: false,
            current_ch: vec![],
            current_pos: (0, 0),
            current_style: (Reset, Reset, Effect::Reset as u32),
            marked_pos: (0, 0),
            cell_buffer: vec![None; capacity].into(),
        }
    }

    pub fn pos(&self) -> (i16, i16) {
        self.current_pos
    }

    pub fn set_pos(&mut self, col: i16, row: i16) {
        // Set from internal call to fetch cursor position.
        self.current_pos = (col, row);
    }

    pub fn size(&self) -> (i16, i16) {
        (self.width, self.height)
    }

    pub fn set_size(&mut self, w: i16, h: i16) {
        // Set from internal call to fetch screen size.
        // eg. after a screen resize or SIGWINCH.
        self.width = w;
        self.height = h;
    }

    pub fn marked_pos(&self) -> (i16, i16) {
        self.marked_pos
    }

    pub fn tabsize(&self) -> i16 {
        self.tabsize
    }

    pub fn set_tabsize(&mut self, size: i16) {
        self.tabsize = size;
    }

    pub fn getch(&self) -> String {
        self.current_ch.iter().collect::<String>()
    }

    pub fn getchw(&self) -> i16 {
        let (col, row) = self.current_pos;
        let index = self.from_pos(col, row);
        match self.cell_buffer.get(index) {
            Some(Some(cell)) => cell.width as i16,
            _ => 1 as i16
        }
    }

    fn tabstop(&self) -> usize {
        let (col, _) = self.current_pos;
        let mut tabstop = (col / self.tabsize)
            * self.tabsize + self.tabsize;
        if tabstop > self.width - 1 {
            tabstop = self.width - 1
        }
        tabstop as usize
    }

    fn from_index(&self, index: usize) -> (i16, i16) {
        let index = index as i16;
        ((index % self.width), (index / self.width))
    }

    fn from_pos(&self, col: i16, row: i16) -> usize {
        let (mut col, mut row) = (col, row);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }
        ((row * self.width) + col) as usize
    }

    fn sync_pos(&mut self, index: usize) {
        // Scenarios:
        // | index, cell | Description                                    |
        // ================================================================
        // |   o  ,  oo  | Valid index; use to return char(c)             |
        // |   o  ,  xx  | None cell; a valid case for a blank char(' ')  |
        // |   o  ,  ox  | Wide cell; use cell index to fetch origin char |
        // |   x  ,  xx  | Index out of bounds; reset index to cap or 0   |
        // ================================================================
        let mut index = index;
        loop {
            match self.cell_buffer.get(index) {
                Some(Some(cell)) => match &cell.glyph {
                    Some(v) => {
                        self.current_ch = v.to_vec();
                        self.current_pos = self.from_index(index);
                        break
                    },
                    None => index = cell.index,
                },
                Some(None) => {
                    self.current_ch = vec![' '];
                    self.current_pos = self.from_index(index);
                    break
                },
                None => {
                    let capacity = (self.width * self.height) as usize;
                    if index >= capacity { index = capacity - 1 }
                    else { index = 0 }
                },
            }
        }
    }

    pub fn sync_goto(&mut self, col: i16, row: i16) {
        let (mut col, mut row) = (col, row);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }

        self.sync_pos(self.from_pos(col, row));
    }

    pub fn sync_left(&mut self, n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        let (col, row) = self.current_pos;
        let mut index = self.from_pos(col, row);

        if n as usize >= index { index = 0 }
        else { index -= n as usize }

        self.sync_pos(index);

    }

    pub fn sync_right(&mut self, n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        let (col, row) = self.current_pos;
        let mut index = self.from_pos(col, row);

        let capacity = self.cell_buffer.len();
        if index + n as usize >= capacity { index = capacity - 1 }
        else { index += n as usize }

        self.sync_pos(index);
    }

    pub fn sync_up(&mut self, n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        let (col, row) = self.current_pos;
        let index = if n >= row { self.from_pos(col, 0) }
        else { self.from_pos(col, row - n) };

        self.sync_pos(index);
    }

    pub fn sync_down(&mut self, n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        let (col, row) = self.current_pos;
        let index = if row + n >= self.height - 1 {
            self.from_pos(col, self.height - 1)
        } else {
            self.from_pos(col, row + n)
        };

        self.sync_pos(index);
    }

    pub fn sync_style(&mut self, style: Style) {
        match style {
            Style::Fg(c) => self.current_style.0 = c,
            Style::Bg(c) => self.current_style.1 = c,
            Style::Fx(f) => self.current_style.2 = f,
        }
    }

    pub fn reset_style(&mut self) {
        self.current_style = (Reset, Reset, Effect::Reset as u32);
    }

    pub fn sync_mark(&mut self) {
        self.marked_pos = self.current_pos;
    }

    pub fn reset_mark(&mut self) {
        self.marked_pos = (0, 0);
    }

    pub fn sync_raw(&mut self, state: bool) {
        self.is_raw_enabled = state;
    }

    pub fn sync_cursor(&mut self, state: bool) {
        self.is_cursor_visible = state;
    }

    pub fn sync_mouse(&mut self, state: bool) {
        self.is_mouse_enabled = state;
    }

    pub fn sync_insert(&mut self, state: bool) {
        self.is_insert_mode = state;
    }

    // (imdaveho) NOTE: Caution! There be dragons until further observation.
    pub fn upsert_buffer(&mut self, content: &str) {
        let (w, h) = (self.width as usize, self.height as usize);
        let (col, row) = self.current_pos;
        let capacity = (w * h) as usize;
        let mut index = self.from_pos(col, row);
        let segments: Vec<&str> = UnicodeGraphemes
            ::graphemes(content, true).collect();
        for ch in segments {
            // Check capacity and truncate to keep at capacity.
            if self.cell_buffer.get(index).is_none() {
                self.cell_buffer.rotate_left(w);
                index = capacity - w;
                for i in index..capacity {
                    self.cell_buffer[i] = None;
                }
            }
            // Ascii segments are single width characters.
            if ch.is_ascii() {
                match ch {
                    "\x00" => continue, // Skip!
                    "\r" => { index = (index / w) * w; continue },
                    "\n" => {
                        let (_, r) = self.from_index(index);
                        let stop = self.from_pos(0, r + 1);
                        for _ in index..stop {
                            self.cell_buffer.pop_back();
                            self.cell_buffer.insert(index, None);
                            index += 1;
                        } continue
                    },
                    "\r\n" => {
                        index = (index / w) * w;
                        let (_, r) = self.from_index(index);
                        let stop = self.from_pos(0, r + 1);
                        for _ in index..stop {
                            self.cell_buffer.pop_back();
                            self.cell_buffer.insert(index, None);
                            index += 1;
                        } continue
                    },
                    "\t" => {
                        let stop = self.tabstop();
                        let length = stop - col as usize;
                        for i in (col as usize)..stop {
                            let mut ch = Some(vec![' '; length]);
                            if i > col as usize { ch = None }
                            self.cell_buffer[index + i] =
                                Some(Cell {
                                    glyph: ch,
                                    index: index,
                                    width: length,
                                    style: self.current_style
                                });
                        }
                        index += length; continue
                    },
                    _ => {
                        let c: Vec<char> = if ch == "\x1B"
                        { vec!['^'] } else { ch.chars().collect() };
                        self.cell_buffer[index] = Some(Cell {
                            glyph: Some(c),
                            width: 1,
                            index: index,
                            style: self.current_style,
                        });
                        index += 1; continue
                    }
                }
            }
            // Unicode: CJK, Emoji, and other.
            let c = ch.chars().collect::<Vec<char>>();
            match ch.width() {
                1 => {
                    self.cell_buffer[index] = Some(Cell {
                        glyph: Some(c),
                        width: 1,
                        index: index,
                        style: self.current_style,
                    });
                    index += 1; continue
                },
                2 => {
                    self.cell_buffer[index] = Some(Cell {
                        glyph: Some(c),
                        width: 2,
                        index: index,
                        style: self.current_style,
                    });
                    self.cell_buffer[index + 1] = Some(Cell {
                        glyph: None,
                        width: 2,
                        index: index,
                        style: self.current_style,
                    });
                    index += 2; continue
                },
                // (imdaveho) NOTE: Not going to handle complex combiner chars
                // until there is a better way to detecting how many cells is
                // going to be taken up or until there is a consistent font to
                // recommend across platforms to handle them.
                _ => (),
            }
        }
        self.sync_pos(index);
    }

    // (imdaveho) NOTE: Caution! There be dragons until further observation.
    // pub fn insert_buffer(&mut self, content: &str) {
    //     // TODO
    //     content;
    //     ();
    // }

    pub fn output_buffer(&self) -> String {
        let capacity = (self.width * self.height + self.width * 2) as usize;
        let mut chars: Vec<char> = Vec::with_capacity(capacity);
        for optc in self.cell_buffer.iter() {
            match optc {
                Some(cell) => match &cell.glyph {
                    Some(vc) => for c in vc {
                        chars.push(*c);
                    },
                    None => (),
                },
                None => chars.push(' '),
            }
        }
        chars.iter().collect::<String>()
    }

    pub fn clear_buffer(&mut self, clear: Clear) {
        match clear {
            Clear::All => {
                let capacity = (self.width * self.height) as usize;
                self.cell_buffer = vec![None; capacity].into();
            }
            Clear::NewLn => {
                let (w, (col, row)) = (self.width, self.current_pos);
                let (here, there) = ((row * w) + col, (row + 1) * w);
                for i in (here as usize)..(there as usize) {
                    self.cell_buffer[i] = None;
                }
            }
            Clear::CurrentLn => {
                let (w, row) = (self.width, self.current_pos.1);
                let (here, there) = ((row * w), (row + 1) * w);
                for i in (here as usize)..(there as usize) {
                    self.cell_buffer[i] = None;
                }
            }
            Clear::CursorUp => {
                let (w, (col, row)) = (self.width, self.current_pos);
                let here = (row * w) + col;
                for i in 0..(here as usize) {
                    self.cell_buffer[i] = None;
                }
            }
            Clear::CursorDn => {
                let ((w, h), (col, row)) = (
                    (self.width, self.height), self.current_pos);
                let (here, there) = ((row * w) + col, w * h);
                for i in (here as usize)..(there as usize) {
                    self.cell_buffer[i] = None;
                }
            }
        }
    }

}



#[cfg(test)]
mod tests {
    #[test]
    fn test_getch() {
        let content = "Hello, t㓘here!\n01234567";
        let mut store = super::Store::new();
        assert_eq!((0, 0), store.pos());
        assert_eq!((86, 30), store.size());
        store.upsert_buffer(content);
        assert_eq!(store.pos(), (8, 1));

        // Test: End of last row.
        store.sync_goto(85, 1);
        assert_eq!(store.getch(), " ");

        // Test: Start of new line.
        store.sync_goto(0, 1);
        assert_eq!(store.getch(), "0");

        // Test: Single char goto.
        store.sync_goto(2, 0);
        assert_eq!(store.getch(), "l");

        // Test: Wide char goto.
        store.sync_goto(2, 1);
        assert_eq!(store.getch(), "2");

        // Test: Move right 1 character.
        store.sync_goto(7, 0);
        assert_eq!(store.getch(), "t");
        store.sync_goto(9, 0);
        assert_eq!(store.getch(), "㓘");
        store.sync_goto(8, 0);
        store.sync_right(store.getchw());
        assert_eq!(store.getch(), "h");

        // Test: Move left 1 character.
        // NOTE: Syncing always moves to the start
        // of wide characters. So, when operating
        // to the left, we can shift by 1 -- this
        // makes less variation. When backspacing,
        // we can shift left by 1, getchw and remove
        // the wide cells by going forward. If this
        // wasn't the case, we would have to handle
        // moving the cursor to both ends vs just one.
        store.sync_left(1);
        store.sync_left(1);
        assert_eq!(store.getch(), "t");

        // Test: Move down by 1.
        store.sync_down(1);
        assert_eq!(store.getch(), "7");

        // Test: Move up by 1.
        store.sync_right(2 + 1);
        store.sync_up(1);
        assert_eq!(store.getch(), "h");
    }

}
