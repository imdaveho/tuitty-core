//! TODO: 

use super::{
    ansi_write,
    Clear, Color, Effect, Effects, Style::*,
    Style, UnicodeWidthStr, UnicodeWidthChar,
};
use super::super::output;


#[derive(Clone)]
pub struct Cell {
    rune: char,
    style: CellStyle,
    width: isize,
}


#[derive(Clone, Copy, PartialEq)]
pub struct CellStyle {
    fg: Color,
    bg: Color,
    fx: Effects,
}

impl CellStyle {
    pub fn new() -> CellStyle {
        CellStyle {
            fg: Color::Reset,
            bg: Color::Reset,
            fx: Effect::Reset as u32
        }
    }
}


#[derive(Clone)]
pub struct CellBuffer {
    screen_pos: (i16, i16),
    screen_size: (i16, i16),
    style: CellStyle,
    cells: Vec<Option<Cell>>,
}

impl CellBuffer {
    pub fn new(w: i16, h: i16) -> CellBuffer {
        let capacity = (w * h) as usize;
        CellBuffer {
            screen_pos: (0, 0),
            screen_size: (w, h),
            style: CellStyle::new(),
            cells: vec![None; capacity],
        }
    }

    // pub fn _width(&self) -> i16 {
    //     self.size.0
    // }

    // pub fn _height(&self) -> i16 {
    //     self.size.1
    // }

    pub fn _screen_size(&self) -> (i16, i16) {
        self.screen_size
    }

    // pub fn _col(&self) -> i16 {
    //     self.pos.0
    // }

    // pub fn _row(&self) -> i16 {
    //     self.pos.1
    // }

    pub fn _screen_pos(&self) -> (i16, i16) {
        self.screen_pos
    }

    pub fn _clear(&mut self, method: Clear) {
        match method {
            Clear::All => {
                let (w, h) = self.screen_size;
                let capacity = (w * h) as usize;
                self.cells = vec![None; capacity];
            }
            Clear::NewLn => {
                let (w, (col, row)) = (self.screen_size.0, self.screen_pos);
                let (here, there) = ((row * w) + col, (row + 1) * w);
                for i in (here as usize)..(there as usize) {
                    self.cells[i] = None;
                }
            }
            Clear::CurrentLn => {
                let (w, row) = (self.screen_size.0, self.screen_pos.1);
                let (here, there) = ((row * w), (row + 1) * w);
                for i in (here as usize)..(there as usize) {
                    self.cells[i] = None;
                }
            }
            Clear::CursorUp => {
                let (w, (col, row)) = (self.screen_size.0, self.screen_pos);
                let here = (row * w) + col;
                for i in 0..(here as usize) {
                    self.cells[i] = None;
                }
            }
            Clear::CursorDn => {
                let ((w, h), (col, row)) = (self.screen_size, self.screen_pos);
                let (here, there) = ((row * w) + col, w * h);
                for i in (here as usize)..(there as usize) {
                    self.cells[i] = None;
                }
            }
        }
    }

    pub fn _resize(&mut self, w: i16, h: i16) {
        self.screen_size = (w, h);
        self.cells.resize((w * h) as usize, None);
        // TODO: re-calc cursor position
    }

    pub fn _restore(&self) {
        let (w, h) = self.screen_size;
        let capacity = (w * h) as isize;
        let mut contents = String::with_capacity((capacity * 2) as usize);
        let mut previous = CellStyle::new();

        for cell in &self.cells {
            // (imdaveho) NOTE: stackoverflow.com/questions/
            // 23975391/how-to-convert-a-string-into-a-static-str
            let cellspace = UnicodeWidthStr::width(&*contents) as isize;
            match cell {
                Some(c) => {
                    if capacity - (cellspace + c.width) < 0 { break }
                    let (fg, bg, fx) = (c.style.fg, c.style.bg, c.style.fx);

                    if c.style != previous && c.style == CellStyle::new() {
                        // Style resets everything and isn't following a prior
                        // reset style.
                        contents.push_str(&output::ansi::reset())
                    } else {
                        // If not, well go through each and update them.
                        if fg != previous.fg {
                            contents.push_str(&output::ansi::set_style(Fg(fg)))
                        }

                        if bg != previous.bg {
                            contents.push_str(&output::ansi::set_style(Bg(bg)))
                        }

                        if fx != previous.fx {
                            contents.push_str(&output::ansi::set_style(Fx(fx)))
                        }
                    }
                    contents.push(c.rune);
                    previous = c.style;
                }

                None => {
                    if capacity - (cellspace + 1) < 0 { break }
                    if previous == CellStyle::new() { contents.push(' '); }
                    else {
                        contents.push_str(&output::ansi::reset());
                        contents.push(' ');
                        previous = CellStyle::new();
                    }
                }
            }
        }
        ansi_write(&contents, true);
    }

    pub fn _reposition(&mut self, col: i16, row: i16) {
        self.screen_pos = (col, row)
    }

    pub fn _sync_up(&mut self, n: i16) {
        if n < 0 { return }
        let current_row = self.screen_pos.1;
        if current_row - n > 0 {
            self.screen_pos.1 -= n
        } else { self.screen_pos.1 = 0 }
    }

    pub fn _sync_dn(&mut self, n: i16) {
        if n < 0 { return }
        let h = self.screen_size.1;
        let current_row = self.screen_pos.1;
        if current_row + n < h {
            self.screen_pos.1 += n
        } else { self.screen_pos.1 = h }
    }

    pub fn _sync_left(&mut self, n: i16) {
        if n < 0 { return }
        let current_col = self.screen_pos.0;
        if current_col - n > 0 {
            self.screen_pos.0 -= n
        } else {
            self.screen_pos.0 = 0
            // (imdaveho) NOTE: Cursor wrapping draft.
            // let w = self.screen_size.0;
            // let rows = n / w;
            // let rest = n % w;
            // self._sync_up(rows);
            // if current_col - rest > 0 {
            //     self.screen_pos.0 -= rest
            // } else {
            //     self.screen_pos.0 = 0
            // }
        }
    }

    pub fn _sync_right(&mut self, n: i16) {
        if n < 0 { return }
        let w = self.screen_size.0;
        let current_col = self.screen_pos.0;
        if current_col + n < w {
            self.screen_pos.0 += n
        } else {
            self.screen_pos.0 = w;
            // (imdaveho) NOTE: Cursor wrapping draft.
            // let rows = n / w;
            // let rest = n % w;
            // self._sync_dn(rows);
            // if current_col + rest < w {
            //     self.screen_pos.0 += rest
            // } else {
            //     self.screen_pos.0 = w
            // }
        }
    }

    pub fn _sync_style(&mut self, style: Style) {
        match style {
            Fg(c) => self.style.fg = c,
            Bg(c) => self.style.bg = c,
            Fx(f) => self.style.fx = f,
        }
    }

    pub fn _reset_style(&mut self) {
        self.style = CellStyle {
            fg: Color::Reset,
            bg: Color::Reset,
            fx: Effect::Reset as u32,
        }
    }
}


