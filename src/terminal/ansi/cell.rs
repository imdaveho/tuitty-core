// ANSI specific screen cache buffer implmentation.

use std::collections::VecDeque;

use super::output;
use super::style;
use crate::common::{
    cache::CacheUpdater,
    enums::{ Clear, Color, Effect, Style },
    wcwidth::{ UnicodeWidthStr, UnicodeWidthChar },
};


#[derive(Clone)]
pub struct CellInfo {
    rune: char,
    style: (Color, Color, u32),
    width: usize,
}

#[derive(Clone)]
pub struct CellInfoCache {
    tab_width: u8,
    screen_pos: (i16, i16),
    screen_size: (i16, i16),
    style: (Color, Color, u32),
    #[cfg(not(test))]
    buffer: VecDeque<Option<CellInfo>>,
    #[cfg(test)]
    pub buffer: VecDeque<Option<CellInfo>>,
}

impl CellInfoCache {
    pub fn new() -> CellInfoCache {
        let (w, h) = {
            #[cfg(unix)] { crate::terminal::unix::size() }
            #[cfg(windows)] { crate::terminal::wincon::screen::size() }
        };
        let capacity = (w * h) as usize;
        CellInfoCache {
            tab_width: 4,
            screen_pos: (0, 0),
            screen_size: (w, h),
            style: (Color::Reset, Color::Reset, Effect::Reset as u32),
            buffer: vec![None; capacity].into(),
        }
    }

    pub fn _clear_buffer(&mut self, method: Clear) {
        match method {
            Clear::All => {
                let (w, h) = self.screen_size;
                let capacity = (w * h) as usize;
                self.buffer = vec![None; capacity].into();
            }
            Clear::NewLn => {
                let (w, (col, row)) = (self.screen_size.0, self.screen_pos);
                let (here, there) = ((row * w) + col, (row + 1) * w);
                for i in (here as usize)..(there as usize) {
                    self.buffer[i] = None;
                }
            }
            Clear::CurrentLn => {
                let (w, row) = (self.screen_size.0, self.screen_pos.1);
                let (here, there) = ((row * w), (row + 1) * w);
                for i in (here as usize)..(there as usize) {
                    self.buffer[i] = None;
                }
            }
            Clear::CursorUp => {
                let (w, (col, row)) = (self.screen_size.0, self.screen_pos);
                let here = (row * w) + col;
                for i in 0..(here as usize) {
                    self.buffer[i] = None;
                }
            }
            Clear::CursorDn => {
                let ((w, h), (col, row)) = (self.screen_size, self.screen_pos);
                let (here, there) = ((row * w) + col, w * h);
                for i in (here as usize)..(there as usize) {
                    self.buffer[i] = None;
                }
            }
        }
    }

    pub fn _cache_content(&mut self, content: &str) {
        let charbuf = content.chars();
        let (w, h) = (
            self.screen_size.0 as usize,
            self.screen_size.1 as usize);
        let (col, row) = (
            self.screen_pos.0 as usize,
            self.screen_pos.1 as usize);
        // Keep this buffer at most as large as the amount of cells present on screen
        // (imdaveho) NOTE: Remember that buffer indices are 0-based, which
        // means that index 0 (col: 0, row: 0) is actually capacity: 1.
        let capacity = w * h;
        let mut index = (row * w) + col;
        let mut length = 0;
        for ch in charbuf {
            // Check capacity.
            match self.buffer.get(index) {
                Some(_) => {
                    // In-bounds
                    // Match on `ch`.
                    match UnicodeWidthChar::width(ch) {
                        Some(width) => {
                            if ch == '\x00' { continue }
                            self.buffer[index] = Some(
                                CellInfo {
                                    rune: ch,
                                    width: width,
                                    style: self.style,
                                });
                            index += 1; length += width;
                        }
                        None => { 
                            match ch {
                                '\x1B' => {
                                    self.buffer[index] = Some(
                                        CellInfo {
                                            rune: '^',
                                            width: 1,
                                            style: self.style,
                                        });
                                    index += 1; length += 1;
                                }
                                '\n' => {
                                    let spaces = w - (index % w);
                                    for _ in 0..spaces {
                                        match self.buffer.get(index) {
                                            Some(_) => {
                                                self.buffer[index] = Some(
                                                    CellInfo {
                                                        rune: ' ',
                                                        width: 1,
                                                        style: self.style,
                                                    });
                                                length += 1;
                                            }
                                            None => {
                                                self.buffer.rotate_left(w);
                                                index = capacity - w;
                                                for i in index..capacity {
                                                    self.buffer[i] = None;
                                                }
                                                self.buffer[index] = Some(
                                                    CellInfo {
                                                        rune: ' ',
                                                        width: 1,
                                                        style: self.style,
                                                    });
                                                length += 1;
                                            }
                                        }
                                        index += 1;
                                    }
                                }
                                '\t' => {
                                    for _ in 0..self.tab_width as usize {
                                        match self.buffer.get(index) {
                                            Some(_) => {
                                                self.buffer[index] = Some(
                                                    CellInfo {
                                                        rune: ' ',
                                                        width: 1,
                                                        style: self.style,
                                                    });
                                                length += 1;
                                            }
                                            None => {
                                                self.buffer.rotate_left(w);
                                                index = capacity - w;
                                                for i in index..capacity {
                                                    self.buffer[i] = None;
                                                }
                                                self.buffer[index] = Some(
                                                    CellInfo {
                                                        rune: ' ',
                                                        width: 1,
                                                        style: self.style,
                                                    });
                                                length += 1;
                                            }
                                        }
                                        index += 1;
                                    }
                                }
                                '\r' => index = (index / w) * w,
                                _ => continue,
                            }
                        }
                    }
                }
                None => {
                    // Out-of-bounds
                    // Rotate the VecDeque buffer. (Truncate).
                    self.buffer.rotate_left(w);
                    index = capacity - w;
                    for i in index..capacity {
                        self.buffer[i] = None;
                    }
                    // Match on `ch`. (Identical to Some(_) case).
                    match UnicodeWidthChar::width(ch) {
                        Some(width) => {
                            if ch == '\x00' { continue }
                            self.buffer[index] = Some(
                                CellInfo {
                                    rune: ch,
                                    width: width,
                                    style: self.style,
                                });
                            index += 1; length += width;
                        }
                        None => { 
                            match ch {
                                '\x1B' => {
                                    self.buffer[index] = Some(
                                        CellInfo {
                                            rune: '^',
                                            width: 1,
                                            style: self.style,
                                        });
                                    index += 1; length += 1;
                                }
                                '\n' => {
                                    let spaces = w - (index % w);
                                    for _ in 0..spaces {
                                        match self.buffer.get(index) {
                                            Some(_) => {
                                                self.buffer[index] = Some(
                                                    CellInfo {
                                                        rune: ' ',
                                                        width: 1,
                                                        style: self.style,
                                                    });
                                                length += 1;
                                            }
                                            None => {
                                                self.buffer.rotate_left(w);
                                                index = capacity - w;
                                                for i in index..capacity {
                                                    self.buffer[i] = None;
                                                }
                                                self.buffer[index] = Some(
                                                    CellInfo {
                                                        rune: ' ',
                                                        width: 1,
                                                        style: self.style,
                                                    });
                                                length += 1;
                                            }
                                        }
                                        index += 1;
                                    }
                                }
                                '\t' => {
                                    for _ in 0..self.tab_width as usize {
                                        match self.buffer.get(index) {
                                            Some(_) => {
                                                self.buffer[index] = Some(
                                                    CellInfo {
                                                        rune: ' ',
                                                        width: 1,
                                                        style: self.style,
                                                    });
                                                length += 1;
                                            }
                                            None => {
                                                self.buffer.rotate_left(w);
                                                index = capacity - w;
                                                for i in index..capacity {
                                                    self.buffer[i] = None;
                                                }
                                                self.buffer[index] = Some(
                                                    CellInfo {
                                                        rune: ' ',
                                                        width: 1,
                                                        style: self.style,
                                                    });
                                                length += 1;
                                            }
                                        }
                                        index += 1;
                                    }
                                }
                                '\r' => index = (index / w) * w,
                                _ => continue,
                            }
                        }
                    }
                }
            }
        }
        let there = ((row * w) + col) + length;
        let (new_col, new_row) = (there % w, there / w);
        if new_row > h - 1 {
            self.screen_pos = ((new_col as i16), (h - 1) as i16)
        } else {
            self.screen_pos = (new_col as i16, new_row as i16);
        }
    }
}

impl CacheUpdater for CellInfoCache {
    fn _tab_width(&mut self, w: u8) {
        self.tab_width = w;
    }

    fn _screen_size(&self) -> (i16, i16) {
        self.screen_size
    }

    fn _screen_pos(&self) -> (i16, i16) {
        self.screen_pos
    }

    fn _sync_size(&mut self, w: i16, h: i16) {
        self.screen_size = (w, h);
        self.buffer.resize((w * h) as usize, None);
        // TODO: re-calc cursor position
    }

    fn _sync_pos(&mut self, col: i16, row: i16) {
        self.screen_pos = (col, row)
    }

    fn _sync_up(&mut self, n: i16) {
        if n < 0 { return }
        let current_row = self.screen_pos.1;
        if current_row - n > 0 {
            self.screen_pos.1 -= n
        } else { self.screen_pos.1 = 0 }
    }

    fn _sync_down(&mut self, n: i16) {
        if n < 0 { return }
        let h = self.screen_size.1;
        let current_row = self.screen_pos.1;
        if current_row + n < h {
            self.screen_pos.1 += n
        } else { self.screen_pos.1 = h }
    }

    fn _sync_left(&mut self, n: i16) {
        if n < 0 { return }
        let current_col = self.screen_pos.0;
        if current_col - n > 0 {
            self.screen_pos.0 -= n
        } else {
            // self.screen_pos.0 = 0
            // (imdaveho) NOTE: Cursor wrapping draft.
            // TODO: n > capacity handling
            let w = self.screen_size.0;
            let rows = n / w;
            let rest = n % w;
            self._sync_up(rows);
            if current_col - rest > 0 {
                self.screen_pos.0 -= rest
            } else {
                self.screen_pos.0 = 0
            }
        }
    }

    fn _sync_right(&mut self, n: i16) {
        if n < 0 { return }
        let w = self.screen_size.0;
        let current_col = self.screen_pos.0;
        if current_col + n < w {
            self.screen_pos.0 += n
        } else {
            // self.screen_pos.0 = w;
            // (imdaveho) NOTE: Cursor wrapping draft.
            // TODO: n > capacity handling
            let rows = n / w;
            let rest = n % w;
            self._sync_down(rows);
            if current_col + rest < w {
                self.screen_pos.0 += rest
            } else {
                self.screen_pos.0 = w
            }
        }
    }

    fn _sync_style(&mut self, style: Style) {
        match style {
            Style::Fg(c) => self.style.0 = c,
            Style::Bg(c) => self.style.1 = c,
            Style::Fx(f) => self.style.2 = f,
        }
    }

    fn _sync_styles(&mut self, fg: Color, bg: Color, fx: u32) {
        self.style = (fg, bg, fx)
    }

    fn _reset_styles(&mut self) {
        self.style = (Color::Reset, Color::Reset, Effect::Reset as u32);
    }

    fn _flush_buffer(&self) {
        let (w, h) = self.screen_size;
        let capacity = (w * h) as usize;
        // TODO: stress test the content.len capacity here.
        let mut contents = String::with_capacity((capacity * 2) as usize );
        let default = (Color::Reset, Color::Reset, Effect::Reset as u32);
        let mut previous = (Color::Reset, Color::Reset, Effect::Reset as u32);
        // Reset everything from the previous screens once at the start.
        contents.push_str(&style::reset());
        for cell in &self.buffer {
            // (imdaveho) NOTE: stackoverflow.com/questions/
            // 23975391/how-to-convert-a-string-into-a-static-str
            let cellspace = UnicodeWidthStr::width(&*contents);
            match cell {
                Some(cl) => {
                    // if (cellspace + cl.width) > capacity { break }
                    if cl.style != previous && cl.style == default {
                        // Reset not just when the current style differs a bit
                        // from the previous, but every field is different and
                        // is a {Color|Effect}::Reset value.
                        contents.push_str(&style::reset())
                    } else {
                        // Else, go through each and update them.
                        if cl.style.0 != previous.0 {
                            contents.push_str(
                                &style::set_style(Style::Fg(cl.style.0)))
                        }

                        if cl.style.1 != previous.1 {
                            contents.push_str(
                                &style::set_style(Style::Bg(cl.style.1)))
                        }

                        if cl.style.2 != previous.2 {
                            contents.push_str(
                                &style::set_style(Style::Fx(cl.style.2)))
                        }
                    }
                    contents.push(cl.rune);
                    previous = cl.style;
                }

                None => {
                    if (cellspace + 1) > capacity { break }
                    if previous == default { contents.push(' '); }
                    else {
                        contents.push_str(&style::reset());
                        contents.push(' ');
                        previous = default;
                    }
                }
            }
        }
        output::printf(&contents);
    }
}
