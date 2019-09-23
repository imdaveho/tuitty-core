// ANSI specific screen cache buffer implmentation.

use std::{
    iter, mem,
    collections::VecDeque,
};

use super::output;
use super::style;
use crate::common::{
    cache::CacheUpdater,
    enums::{ Clear, Color, Effect, Style },
    unicode::{grapheme::*, wcwidth::*},
};


#[derive(Clone, Debug)]
enum Rune {
    Single(char),
    Double(char),
    Compound(Vec<char>),
    Null,
}


#[derive(Clone, Debug)]
pub struct CellInfo {
    rune: Rune,
    width: usize,
    style: (Color, Color, u32),
}


#[derive(Clone)]
pub struct CellInfoCache {
    tab_width: u8,
    style: (Color, Color, u32),
    screen_pos: (i16, i16),
    screen_size: (i16, i16),
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
        let (w, h) = (
            self.screen_size.0 as usize,
            self.screen_size.1 as usize);
        let (col, row) = (
            self.screen_pos.0 as usize,
            self.screen_pos.1 as usize);
        // (imdaveho) NOTE: Remember that buffer indices are 0-based, 
        // which means that index[0] is actually cell[1] (col: 0, row: 0).
        let capacity = w * h;
        let mut index = (row * w) + col;
        let segments: Vec<&str> = UnicodeGraphemes
            ::graphemes(content, true).collect();
        for ch in segments {
            // Check capacity and truncate to keep at capacity.
            if self.buffer.get(index).is_none() {
                self.buffer.rotate_left(w);
                index = capacity - w;
                for i in index..capacity {
                    self.buffer[i] = None;
                }
            }
            // Ascii segments are single width characters.
            if ch.is_ascii() {
                let mut c = ch.chars().next().unwrap_or('\x00');
                if c == '\x00' { continue }
                if c == '\x1B' { c = '^' }
                if c == '\r' { index = (index / w) * w; continue }
                self.buffer[index] = Some(CellInfo {
                    rune: Rune::Single(c),
                    width: 1,
                    style: self.style,
                });
                index += 1;
                continue
            }
            // Unicode: CJK, Emoji, and other.
            let chbuf = ch.chars().first_last();
            for (is_first, is_last, c) in chbuf {
                let w = UnicodeWidthChar::width(c).unwrap_or(1);
                // This can only be a single Unicode character.
                // Or a single character continuation from a joiner.
                if is_first && is_last {
                    if let Some(info) = &mut self.buffer[index] {
                        // Continuation of previous sequence.
                        if let Rune::Compound(chseq) = &mut info.rune {
                            chseq.push(c);
                            // Since char is first and last, wrap it up.
                            index += 1;
                            self.buffer[index] = Some(CellInfo {
                                rune: Rune::Null,
                                width: 0,
                                style: self.style,
                            });
                            index += 1;
                        }
                    } else {
                        if w == 1 {
                            self.buffer[index] = Some(CellInfo {
                                rune: Rune::Single(c),
                                width: 1,
                                style: self.style,
                            });
                            index += 1;
                        }
                        if w == 2 {
                            self.buffer[index] = Some(CellInfo {
                                rune: Rune::Double(c),
                                width: 2,
                                style: self.style,
                            });
                            index += 1;
                            self.buffer[index] = Some(CellInfo {
                                rune: Rune::Null,
                                width: 0,
                                style: self.style,
                            });
                            index += 1;
                        }
                    }
                } else if is_first && !is_last {
                    // This will have to be a compound Unicode character.
                    // Start of the compound character.
                    if let Some(info) = &mut self.buffer[index] {
                        if let Rune::Compound(chseq) = &mut info.rune {
                            // Existing compound sequence that ended in a ZWJ.
                            chseq.push(c);
                        } else {
                            // Overwriting.
                            let mut chseq = Vec::with_capacity(8);
                            chseq.push(c);
                            self.buffer[index] = Some(CellInfo {
                                rune: Rune::Compound(chseq),
                                width: w,
                                style: self.style,
                            });
                        }
                    } else {
                        // Starting new.
                        let mut chseq = Vec::with_capacity(8);
                        chseq.push(c);
                        self.buffer[index] = Some(CellInfo {
                            rune: Rune::Compound(chseq),
                            width: w,
                            style: self.style,
                        });
                    }
                } else if !is_first && !is_last {
                    // Middle of the compound character.
                    if let Some(info) = &mut self.buffer[index] {
                        if let Rune::Compound(chseq) = &mut info.rune {
                            chseq.push(c);
                        }
                    }
                } else {
                    // End of the compound character.
                    // !is_first && is_last
                    if let Some(info) = &mut self.buffer[index] {
                        if let Rune::Compound(chseq) = &mut info.rune {
                            chseq.push(c);
                        }
                    }
                    if c != '\u{200d}' {
                        // The next segment is not connected to this one.
                        index += 1;
                        self.buffer[index] = Some(CellInfo {
                            rune: Rune::Null,
                            width: 0,
                            style: self.style,
                        });
                        index += 1;
                    }
                    // If the last character was a joining character, we
                    // keep the index as is, and insert upon the next loop.
                }
            }
        }
        // Calculate where the new cursor position should be.
        self.screen_pos = ((index % w) as i16, (index / w) as i16);
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
        let (w, h) = (
            self.screen_size.0 as usize,
            self.screen_size.1 as usize);
        let capacity = w * h;
        let default = (Color::Reset, Color::Reset, Effect::Reset as u32);
        // TODO: stress test the content.len capacity here.
        let mut contents = String::with_capacity(capacity * 3);
        let mut previous = (Color::Reset, Color::Reset, Effect::Reset as u32);
        // Reset everything from the previous screens once at the start.
        contents.push_str(&style::reset());

        for cell in &self.buffer {
            match cell {
                Some(cel) => {
                    // Restore styles.
                    if cel.style != previous && cel.style == default {
                        // Reset not just when the current style differs a bit
                        // from the previous, but every field is different and
                        // is a {Color|Effect}::Reset value.
                        contents.push_str(&style::reset())
                    } else {
                        // Else, go through each and update them.
                        if cel.style.0 != previous.0 {
                            contents.push_str(
                                &style::set_style(Style::Fg(cel.style.0)))
                        }

                        if cel.style.1 != previous.1 {
                            contents.push_str(
                                &style::set_style(Style::Bg(cel.style.1)))
                        }

                        if cel.style.2 != previous.2 {
                            contents.push_str(
                                &style::set_style(Style::Fx(cel.style.2)))
                        }
                    }
                    previous = cel.style;
                    // Insert contents.
                    match &cel.rune {
                        Rune::Single(c) => match c {
                            '\t' => for _ in 0..self.tab_width {
                                contents.push(' ')
                            },
                            // '\n' => (),
                            _ => contents.push(*c),
                        },
                        Rune::Double(c) => contents.push(*c),
                        Rune::Null => (),
                        Rune::Compound(v) => {
                            for c in v {
                                contents.push(*c)
                            }
                        }
                    }
                },
                None => {
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
        //     // (imdaveho) NOTE: stackoverflow.com/questions/
        //     // 23975391/how-to-convert-a-string-into-a-static-str
        //     let cellspace = UnicodeWidthStr::width(&*contents);
        //     match cell {
        //         Some(cl) => {
        //             // if (cellspace + cl.width) > capacity { break }
        //             if cl.style != previous && cl.style == default {
        //                 // Reset not just when the current style differs a bit
        //                 // from the previous, but every field is different and
        //                 // is a {Color|Effect}::Reset value.
        //                 contents.push_str(&style::reset())
        //             } else {
        //                 // Else, go through each and update them.
        //                 if cl.style.0 != previous.0 {
        //                     contents.push_str(
        //                         &style::set_style(Style::Fg(cl.style.0)))
        //                 }

        //                 if cl.style.1 != previous.1 {
        //                     contents.push_str(
        //                         &style::set_style(Style::Bg(cl.style.1)))
        //                 }

        //                 if cl.style.2 != previous.2 {
        //                     contents.push_str(
        //                         &style::set_style(Style::Fx(cl.style.2)))
        //                 }
        //             }
        //             contents.push(cl.rune);
        //             previous = cl.style;
        //         }

        //         None => {
        //             if (cellspace + 1) > capacity { break }
        //             if previous == default { contents.push(' '); }
        //             else {
        //                 contents.push_str(&style::reset());
        //                 contents.push(' ');
        //                 previous = default;
        //             }
        //         }
        //     }
        // }
        // output::printf(&contents);
    }
}


trait FirstLastIterator: Iterator + Sized {
    fn first_last(self) -> FirstLast<Self>;
}

impl<I> FirstLastIterator for I where I: Iterator {
    fn first_last(self) -> FirstLast<Self> {
        FirstLast(true, self.peekable())
    }
}

pub struct FirstLast<I>(bool, iter::Peekable<I>) where I: Iterator;

impl<I> Iterator for FirstLast<I> where I: Iterator {
    type Item = (bool, bool, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let first = mem::replace(&mut self.0, false);
        self.1.next().map(|item| (first, self.1.peek().is_none(), item))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cache_content_ansii() {
        use super::CellInfoCache;

        let ascii = "AA";
        let mut cache = CellInfoCache::new();
        cache._cache_content(ascii);
        let mut cache_copy = Vec::with_capacity(5);
        for i in 0..5 {
            if let Some(info) = &cache.buffer[i] {
                match info.rune {
                    super::Rune::Single(c) => cache_copy.push(c.to_string()),
                    super::Rune::Null => cache_copy.push(String::from("_")),
                    _ => cache_copy.push(String::from("Other")),
                }
            } else {
                cache_copy.push(String::from("None"));
            }
        }
        println!("{:?}", cache_copy);
        std::thread::sleep(std::time::Duration::from_millis(10000));
    }

    #[test]
    fn test_cache_content_cjk() {
        use super::CellInfoCache;

        let cjk = "色A";
        let mut cache = CellInfoCache::new();
        cache._cache_content(cjk);
        let mut cache_copy = Vec::with_capacity(5);
        for i in 0..5 {
            if let Some(info) = &cache.buffer[i] {
                match info.rune {
                    super::Rune::Single(c) => cache_copy.push(c.to_string()),
                    super::Rune::Double(c) => cache_copy.push(c.to_string()),
                    super::Rune::Null => cache_copy.push(String::from("_")),
                    _ => cache_copy.push(String::from("Other")),
                }
            } else {
                cache_copy.push(String::from("None"));
            }
        }
        println!("{:?}", cache_copy);
    }

    #[test]
    fn test_cache_content_compound() {
        use super::CellInfoCache;

        // let compound = "👨‍👩‍👧A👨‍🚀A🤦‍♀️A";
        let compound = "👨‍🚀";
        let mut cache = CellInfoCache::new();
        cache._cache_content(compound);
        let mut cache_copy = Vec::with_capacity(5);
        for i in 0..15 {
            if let Some(info) = &cache.buffer[i] {
                match &info.rune {
                    super::Rune::Single(c) => cache_copy.push(c.to_string()),
                    super::Rune::Double(c) => cache_copy.push(c.to_string()),
                    super::Rune::Compound(v) => {
                        println!("Compound!");
                        for c in v {
                            cache_copy.push(c.to_string())
                        }
                    },
                    super::Rune::Null => cache_copy.push(String::from("_")),
                }
            } else {
                cache_copy.push(String::from("None"));
            }
        }
        println!("{:?}", cache_copy);
    }
}