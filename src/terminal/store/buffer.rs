// This module provides an internal representation of the contents that
// make up the terminal screen.
use std::cmp::Ordering;

use crate::common::{
    enums::{ Color::{*, self}, Effect, Style, Clear },
    unicode::{
        wcwidth::UnicodeWidthChar,
        grapheme::UnicodeGraphemes,
    },
};

#[cfg(unix)]
use crate::terminal::actions::posix;

#[cfg(windows)]
use crate::terminal::actions::win32;


#[derive(Clone)]
enum Content {
    Single(char),
    Double(char),
    Complex(Vec<char>),
    Pointer(usize, usize),
    Blank,
}


#[derive(Clone)]
pub struct Cell {
    content: Content,
    style: (Color, Color, u32),
    is_dirty: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            content: Content::Blank,
            style: (Reset, Reset, Effect::Reset as u32),
            is_dirty: false
        }
    }
}


pub struct Buffer {
    cursor: usize,
    cells: Vec<Cell>,
    capacity: usize,
    // window: (i16, i16),
    // tab_size: usize,
    // marked_pos: usize,
    // active_style: (Color, Color, u32)
}

impl Buffer {
    pub fn new() -> Self {
        #[cfg(unix)]
        let (w, h) = posix::size();
        #[cfg(windows)]
        let (w, h) = win32::size();
        let cursor = 0;
        let capacity = (w * h) as usize;
        let cells = vec![Default::default(); capacity];
        Self { cursor, cells, capacity }
    }

    pub fn cursor(&mut self) -> usize {
        let mut index = self.cursor;
        match self.cells.get(index) {
            Some(c) => match c.content {
                Content::Pointer(a, _) => a,
                _ => index
            },
            None => {
                // Could be out-of-bounds.
                let length = self.cells.len();
                match self.capacity.cmp(&length) {
                    // Scenario A: cell buffer length < capacity:
                    Ordering::Greater => {
                        // Pop from extra back into cells to get
                        // Label { Label }ack to len == capacity.
                        let cycles = self.capacity - length;
                        for _ in 0..cycles {
                            self.cells.push(Default::default());
                        }
                    },
                    // Scenario B: cell buffer length > capacity:
                    Ordering::Less => {
                        // Pop from cells into extra to get back
                        // to len == capacity.
                        let cycles = length - self.capacity;
                        for _ in 0..cycles {
                            self.cells.pop();
                        }
                    },
                    _ => (),
                }
                // Scenario C: no issues with buffer; cursor index just
                // out of bounds. Set cursor to last Cell in buffer:
                index = self.capacity - 1;
                self.cursor = index;
                match self.cells[index].content {
                    Content::Pointer(a, _) => a,
                    _ => index
                }
            }
        }
    }

    // pub fn next_idx(&mut self) -> usize {}
    // pub fn prev_idx(&mut self)

    pub fn patch(&mut self, s: &str) {
        // 1. get cursor
        let mut index = self.cursor();
        // Simple case (ascii-only)
        // let mut count = 0;
        // let chars = s.chars();
        // let patch = vec![];
        // for ch in chars {
        //     let cell = self.cells.get(index)
        //     count += 1
        // }
        // Below: complex case with CJK and multi-cell unicode
        // // 2. chunk the &str
        // let graphemes: Vec<&str> = UnicodeGraphemes::graphemes(s, true).collect();
        // // 3. iterate through chunks
        // let mut patch_buffer = vec![];
        // for s in graphemes {
        //     match s.width() {
        //         0 => patch_buffer.push(Content::Blank),
        //         1 => patch_buffer.push(Content::Single(
        //             s.chars().next().expect("Error patching Single"))),
        //         2 => {
        //             patch_buffer.push(Content::Double(
        //                 s.chars().next().expect("Error patching Double")));
        //             patch_buffer.push(Content::Pointer())
        //         },
        //         _ => Content::Complex(
        //             s.chars().collect())
        //     };
        //     patch_content.push(content);
        // }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grapheme_split() {
        let string = "He㓘o, 👦🏿  👩‍🔬 क्‍ष \t \0 \r\n";
        let graphemes: Vec<&str> = UnicodeGraphemes
            ::graphemes(string, true).collect();

        while let Some(s) = graphemes.next() {
            let mut chars = s.chars.peekable();
            for ch in chars {
           }
        }
    }

    #[test]
    fn test_ascii_length() {
        assert_eq!((2 + 2), 4)
    }
}
