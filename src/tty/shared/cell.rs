//! TODO: 

use super::{
    ansi_write,
    Clear, Color, Effect, Style::*,
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
    fx: u32,
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
    pub fn new() -> CellBuffer {
        #[cfg(unix)]
        let (w, h) = super::super::screen::ansi::size();        
        #[cfg(windows)]
        let (w, h) = super::super::screen::wincon::size();
        
        let capacity = (w * h) as usize;
        CellBuffer {
            screen_pos: (0, 0),
            screen_size: (w, h),
            style: CellStyle::new(),
            cells: vec![None; capacity],
        }
    }

    pub fn _screen_size(&self) -> (i16, i16) {
        self.screen_size
    }

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

    pub fn _restore_buffer(&self) {
        let (w, h) = self.screen_size;
        let capacity = (w * h) as isize;
        // TODO: stress test the content.len capacity here.
        let mut contents = String::with_capacity((capacity * 2) as usize);
        let mut previous = CellStyle::new();
        // Reset everything from the previous screens once at the start.
        contents.push_str(&output::ansi::reset());
        for cell in &self.cells {
            // (imdaveho) NOTE: stackoverflow.com/questions/
            // 23975391/how-to-convert-a-string-into-a-static-str
            let cellspace = UnicodeWidthStr::width(&*contents) as isize;
            match cell {
                Some(c) => {
                    if capacity - (cellspace + c.width) < 0 { break }
                    // let (fg, bg, fx) = (c.style.fg, c.style.bg, c.style.fx);

                    // if c.style != previous && c.style == CellStyle::new() {
                    //     // Reset not just when the current style differs a bit
                    //     // from the previous, but every field is different and
                    //     // is a {Color|Effect}::Reset value.
                    //     contents.push_str(&output::ansi::reset())
                    // } else {
                    //     // If not, well go through each and update them.
                    //     if fg != previous.fg {
                    //         contents.push_str(&output::ansi::set_style(Fg(fg)))
                    //     }

                    //     if bg != previous.bg {
                    //         contents.push_str(&output::ansi::set_style(Bg(bg)))
                    //     }

                    //     if fx != previous.fx {
                    //         contents.push_str(&output::ansi::set_style(Fx(fx)))
                    //     }
                    // }
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

    pub fn _sync_buffer(&mut self, content: &str) {
        let length = UnicodeWidthStr::width(content);
        let charbuf = content.chars();
        let (w, h) = self.screen_size;
        let (col, row) = self.screen_pos;
        let here = ((row * w) + col) as usize;
        let there = here + length;
        let (ncol, nrow) = (there % w as usize, (there / w as usize));

        // (imdaveho) NOTE: Remember that buffer indices are 0-based, which
        // means that index 0 (col: 0, row: 0) is actually capacity: 1.
        let capacity = (w * h) as usize;
        // If length == capacity, the cursor will overflow by 1, so subtract it.
        // TODO: Truncate the first n rows, and print the overflow n rows. Needs
        // to handle control characters in loop...
        // let capacity = meta.buffer_size();
        if length > capacity - 1 { return };

        let mut iteration = 0;
        for ch in charbuf {
            match UnicodeWidthChar::width(ch) {
                Some(width) => {
                    // (imdaveho) NOTE: The only control character that returns
                    // Some() is the null byte. If for some reason, there is a
                    // null byte passed within the &str parameter, we should
                    // simple ignore it and not update the backbuf.
                    if ch == '\x00' { continue } ;

                    self.cells[here + iteration] = Some(Cell {
                        rune: ch,
                        width: width as isize,
                        style: self.style.clone(),
                    });
                    iteration += 1;
                }
                None => {
                    // (imdaveho) note: this is an escape sequence or a `char`
                    // with ambiguous length defaulting to `::width()` == 1 or
                    // `::width_cjk()` == 2.

                    // (imdaveho) todo: this would only happen if the
                    // user is trying to manually write an escape sequence.
                    // attempt to interpret what the escape sequence is, and
                    // update meta.cell_style with the details of the sequence.
                    // difficulty: medium/hard -
                    // * create a byte vector that fills with an ansi esc seq
                    // * when you hit a printable char, take the byte vector,
                    //   and map it to a cell style (medium) or specific
                    //   ansii function (hard).
                    ()
                }
            }
        }
        self.screen_pos = (ncol as i16, nrow as i16);
    }
}


