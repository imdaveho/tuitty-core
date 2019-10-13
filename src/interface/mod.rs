//! TODO: placeholder for specific components to build a TUI.


use crate::terminal::Terminal;
use crate::common::{
    enums::{ InputEvent, KeyEvent, MouseEvent, MouseButton, Color },
    traits::{ 
        TerminalFormatter, TerminalInput, TerminalCursor, TerminalWriter
    },
};


pub struct AlertBox<'t> {
    width: i16,
    height: i16,
    voffset: i16,
    message: String,
    termref: &'t mut Terminal,
    glyphs: (
        char, char, 
        char, char, char, char,
        char, char
    ),
    dims: (i16, i16, i16, i16),
}

impl<'t> AlertBox<'t> {
    pub fn new(t: &'t mut Terminal, msg: String) -> AlertBox {
        let (sw, sh) = t.screen_size();
        let (mw, mh) = (sw / 2, sh / 2);
        let (w, h, vo) = (52, 8, 2);
        let (top, bot) = (mh - h / 2 - vo, mh + h / 2 - vo);
        let (left, right) = (mw - w / 2, mw + w / 2);
        AlertBox {
            width: w,
            height: h,
            voffset: vo,
            message: msg,
            termref: t,
            glyphs: (
                '─', '│',
                '┌', '┐', '└', '┘',
                '├',  '┤'
            ),
            dims: (top, bot, left, right),
        }
    }

    pub fn render(&mut self) {
        self.draw_content();
        self.draw_sides();
        self.draw_corners();
        self.draw_header();
        self.draw_buttons();
        self.termref.flush();
    }

    pub fn handle(&mut self) -> bool {
        let mut input = self.termref.read_async();

        let midpt = self.termref.screen_size().0 / 2;
        let ystart_col = midpt - 8;
        let yfinish_col = ystart_col + 5;
        let xstart_col = midpt + 2;
        let xfinish_col = xstart_col + 5;
        let button_row = self.dims.1 - 2;

        loop {
            if let Some(evt) = input.next() {
                match evt {
                    InputEvent::Keyboard(k) => match k {
                        KeyEvent::Char(c) => match c {
                            'Y' | 'y' => return true,
                            'N' | 'n' => return false,
                            _ => (),
                        },
                        KeyEvent::Esc => return false,
                        _ => ()
                    },
                    InputEvent::Mouse(m) => {
                        self.termref.goto(0, button_row + 1);
                        self.termref.printf("Mouse!");
                        match m {
                        MouseEvent::Press(b, col, row) => match b {
                            MouseButton::Left => {
                                // self.termref.goto(0, button_row + 1);
                                // self.termref.printf(&format!("x:{}, y:{}", col, row));
                                if row == button_row {
                                    if col <= yfinish_col && col >= ystart_col {
                                        return true;
                                    }
                                    if col <= xfinish_col && col >= xstart_col {
                                        return false;
                                    }
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    }},
                    _ => ()
                }
            }
        }
    }

    fn draw_sides(&mut self) {
        let (top, bot, left, right) = self.dims;
        let hstr = self.glyphs.0.to_string().repeat((right - left) as usize);
        self.termref.goto(left, top);
        self.termref.prints(&hstr);
        self.termref.goto(left, bot);
        self.termref.prints(&hstr);
        for row in top..bot {
            let vstr = &self.glyphs.1.to_string();
            self.termref.goto(left, row);
            self.termref.prints(vstr);
            self.termref.goto(right, row);
            self.termref.prints(vstr);
        } 
    }

    fn draw_corners(&mut self) {
        let (top, bot, left, right) = self.dims;
        self.termref.goto(left, top);
        self.termref.prints(&self.glyphs.2.to_string());
        self.termref.goto(right, top);
        self.termref.prints(&self.glyphs.3.to_string());
        self.termref.goto(left, bot);
        self.termref.prints(&self.glyphs.4.to_string());
        self.termref.goto(right, bot);
        self.termref.prints(&self.glyphs.5.to_string());
    }

    fn draw_header(&mut self) {
        let (top, _, left, right) = self.dims;
        self.termref.goto(left + 1, top + 1);
        self.termref.prints("[");
        self.termref.set_fg(Color::Yellow);
        self.termref.prints("!");
        self.termref.set_fg(Color::Reset);
        let spaces = " ".repeat((right - left - 11) as usize);
        let remainder = format!("] Notice{}", spaces);
        self.termref.prints(&remainder);
        self.termref.reset_styles();

        let botline = self.glyphs.0.to_string().repeat((right - left - 1) as usize);
        self.termref.goto(left + 1, top + 2);
        self.termref.prints(&botline);
        self.termref.goto(left, top + 2);
        self.termref.prints(&self.glyphs.6.to_string());
        self.termref.goto(right, top + 2);
        self.termref.prints(&self.glyphs.7.to_string());
    }

    fn draw_content(&mut self) {
        let (top, _, left, right) = self.dims;
        let boxwidth = (right - left - 4) as usize;
        let msglength = self.message.len();
        if msglength > boxwidth {
            let linecount = msglength / boxwidth;
            self.set_height(self.height + linecount as i16);
            let (top, bot, left, right) = self.dims;
            let boxwidth = (right - left - 4) as usize;
            let mut start = 0;
            for i in 0..linecount {
                let finish = start + boxwidth;
                let row = top + 4 + i as i16;
                if row == bot - 3 {
                    break
                }
                self.termref.goto(left + 2, row);
                self.termref.prints(&self.message[start..=finish]);
                start = finish + 1;
            }
        } else {
            self.termref.goto(left + 2, top + 4);
            self.termref.prints(&self.message);
        }
    }

    fn draw_buttons(&mut self) {
        let midpt = self.termref.screen_size().0 / 2;
        self.termref.goto(midpt - 8, self.dims.1 - 2);
        self.termref.set_bg(Color::DarkCyan);
        self.termref.set_fg(Color::Green);
        self.termref.prints("[ Y ]");
        self.termref.goto(midpt + 2, self.dims.1 - 2);
        self.termref.set_bg(Color::Yellow);
        self.termref.set_fg(Color::Red);
        self.termref.prints("[ N ]");
        self.termref.reset_styles();
    }

    pub fn set_width(&mut self, w: i16) {
        let mut w = w;
        if w < 0 { w = w.abs(); }
        let sw = self.termref.screen_size().0;
        let mw = sw / 2;
        let (left, right) = (mw - w / 2, mw + w / 2);
        self.width = w;
        self.dims.2 = left;
        self.dims.3 = right;
    }

    pub fn set_height(&mut self, h: i16) {
        let mut h = h;
        if h < 0 { h = h.abs(); }
        let sh = self.termref.screen_size().1;
        let mh = sh / 2;
        let vo = self.voffset;
        let (top, bot) = (mh - h / 2 - vo, mh + h / 2 - vo);
        self.height = h;
        self.dims.0 = top;
        self.dims.1 = bot;
    }

    pub fn set_offset(&mut self, vo: i16) {
        let mut vo = vo;
        if vo < 0 { vo = vo.abs(); }
        let sh = self.termref.screen_size().1;
        let mh = sh / 2;
        let h = self.height;
        let (top, bot) = (mh - h / 2 - vo, mh + h / 2 - vo);
        self.voffset = vo;
        self.dims.0 = top;
        self.dims.1 = bot;
    }
}