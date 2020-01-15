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


#[derive(Clone, PartialEq)]
enum Cell {
    Single{ value: char, width: usize, style: (Color, Color, u32) },
    Double{ value: char, width: usize, style: (Color, Color, u32) },
    Vector{ value: Vec<char>, width: usize, style: (Color, Color, u32) },
    // Linker is used for complex unicode values kept in a Vector. It
    // contains index offsets to the left and right from its position.
    Linker(usize, usize),
    TAB, NIL
}


// e.g. ansi
// [ goto(x,y), print(s), goto(x, y), print(s), ..., goto(cursor) ] + flush()
// e.g. windows (unused)
// [ writeconsoleoutput{ rect }, writeconsoleoutput{ rect }, ... ]
// In both cases, the cursor returns to where it needs to be after executing.
enum Edit {
    Index(usize),
    Content(String),
    Styling(Color, Color, u32),
}


pub struct Buffer {
    index: usize,
    cells: Vec<Cell>,  // for get ch at point
    edits: Vec<Edit>,  // for updating stdout
    width: i16,
    height: i16,
    style: (Color, Color, u32),
    tabwidth: usize,
    savedpos: usize,
    // treatment: usize, // 0: no zwj; 1: zwj; 2: zwj+fitz; default: 0
}

impl Buffer {
    pub fn new() -> Self {
        #[cfg(unix)]
        let (width, height) = posix::size();
        #[cfg(windows)]
        let (width, height) = win32::size();
        let capacity = (width * height) as usize;
        Self {
            index: 0,
            cells: vec![Cell::NIL; capacity],
            edits: Vec::with_capacity(12),
            width,
            height,
            style: (Reset, Reset, Effect::Reset as u32),
            tabwidth: 8,
            savedpos: 0,
            // treatment: 0,
        }
    }

    // pub fn treatment(&mut self, level: usize) {
    //     self.treatment = level;
    // }

    // Returns a coordinate tuple from an index.
    // Does NOT update internal index.
    pub fn coord(&self, index: usize) -> (i16, i16) {
        let width = self.width;
        let index = index as i16;
        ((index % width), (index / width))
    }

    // Returns an index from a coordinate tuple.
    // Does NOT update internal index.
    pub fn index(&self, coord: (i16, i16)) -> usize {
        let (mut col, mut row) = (coord.0, coord.1);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }
        ((row * self.width) + col) as usize
    }

    // Returns t next tabstop given a tab length from a coordinate tuple.
    // Does NOT update internal index.
    pub fn tabstop(&self, coord: (i16, i16)) -> usize {
        let (col, row) = (coord.0, coord.1);
        // 1. handle new tab stop:
        let prev_stop = (col as usize / self.tabwidth) * self.tabwidth;
        let mut new_stop = (prev_stop + self.tabwidth) as i16;
        let width = self.width - 1;
        if new_stop > width { new_stop = width }
        // 2. update cursor and return:
        ((row * self.width) + new_stop) as usize
    }

    // Returns the index shifted left a col.
    // Does NOT update internal index.
    fn index_left(&self, index: usize, n: i16) -> usize { 0 }

    // Returns the index shifted right a col.
    // Does NOT update internal index.
    fn index_right() {}

    // Returns the index shifted up a row.
    // Does NOT update internal index.
    fn index_up() {}

    // Returns the index shifted down a row.
    // Does NOT update internal index.
    fn index_down(&self, index: usize, n: i16) -> usize {
        let mut n = n;
        if n < 0 { n = n.abs() }
        let (col, mut row) = self.coord(index);
        let maxrow = self.height - 1;
        if row + n >= maxrow { row = maxrow } else { row += n }
        self.index((col, row))
    }

    // Returns a cleaned index after bounds checking. Always provides
    // an index at the beginning of a Cell (no Spacers).
    // NOTE: this WILL update the internal index AFTER first setting
    // `self.index` -- use with `coord`, `index`, and `tabstop`.
    fn cursor(&mut self) -> usize {
        match self.cells.get(self.index) {
            Some(Cell::Linker(offset, _)) => self.index -= offset,
            Some(_) => (),
            None => {
                // Could be out-of-bounds.
                let length = self.cells.len();
                let capacity = (self.width * self.height) as usize;
                match capacity.cmp(&length) {
                    // Scenario A: cell buffer length < capacity:
                    Ordering::Greater => {
                        // Pop from extra back into cells to get
                        // Label { Label }ack to len == capacity.
                        let cycles = capacity - length;
                        for _ in 0..cycles {
                            self.cells.push(Cell::NIL);
                        }
                    },
                    // Scenario B: cell buffer length > capacity:
                    Ordering::Less => {
                        // Pop from cells into extra to get back
                        // to len == capacity.
                        let cycles = length - capacity;
                        for _ in 0..cycles {
                            self.cells.pop();
                        }
                    },
                    // Scenario C: no issues with buffer; cursor index just
                    // out of bounds. Set cursor to last Cell in buffer:
                    Ordering::Equal => {
                        self.index = capacity - 1;
                    },
                }
                // Should always be a valid index after the above:
                if let Cell::Linker(offset, _) = self.cells[self.index] {
                    self.index -= offset;
                }
            }
        }

        self.index
    }


    fn compare(&mut self, this: &Cell, data: (&usize, &usize, &usize)) {
        let that = &self.cells[index];
        if this == that {
            let size = std::mem::size_of_val(cluster);
            if eq_memsize + size >= 8 {
                let length = outstr.len();
                outstr.truncate(length - eq_memsize);
                self.edits.push(Edit::Content(outstr));
                outstr = String::with_capacity(
                    self.width as usize);
            } else {
                eq_memsize += size;
                outstr.push(car);
            }
        } else {

        }
    }

    #[cfg(unix)]
    pub fn parse(&mut self, s: &str) {
        // let mut index = self.cursor();
        // let mut outstr = String::with_capacity(self.width as usize);
        // let mut offset = 0;
        // let mut memory = 8;
        struct ``(usize, usize, usize, String);
        let

        for cluster in UnicodeGraphemes::graphemes(s, true) {
            let mut chars = cluster.chars().peekable();
            if let Some(car) = chars.next() { match chars.peek() {
                // A single grapheme - can be ascii, cjk, or escape seq:
                // char.width() returns the character's displayed
                // width in columns, or `None` if the character
                // is a control character other than `'\x00'`.
                None => match car.width() {
                    // Ascii or CJK
                    Some(w) => match w {
                        0 => continue,
                        1 => {
                            index += w;
                            let cell = Cell::Single{
                                value: car,
                                width: w,
                                style: self.style
                            };

                            // Diff Cell
                            // memsize = 0;
                            // match compare {
                            //     Cell::Single{value: ch, width: _, style: s} => (),
                            //     Cell::Double{value: ch, width: _, style: s} => (),
                            //     Cell::Vector{value: ch, width: _, style: s} => (),
                            //     Cell::Linker(_, _) => unreachable!(),
                            //     Cell::TAB => (),
                            //     Cell::NIL => self.cells[index] = cell,
                            // }
                        },
                        2 => {
                            // parsed.push(Cell::Double{
                            //     value: car,
                            //     width: 2,
                            //     style: self.style
                            // });
                            // parsed.push(Cell::Joiner(1, 1));
                            // index += 2;
                        },
                        _ => continue,
                    },
                    // Escape character
                    None => match car {
                        '\t' => {
                            // parsed.push(Cell::TAB);
                            // let n = self.tabstop(
                            //     self.coord(index)) - index;
                            // for i in 0..n {
                            //     let m = i + 1;
                            //     let (left, right) = (m, n - m);
                            //     parsed.push(Cell::Joiner(left, right));
                            // }
                            // index += n;
                        },
                        '\n' => {
                            // #[cfg(unix)] { index = self.index_down(index, 1) }
                            // // LF (`\n`) is treated like CRLF (`\r\n`)
                            // #[cfg(windows)] {
                            //     let row = (index as i16) / width + 1;
                            //     if self.height > row {
                            //         index = self.index((0, row));
                            //     } else {
                            //         index = self.index((0, self.height - 1))
                            //     }
                            // }
                            // // TODO: send parsed to edit table
                            // // parsed.clear();
                        },
                        '\r' => {
                            let row = (index as i16) / self.width;
                            index = self.index((0, row));
                            // TODO: send parsed to edit table
                            // parsed.clear();
                        },
                        // '\x1B' => parsed.push(Cell::Single{
                        //     value: '^',
                        //     width: 1,
                        //     style: self.style
                        // }),
                        _ => continue,
                    }
                },
                Some(cadr) => (),
            }}
        }
    }



    //
    // fn set_cell(&mut self, index: usize, cell: Cell, parsed: &mut Vec<Cell>) {
    //     // 1. Push into the parsed buffer.
    //     parsed.push(cell);
    //     // 2. Get number of Spacers per Cell variant.
    //     let coord = self.coord(index);
    //     let width = match parsed.last().unwrap() {
    //         Cell::Tab => self.tabstop(coord) - index,
    //         #[cfg(unix)]
    //         Cell::LineFeed => self.down(index, 1) - index,
    //         #[cfg(windows)]
    //         Cell::LineFeed => {},
    //         Cell::Double{ value: _, width: _, style: _ } => {},
    //         Cell::Scalar{ value: _, width: _, style: _ } => {},
    //         // Spacer is never passed in as a parameter to be set.
    //         // It gets automatically set via this function, therefore
    //         // it is an unreachable variant in this match statement.
    //         Cell::Spacer(_, _) => unreachable!(),
    //         // Blank and Single
    //         _ => 0,
    //     }
    //     // 3. Push each Spacer into the parsed buffer.
    //     for i in 1..=width {
    //         parsed.push(Cell::Spacer(i, width - i));
    //     }
    // }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {
        assert_eq!((2 + 2), 4)
    }

    #[test]
    fn test_ascii_buffer() {
        // let input = "Hello, World!";
        // let index = 18;
        // let mut graphemes = UnicodeGraphemes::graphemes(input, true);
        // while let Some(cluster) = graphemes.next() {
        //     let mut chars = cluster.chars().peekable();
        //     if let Some(car) = chars.next() { match chars.peek() {

        //     }}
        // }
    }

}
