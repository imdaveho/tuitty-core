// This module provides an internal representation of the contents that
// make up the terminal screen.
use std::cmp::Ordering;
use std::mem::size_of_val;

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
    Single(char, (Color, Color, u32)),
    Double(char, (Color, Color, u32)),
    Multi(Vec<char>, usize, (Color, Color, u32)),
    Link, NIL
}


pub struct Buffer {
    index: usize,
    cells: Vec<Cell>,
    strbuf: String,
    pub width: i16,
    pub height: i16,
    pub style: (Color, Color, u32),
    pub savedpos: usize,
    pub tabwidth: usize,
    pub use_winapi: bool,
    pub can_modify: bool,
    // NOTE: We don't support emoji joiners (yet). The reason is due to
    // there being few fonts and systems that support it well. While OSX
    // does provide proper rendering, the cursor position gets messy. It
    // is better to constrain to standalone emojis, but store the full
    // contents in the Cell enum for copy/paste purposes...
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
            strbuf: String::with_capacity(capacity),
            width, height,
            style: (Reset, Reset, Effect::Reset as u32),
            savedpos: 0,
            tabwidth: 4,
            use_winapi: false,
            can_modify: false,
        }
    }

    // #[cfg(unix)]
    // pub fn check_mod(&mut self) -> i16 {
    //     let test = &["🧗", "🏽", "\u{200d}", "♀", "\u{fe0f}"].concat();
    //     // TODO: Replace this once actions are unified
    //     posix::enable_alt();
    //     let mode = posix::get_mode();
    //     posix::raw();
    //     posix::goto(0, 0);
    //     posix::printf(test);
    //     let (col, _) = pos_raw();
    //     posix::cook(&mode);
    //     posix::disable_alt();
    //     col
    // }

    pub fn resize(&mut self, w: i16, h: i16) {
        self.width = w;
        self.height = h;
        let capacity = (w * h) as usize;
        self.cells.resize(capacity, Cell::NIL);
    }

    pub fn clear(&mut self, c: Clear) {
        match c {
            Clear::All => {
                let capacity = (self.width * self.height) as usize;
                self.cells = vec![Cell::NIL; capacity];
                self.index = 0;
            }
            Clear::NewLn => {
                let index = self.cursor();
                let (w, (col, row)) = (self.width,
                                       self.coord(index));
                let (start, stop) = (
                    ((row * w) + col) as usize,
                    ((row + 1) * w) as usize );
                for i in start..stop { self.cells[i] = Cell::NIL }
            }
            Clear::CurrentLn => {
                let index = self.cursor();
                let (w, (_, row)) = (self.width,
                                     self.coord(index));
                let (start, stop) = (
                    (row * w) as usize,
                    ((row + 1) * w) as usize );
                for i in start..stop { self.cells[i] = Cell::NIL }
                self.goto_coord(0, row);
            }
            Clear::CursorUp => {
                let index = self.cursor();
                let (w, (col, row)) = (self.width,
                                       self.coord(index));
                let stop = ((row * w) + col) as usize;
                for i in 0..stop { self.cells[i] = Cell::NIL }
            }
            Clear::CursorDn => {
                let index = self.cursor();
                let ((w, h), (col, row)) = (self.size(),
                                            self.coord(index));
                let (start, stop) = (
                    ((row * w) + col) as usize,
                    (w * h) as usize );
                for i in start..stop { self.cells[i] = Cell::NIL }
            }
        }
    }

    // Buffer navigation specific functions.
    //
    // Returns a coordinate tuple from an index.
    // NOTE: Does NOT update internal index.
    pub fn coord(&self, index: usize) -> (i16, i16) {
        let width = self.width;
        let index = index as i16;
        ((index % width), (index / width))
    }

    // Returns an index from a coordinate tuple.
    // NOTE: Does NOT update internal index.
    pub fn index(&self, coord: (i16, i16)) -> usize {
        let (mut col, mut row) = (coord.0, coord.1);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }
        ((row * self.width) + col) as usize
    }

    // Returns t next tabstop given a tab length from a coordinate tuple.
    // NOTE: Does NOT update internal index.
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

    // // Returns the index shifted left a col.
    // // Does NOT update internal index.
    // pub fn index_left(&self, index: usize, n: i16) -> usize {
    //     if n < 0 { return self.index_right(index, n.abs()) }
    //     let (col, row) = self.coord(index);
    //     let mincol = 0;
    //     let newcol = col - n;
    //     if newcol <= mincol {
    //         self.index((mincol, row))
    //     } else {
    //         self.index((newcol, row))
    //     }
    // }

    // // Returns the index shifted right a col.
    // // Does NOT update internal index.
    // pub fn index_right(&self, index: usize, n: i16) -> usize {
    //     if n < 0 { return self.index_left(index, n.abs()) }
    //     let (col, row) = self.coord(index);
    //     let maxcol = self.width - 1;
    //     let newcol = col + n;
    //     if newcol >= maxcol {
    //         self.index((maxcol, row))
    //     } else {
    //         self.index((newcol, row))
    //     }
    // }

    // // Returns the index shifted up a row.
    // // Does NOT update internal index.
    // pub fn index_up(&self, index:usize, n: i16) -> usize {
    //     if n < 0 { return self.index_down(index, n.abs()) }
    //     let (col, row) = self.coord(index);
    //     let minrow = 0;
    //     let newrow = row - n;
    //     if newrow <= minrow {
    //         self.index((col, minrow))
    //     } else {
    //         self.index((col, newrow))
    //     }
    // }

    // // Returns the index shifted down a row.
    // // Does NOT update internal index.
    // pub fn index_down(&self, index: usize, n: i16) -> usize {
    //     if n < 0 { return self.index_up(index, n.abs()) }
    //     let (col, row) = self.coord(index);
    //     let maxrow = self.height - 1;
    //     let newrow = row + n;
    //     if newrow >= maxrow {
    //         self.index((col, maxrow))
    //     } else {
    //         self.index((col, newrow))
    //     }
    // }

    // Returns a valid index after bounds checking. Always
    // provides an index at the beginning of a Cell (no Linkers).
    pub fn cursor(&mut self) -> usize {
        match self.cells.get(self.index) {
            Some(Cell::Link) => self.index -= 1,
            Some(_) => (),
            None => {
                // Could be out-of-bounds. Fix len/cap issues.
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
                    // Scenario C: no issues with buffer:
                    Ordering::Equal => (),
                }
                // Ensure index is valid after fixing buffer len/cap issues.
                if self.index >= capacity { self.index = capacity - 1 }
                // Should always be a valid index after the above:
                if let Cell::Link = self.cells[self.index] { self.index -= 1 }
            }
        }

        self.index
    }

    pub fn goto_index(&mut self, index: usize) -> usize {
        self.index = index;
        self.cursor()
    }

    pub fn goto_coord(&mut self, col: i16, row: i16) -> usize {
        self.index = self.index((col, row));
        self.cursor()
    }

    pub fn goto_mark(&mut self) {
        let index = self.cursor();
        self.goto_index(self.savedpos);
        self.savedpos = index;
    }

    // TODO: WINDOWS vs ANSI.
    fn patch(&mut self, cell: Cell, data: (usize, usize)) -> bool {
        // TODO: WINDOWS
        // if self.use_winapi { return false }
        let (index, cutoff) = data;
        let mut reset_cutoff = false;
        let that = &self.cells[index];
        // Handles only different cells.
        if &cell != that {
            // Handle output contents.
            // Check consecutive unchanged Cells.
            if cutoff > 8 {
                // 1. Truncate the output strbuf.
                let len = self.strbuf.len();
                let mut tail = String::with_capacity(16);
                match &cell {
                    // 1b. Adjust cutoff to strip only contents of Multi.
                    Cell::Multi(v, w, ..) => {
                        let mut cutoff = 0;
                        if w >= &2 {
                            let pred = |m: &char| *m == '\u{200d}';
                            let slice = v.split(pred).next();
                            for c in slice.unwrap_or(&[]) {
                                tail.push(*c);
                                cutoff += c.len_utf8();
                                if !self.can_modify { break }
                            }
                        } else { for c in v {
                            tail.push(*c);
                            cutoff += c.len_utf8();
                        }}
                        // Ensure truncation will not go less than 0.
                        if len >= cutoff { self.strbuf.truncate(len - cutoff) }
                    },
                    _ => {
                        // Ensure truncation will not go less than 0.
                        if len >= cutoff { self.strbuf.truncate(len - cutoff) }
                        // 2. Send a Goto escape sequence.
                        let (col, row) = self.coord(index);
                        let goto = format!("\x1B[{};{}H", row, col);
                        self.strbuf.push_str(&goto);
                    }
                }
                // 3. Restore the last char that was truncated.
                match &cell {
                    Cell::Single(ch, ..) => self.strbuf.push(*ch),
                    Cell::Double(ch, ..) => self.strbuf.push(*ch),
                    Cell::Multi(..) => self.strbuf.push_str(&tail),
                    Cell::NIL => self.strbuf.push(' '),
                    Cell::Link => (),
                }
            }
            // Reset the cutoff anytime we change the index or
            // Cells are different.
            reset_cutoff = true;
            // Handle internal cell buffer.
            // Replace Linkers for NIL.
            match *that {
                Cell::NIL => (),
                Cell::Single(..) => (),
                Cell::Link => self.cells[index - 1] = Cell::NIL,
                _ => self.cells[index + 1] = Cell::NIL,
            }
            // Swap current index with new Cell.
            self.cells[index] = cell
        }

        reset_cutoff
    }

    pub fn parse(&mut self, s: &str) {
        // https://eng.getwisdom.io/emoji-modifiers-and-sequence-combinations/
        let mods = [
            // skin tone (light, med-l, med, med-d, dark)
            '\u{1f3fb}', '\u{1f3fc}', '\u{1f3fd}', '\u{1f3fe}', '\u{1f3ff}',
            // gender (male, female)
            '\u{2640}', '\u{2642}',
            // hair (red, curly, bald, white)
            '\u{1f9b0}', '\u{1f9b1}', '\u{1f9b2}', '\u{1f9b3}'
        ];
        let mut index: usize = self.cursor();
        // Keep track of the memory size of consecutive unchanged Cells
        // to truncate or cutoff once a changed Cell or the end of the
        // iteration is reached. The threadhold is `8` or the size of
        // a "goto" ansi sequence (`"\x1B[00;00H"`).
        let mut cutoff: usize = 0;
        // Index of the last diff or strbuf trunctation.
        let mut freeze: usize = 0;
        let mut graphemes = UnicodeGraphemes::graphemes(s, true).peekable();
        while let Some(g) = graphemes.next()  {
            let mut chars = g.chars().peekable();
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
                            cutoff += size_of_val(g);
                            self.strbuf.push(car);
                            let data = (index, cutoff);
                            let reset = self.patch(
                                if car == ' ' { Cell::NIL }
                                else { Cell::Single(car, self.style) }, data);
                            index += 1;
                            if reset { cutoff = 0; freeze = index }
                        },
                        2 => {
                            let pk = graphemes.peek().unwrap_or(&"0");
                            let ch = pk.chars().next().unwrap();
                            if mods.contains(&ch) {
                                // Modified Double-width Cell
                                let (mut w, mut v) = (2, vec![car]);
                                let next = graphemes.next().unwrap();
                                chars = next.chars().peekable();
                                loop { match chars.next() {
                                    Some(next) => {
                                        v.push(next);
                                        w += next.width().unwrap_or(0);
                                    },
                                    None => match v.last().unwrap() {
                                        '\u{200d}' => match graphemes.next() {
                                            Some(g) => {
                                                chars = g.chars().peekable();
                                                continue;
                                            },
                                            _ => break
                                        }
                                        _ => break
                                    }
                                }}
                                if w >= 2 {
                                    let pred = |m: &char| *m == '\u{200d}';
                                    let slice = v.split(pred).next();
                                    for c in slice.unwrap_or(&[]) {
                                        self.strbuf.push(*c);
                                        cutoff += c.len_utf8();
                                        if !self.can_modify { break }
                                    }
                                } else { for c in &v {
                                    self.strbuf.push(*c);
                                    cutoff += c.len_utf8();
                                }}
                                let data = (index, cutoff);
                                let reset = self.patch(Cell::Multi(
                                    v, w, self.style), data);
                                self.patch(Cell::Link, (index + 1, 0));
                                index += 2;
                                if reset { cutoff = 0; freeze = index }
                            } else {
                                // Standard Double-width Cell
                                cutoff += size_of_val(g);
                                self.strbuf.push(car);
                                let data = (index, cutoff);
                                let reset = self.patch(
                                    Cell::Double(car, self.style), data);
                                self.patch(Cell::Link, (index + 1, 0));
                                index += 2;
                                if reset { cutoff = 0; freeze = index }
                            }
                        },
                        _ => continue,
                    },
                    // Escape character
                    None => match car {
                        // These update the cursor.
                        // They do not overwrite or make content changes.
                        // TODO: instead of hardcoding the CSI sequences
                        // use ansi functions.
                        '\t' => {
                            let tabbed = self.tabstop(self.coord(index));
                            let offset = tabbed - index;
                            if offset > 0 {
                                self.strbuf.push_str(
                                    &format!("\x1B[{}C", offset));
                                // Updates to the strbuf resets the cutoff.
                                cutoff = 0;
                                index = tabbed;
                                freeze = index;
                            }
                        },
                        '\n' => {
                            // TODO: Toggle between raw mode and cooked
                            // mode treatments...for ansi
                            // TODO: Windows simply needs to update index
                            #[cfg(windows)] { if self.use_winapi {
                                let row = index as i16 / self.width;
                                // NOTE: `\n` is equal to `\r\n` this treats
                                // them consistently...
                                index = if self.height > row + 1 {
                                    self.index((0, row + 1))
                                } else {
                                    self.index((0, self.height - 1))
                                };
                            } else {
                                let (col, row) = self.coord(index);
                                self.strbuf.push_str(&String::from("\x1B[B"));
                                // Updates to the strbuf resets the cutoff.
                                cutoff = 0;
                                // index = self.index_down(index, 1);
                                index = if self.height > row + 1 {
                                    self.index((col, row + 1))
                                } else {
                                    self.index((col, self.height - 1))
                                };
                                freeze = index;
                            };}

                            let (col, row) = self.coord(index);
                            self.strbuf.push_str(&String::from("\x1B[B"));
                            // Updates to the strbuf resets the cutoff.
                            cutoff = 0;
                            // index = self.index_down(index, 1);
                            index = if self.height > row + 1 {
                                self.index((col, row + 1))
                            } else {
                                self.index((col, self.height - 1))
                            };
                            freeze = index;
                        },
                        '\r' => {
                            let (col, row) = self.coord(index);
                            self.strbuf.push_str(&format!("\x1B[{}D", col));
                            // Updates to the strbuf resets the cutoff.
                            cutoff = 0;
                            index = self.index((0, row));
                            freeze = index;
                        },
                        '\x1B' => {
                            cutoff += size_of_val(g);
                            self.strbuf.push('^');
                            let data = (index, cutoff);
                            let reset = self.patch(
                                Cell::Single('^', self.style), data);
                            index += 1;
                            if reset { cutoff = 0; freeze = index }
                        },
                        _ => continue,
                    }
                },
                Some(cadr) => match (car, cadr) {
                    ('\r', '\n') => {
                        let row = index as i16 / self.width;
                        // Updates to the strbuf resets the cutoff.
                        cutoff = 0;
                        if self.height > row + 1 {
                            self.strbuf.push_str(
                                &format!("\x1B[0;{}H", row + 1));
                            index = self.index((0, row + 1));
                        } else {
                            self.strbuf.push_str(
                                &format!("\x1B[0;{}H", self.height - 1));
                            index = self.index((0, self.height - 1));
                        }
                        freeze = index;
                    },
                    _ => {
                        let mut w = car.width().unwrap_or(0);
                        let mut v = vec![car];
                        // Gather all characters into content.
                        loop { match chars.next() {
                            Some(next) => {
                                v.push(next);
                                w += next.width().unwrap_or(0);
                            },
                            None => match v.last().unwrap() {
                                '\u{200d}' => match graphemes.next() {
                                    Some(g) => {
                                        chars = g.chars().peekable();
                                        continue;
                                    },
                                    None => break
                                },
                                _ => break
                            }
                        }}
                        if w >= 2 {
                            let pred = |m: &char| *m == '\u{200d}';
                            let slice = v.split(pred).next();
                            for c in slice.unwrap_or(&[]) {
                                self.strbuf.push(*c);
                                cutoff += c.len_utf8();
                                if !self.can_modify { break }
                            }
                        } else { for c in &v {
                            self.strbuf.push(*c);
                            cutoff += c.len_utf8();
                        }}
                        let data = (index, cutoff);
                        let reset = self.patch(Cell::Multi(
                            v, w, self.style), data);
                        self.patch(Cell::Link, (index + 1, 0));
                        index += 2;
                        if reset { cutoff = 0; freeze = index }
                    }
                }
            }}
        }
        // Truncate remaining cutoff, if any, from output string.
        let len = self.strbuf.len();
        if cutoff > 0 && len >= cutoff {
            self.strbuf.truncate(len - cutoff);
            index = freeze;
        }
        // Set index to the new index
        self.index = index;
        self.cursor();
    }

    pub fn getch(&mut self) -> String {
        let index = self.cursor();
        match &self.cells[index] {
            Cell::Single(ch, ..) => format!("{}", ch),
            Cell::Double(ch, ..) => format!("{}", ch),
            Cell::Multi(chs,..) => chs.iter().collect(),
            Cell::NIL => String::from(" "),
            Cell::Link => unreachable!()
        }
    }

    #[cfg(test)]
    fn flush(&mut self) -> String {
        let output = self.strbuf.to_string();
        self.strbuf.clear();
        output
    }
}


// fn pos_raw() -> (i16, i16) {
//     use std::io::{ Write, BufRead };
//     let ln = 603;
//     // Where is the cursor?
//     // Use `ESC [ 6 n`.
//     let mut stdout = std::io::stdout();
//     let stdin = std::io::stdin();

//     // Write command
//     stdout.write_all(b"\x1B[6n").expect(&format!(
//         "buffer.rs [Ln: {}]: Error writing to stdout", ln + 9));
//     stdout.flush().expect(&format!(
//         "buffer.rs [Ln: {}]: Error flushing stdout", ln + 11));

//     stdin.lock().read_until(b'[', &mut vec![]).expect(&format!(
//         "buffer.rs [Ln {}]: Error reading stdin", ln + 14));

//     let mut rows = vec![];
//     stdin.lock().read_until(b';', &mut rows).expect(&format!(
//         "buffer.rs [Ln {}]: Error reading stdin", ln + 18));

//     let mut cols = vec![];
//     stdin.lock().read_until(b'R', &mut cols).expect(&format!(
//         "buffer.rs [Ln {}]: Error reading stdin", ln + 22));

//     // remove delimiter
//     rows.pop();
//     cols.pop();

//     let rows = rows
//         .into_iter()
//         .map(|b| (b as char))
//         .fold(String::new(), |mut acc, n| {
//             acc.push(n);
//             acc
//         })
//         .parse::<usize>()
//         .expect(&format!(
//             "buffer.rs [Ln {}]: Error parsing row position.", ln + 29
//         ));
//     let cols = cols
//         .into_iter()
//         .map(|b| (b as char))
//         .fold(String::new(), |mut acc, n| {
//             acc.push(n);
//             acc
//         })
//         .parse::<usize>()
//         .expect(&format!(
//             "buffer.rs [Ln {}]: Error parsing col position.", ln + 40
//         ));

//     ((cols - 1) as i16, (rows - 1) as i16)
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tabstop() {
        let mut buf = Buffer::new();
        buf.goto_coord(15, 0);
        let tabbed = buf.tabstop((15, 0));
        assert_eq!(tabbed, 16);
        buf.goto_index(tabbed);
        assert_eq!(buf.cursor(), 16);
    }

    #[test]
    fn test_ascii_buffer_simple() {
        let mut buf = Buffer::new();
        // NOTE: this test is only for ANSI supported terminals.
        #[cfg(windows)] { if buf.winmode() { return } }

        let original_input = "Hello, world!";
        buf.parse(original_input);
        buf.flush();
        // TODO: probably want to write the output to stdout asap;
        // TODO: don't want the stfbuf getting wonky from multiple write ops.

        let modified_input = "Bella, whale!";
        buf.goto_coord(0, 0);
        buf.parse(modified_input);
        let output = buf.flush();
        assert_eq!(String::from("Bella, whale"), output);
        assert_eq!(buf.cursor(), 12);
        assert_eq!("!", &buf.getch());

        // Mimick goto:
        buf.goto_coord(5, 0);
        assert_eq!(",", &buf.getch());

        let consecutive_input = "Hella, wharf!";
        buf.goto_coord(0, 0);
        buf.parse(consecutive_input);
        let output = buf.flush();
        assert_eq!(String::from("H\x1B[0;10Hrf"), output);
    }

    #[test]
    fn test_ascii_buffer_simple_v2() {
        // This was used to catch the self.strbuf.truncate
        // issue when using .saturating_sub which didn't
        // truncate because len == cutoff.
        let mut buf = Buffer::new();
        let original_input = "-------- a";
        buf.parse(original_input);
        let _ = buf.flush();
        let modified_input = "-------- b";
        buf.goto_index(0);
        buf.parse(modified_input);
        let output = buf.flush();
        assert_eq!("\x1B[0;9Hb", output);
    }

    #[test]
    fn test_cjk_buffer() {
        let mut buf = Buffer::new();
        // NOTE: this test is only for ANSI supported terminals.
        #[cfg(windows)] { if buf.winmode() { return } }

        let original_input = "... Hello, 中易隶书, world!";
        buf.parse(original_input);
        buf.flush();
        buf.goto_coord(12, 0);
        assert_eq!("中", buf.getch());
        // 12 is not a valid index.
        assert_ne!(buf.cursor(), 12);
        assert_eq!(buf.cursor(), 11);
        let index = buf.cursor() as i16;
        // 13 is valid here for the next character `易`.
        // buf.goto_index(buf.index_right(index, 2));
        buf.goto_coord(index + 2, 0);
        assert_eq!(buf.cursor(), 13);
        assert_eq!("易", buf.getch());

        let tabbed_input = "\tx";
        buf.parse(tabbed_input);
        let output = buf.flush();
        assert_eq!("\x1B[3Cx", output);
        buf.goto_coord(15, 0);
        assert_eq!(" ", &buf.getch());
        buf.goto_coord(16, 0);
        assert_eq!("x", &buf.getch());

        let modified_input = "o,  中易隶书, wrld!";
        buf.goto_coord(8, 0);
        buf.parse(modified_input);
        let output = buf.flush();
        assert_eq!(String::from("o,  中易隶书, w"), output);
        assert_eq!(buf.cursor(), 23);
        assert_eq!("r", &buf.getch());

        // The character should not be at (11, 0)
        buf.goto_coord(11, 0);
        assert_ne!("中", &buf.getch());
        let index = buf.cursor() as i16;
        // Indeed 11 is a valid "NIL" character.
        assert_eq!(index, 11);
        assert_eq!(" ", &buf.getch());

        // The "中" character has moved one to the right.
        buf.goto_coord(index + 1, 0);
        assert_eq!("中", &buf.getch());
        // 12 is a valid index now.
        assert_eq!(buf.cursor(), 12);
        // 13 no longer is for `易`.
        buf.goto_coord(13, 0);
        assert_eq!("中", &buf.getch());
        assert_eq!(buf.cursor(), 12);
        // // The character `易` is now at index 14.
        buf.goto_coord(14, 0);
        assert_eq!("易", buf.getch());
    }

    #[test]
    fn test_ascii_buffer_complex() {
        let mut buf = Buffer::new();
        // NOTE: this test is only for ANSI supported terminals.
        #[cfg(windows)] { if buf.winmode() { return } }

        let original_input = "Hello, world!\n\nH23\towdy, neighbor!";
        buf.parse(original_input);
        buf.flush();
        // Mimick goto:
        buf.goto_coord(12, 0);
        assert_eq!("!", &buf.getch());
        buf.goto_coord(13, 2);
        assert_eq!("H", &buf.getch());
        buf.goto_coord(14, 2);
        assert_eq!("2", &buf.getch());
        buf.goto_coord(15, 2);
        assert_eq!("3", &buf.getch());
        buf.goto_coord(20, 2);
        assert_eq!("o", &buf.getch());

        buf.goto_coord(13, 2);
        let modified_input = "B23\tella, nutrition!";
        buf.parse(modified_input);
        let output = buf.flush();
        // NOTE: Below the cursor is at 16 so the next
        // tabstop would be 20 or 4 cells away.
        assert_eq!(String::from("B23\x1B[4Cella, nutrition!"), output);

        buf.goto_coord(26, 2);
        assert_eq!("n", &buf.getch());

        buf.goto_coord(13, 2);
        let repeated_input = "B23\tella, natrition!";
        buf.parse(repeated_input);
        let output = buf.flush();
        // NOTE: Even though the only changed letter is `a` from `nutrition`
        // the parser is smart enough to trim the excess unchanged letters.
        assert_ne!(String::from("B23\x1B[4Cella, natrition!"), output);
        assert_eq!(String::from("B23\x1B[4Cella, na"), output);
    }

    #[test]
    fn test_modifier_false_support() {
        let mut buf = Buffer::new();
        // // NOTE: this test is only for ANSI supported terminals.
        // #[cfg(windows)] { if buf.winmode() { return } }
        let original_input = "-------- 👨🏿‍🦰 ----";
        buf.parse(original_input);
        let output = buf.flush();
        assert_ne!(original_input, output);
        assert_eq!("-------- 👨 ----", output);
        buf.goto_index(0);
        let adjusted_input = "-------- 👨🏿‍🦰 ****";
        buf.parse(adjusted_input);
        let output = buf.flush();
        assert_eq!("\x1B[0;12H****", output);
    }

    #[test]
    fn test_modifier_true_support() {
        let mut buf = Buffer::new();
        buf.can_modify = true;
        let original_input = "---- 👨🏿‍🦰 ----";
        buf.parse(original_input);
        let output = buf.flush();
        let modified_input = "---- 👨🏿 ----";
        assert_eq!(modified_input, output);
        buf.goto_index(0);
        let adjusted_input = "**** 👨🏿‍🦰 ****";
        buf.parse(adjusted_input);
        let output = buf.flush();
        buf.goto_index(5);
        let chat5 = buf.getch();
        buf.goto_index(6);
        let chat6 = buf.getch();
        // NOTE: If can_modify = true, that means that
        // "👨" and "🏿" combine into a single 2-cell
        // char. In this case, the ch at index 5 and 6
        // are equivalent...
        assert_eq!(chat5, chat6);
        buf.goto_index(7);
        // ...and what is at index 7 is the space before
        // the strbuf change ('-' -> '*')
        assert_eq!(" ", buf.getch());
        assert_eq!("****\x1B[0;8H****", output);
    }

    #[test]
    fn test_emoji_support() {
        let mut buf = Buffer::new();
        let original_input = "-------- 👨‍👩‍👦 ----";
        buf.parse(original_input);
        let output = buf.flush();
        assert_eq!("-------- 👨 ----", output);
        let adjusted_input = "-------- 👨‍👩‍👦 ****";
        buf.goto_index(0);
        buf.parse(adjusted_input);
        let output = buf.flush();
        assert_eq!("\x1B[0;12H****", output);
        buf.clear(Clear::All);
        buf.can_modify = true;
        let modified_input = "-------- 👨🏽‍👩🏽‍👧🏽 ----";
        buf.parse(modified_input);
        let output = buf.flush();
        assert_eq!("-------- 👨🏽 ----", output);
        let adjusted_input = "-------- 👨🏽‍👩🏽‍👧🏽 ****";
        buf.goto_index(0);
        buf.parse(adjusted_input);
        let output = buf.flush();
        assert_eq!("\x1B[0;12H****", output);
    }
}
