use std::mem::zeroed;
use winapi::um::wincon::{
    WriteConsoleOutputW, ReadConsoleOutputW,
    COORD, CHAR_INFO, SMALL_RECT,
};
use winapi::shared::ntdef::{NULL, VOID};
use super::style;
use super::handle::Handle;
use crate::common::{
    cache::CacheHandler,
    enums::{ Clear, Color, Effect, Style::* },
};


#[derive(Clone)]
pub struct CharInfoCache {
    screen_pos: (i16, i16),
    screen_size: (i16, i16),
    style: (Color, Color, u32),
    buffer: Vec<CHAR_INFO>,
}

impl CacheHandler for CharInfoCache {
    pub fn new() -> CharInfoCache {
        let (w, h) = super::super::screen::wincon::size();
        let capacity = (w * h) as usize;
        CharInfoCache {
            screen_pos: (0, 0),
            screen_size: (w, h),
            style: (Color::Reset, Color::Reset, Effect::Reset),
            buffer: unsafe {vec![zeroed(); capacity]},
        }
    }

    pub fn _screen_size(&self) -> (i16, i16) {
        self.screen_size
    }

    pub fn _screen_pos(&self) -> (i16, i16) {
        self.screen_pos
    }

    pub fn _sync_size(&mut self, w: i16, h: i16) {
        self.screen_size = (w, h);
        self.buffer.resize((w * h) as usize, zeroed());
        // TODO: re-calc cursor position
    }
    
    pub fn _sync_pos(&mut self, col: i16, row: i16) {
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
            // self.screen_pos.0 = 0
            // (imdaveho) NOTE: Cursor wrapping draft.
            // TODO: confirm behavior on Windows.
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

    pub fn _sync_right(&mut self, n: i16) {
        if n < 0 { return }
        let w = self.screen_size.0;
        let current_col = self.screen_pos.0;
        if current_col + n < w {
            self.screen_pos.0 += n
        } else {
            // self.screen_pos.0 = w;
            // (imdaveho) NOTE: Cursor wrapping draft.
            // TODO: confirm behavior on Windows.
            let rows = n / w;
            let rest = n % w;
            self._sync_dn(rows);
            if current_col + rest < w {
                self.screen_pos.0 += rest
            } else {
                self.screen_pos.0 = w
            }
        }
    }

    pub fn _sync_style(&mut self, style: Style) {
        match style {
            Fg(c) => self.style.0 = c,
            Bg(c) => self.style.1 = c,
            Fx(f) => self.style.2 = f,
        }
    }

    pub fn _sync_styles(&mut self, fg: Color, bg: Color, fx: u32) {
        self.style = (fg, bg, fx)
    }

    pub fn _reset_styles(&mut self) {
        self.style = (Color::Reset, Color::Reset, Effect::Reset);
    }

    pub fn _flush(&self) {
        let (w, h) = self.screen_size;
        let conout = Handle::conout().unwrap();
        let dimens = COORD {X: w, Y: h};
        let origin = COORD {X; 0, Y: 0};
        let mut dest_rect = SMALL_RECT {
            Top: 0,
            Left: 0,
            Bottom: h,
            Right: w,
        };
        unsafe {
            // https://docs.microsoft.com/en-us/windows/console/writeconsole
            if WriteConsoleOutputW(
                conout.0,
                self.buffer.as_ptr(),
                dimens, origin,
                &mut dest_rect, NULL
            ) == 0 {
                panic!("Error restoring screen from buffer cache")
            }
        }
    }

    pub fn _cache(&self) {
        let (w, h) = self.screen_size;
        let conout = Handle::conout().unwrap();
        let dimens = COORD {X: w, Y: h};
        let origin = COORD {X; 0, Y: 0};
        let mut dest_rect = SMALL_RECT {
            Top: 0,
            Left: 0,
            Bottom: h,
            Right: w,
        };
        unsafe {
            // https://docs.microsoft.com/en-us/windows/console/writeconsole
            if ReadConsoleOutputW(
                conout.0,
                self.buffer.as_mut_ptr(),
                dimens, origin,
                &mut dest_rect, NULL
            ) == 0 {
                panic!("Error caching screen into buffer cache")
            }
        }
    }
}