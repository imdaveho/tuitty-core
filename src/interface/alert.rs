//! TODO:

use crate::terminal::CommonTerminal;
use crate::common::{
    traits::*,
    enums::{ InputEvent, KeyEvent, MouseEvent, MouseButton, Color, Style },
};

#[cfg(unix)]
use crate::terminal::unix::{ AsyncReader, size };

#[cfg(windows)]
use crate::terminal::windows::AsyncReader;

#[cfg(windows)]
use crate::terminal::wincon::screen::size;


pub struct AlertBox {
    screen_size: (i16, i16),
    width: i16, height: i16, voffset: i16,
    message: String,
    dims: (i16, i16, i16, i16),
}

impl AlertBox {
    pub fn new() -> AlertBox {
        let (sw, sh) = size();
        let (mw, mh) = (sw / 2, sh / 2);
        let (wide, high, vo) = (52, 8, 2);
        let (top, bot) = (mh - high / 2 - vo, mh + high / 2 - vo);
        let (left, right) = (mw - wide / 2, mw + wide / 2);
        AlertBox {
            screen_size: (sw, sh),
            width: wide, height: high, voffset: vo,
            message: "".to_string(),
            dims: (top, bot, left, right),
        }
    }

    pub fn render(&mut self) {
        let tty = CommonTerminal::new();
        self.draw_box(&tty);
        self.draw_body(&tty);
        tty.flush();
    }

    pub fn handle(&mut self, input: &mut AsyncReader) -> bool {
        let midw = self.screen_size.0 / 2;
        let yrange = (midw - 8, midw - 3);
        let nrange = (midw + 2, midw + 7);
        let btnrow = self.dims.1 - 2;

        loop {
            if let Some(evt) = input.next() {
                match evt {
                    InputEvent::Keyboard(k) => match k {
                        KeyEvent::Char(c) => match c {
                            'Y' | 'y' => return true,
                            'N' | 'n' => return false,
                            _ => (),
                        },
                        // (imdaveho) TODO: Handle the below (issue #2)
                        // KeyEvent::Esc => return false,
                        _ => ()
                    },
                    InputEvent::Mouse(m) => match m {
                        MouseEvent::Press(b, col, row) => match b {
                            MouseButton::Left => {
                                if row == btnrow {
                                    if col <= yrange.1 && col >= yrange.0 {
                                        return true;
                                    }
                                    if col <= nrange.1 && col >= nrange.0 {
                                        return false;
                                    }
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    },
                    _ => ()
                }
            }
	          std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }

    fn draw_box(&mut self, tty: &CommonTerminal) {
        let (top, bot, left, right) = self.dims;

        // Draw corners.
        tty.goto(left, top);
        tty.prints("┌");
        tty.goto(right, top);
        tty.prints("┐");
        tty.goto(left, bot);
        tty.prints("└");
        tty.goto(right, bot);
        tty.prints("┘");

        // Draw top and bot horiz lines.
        let h_line = "─".repeat((self.width - 1) as usize);
        tty.goto(left + 1, top);
        tty.prints(&h_line);
        tty.goto(left + 1, bot);
        tty.prints(&h_line);

        // Draw left and right vert lines.
        for row in (top + 1)..bot {
            let v_line = "│";
            tty.goto(left, row);
            tty.prints(v_line);
            tty.goto(right, row);
            tty.prints(v_line);
        }

        // Draw title bar.
        tty.goto(left + 1, top + 1);
        let title = "[!] Alert";
        tty.prints("[");
        tty.set_style(Style::Fg(Color::Yellow));
        tty.prints("!");
        tty.set_style(Style::Fg(Color::Reset));
        let nbspc = self.width as usize - title.len() - 1;
        tty.prints(&format!("] Alert{:>1$}", " ", nbspc));
        tty.reset_styles();

        // Draw bottom title bar.
        tty.goto(left + 1, top + 2);
        tty.prints(&h_line);
        tty.goto(left, top + 2);
        tty.prints("├");
        tty.goto(right, top + 2);
        tty.prints("┤");
    }


    pub fn set_content(&mut self, msg: &str) {
        self.message = msg.to_string();
        // (imdaveho) TODO: handle \n and \t
        let box_width = (self.width - 4) as usize;
        let msg_width = self.message.len();
        if msg_width > box_width {
            let countln = msg_width / box_width;
            self.set_height(self.height + countln as i16);
        }
    }

    fn draw_body(&mut self, tty: &CommonTerminal) {
        // Render message.
        let (top, bot, left, _) = self.dims;
        let box_width = (self.width - 4) as usize;
        let msg_width = self.message.len();
        if msg_width > box_width {
            let countln = msg_width / box_width;
            let mut start = 0;
            for i in 0..countln {
                let finish = start + box_width;
                let row = top + 4 + i as i16;
                if row == bot - 3 {
                    break
                }
                tty.goto(left + 2, row);
                tty.prints(&self.message[start..=finish]);
                start = finish + 1;
            }
        } else {
            tty.goto(left + 2, top + 4);
            tty.prints(&self.message);
        }

        // Draw buttons.
        let midw = self.screen_size.0 / 2;
        let bot = self.dims.1;
        tty.goto(midw - 8, bot - 2);
        tty.set_style(Style::Bg(Color::DarkBlue));
        tty.set_style(Style::Fg(Color::Green));
        tty.prints("[ Y ]");
        tty.goto(midw + 2, bot - 2);
        tty.set_style(Style::Bg(Color::DarkMagenta));
        tty.set_style(Style::Fg(Color::Red));
        tty.prints("[ N ]");
        tty.reset_styles();
    }

    pub fn set_width(&mut self, w: i16) {
        let mut w = w;
        if w < 0 { w = w.abs(); }
        let sw = self.screen_size.0;
        let mw = sw / 2;
        let (left, right) = (mw - w / 2, mw + w / 2);
        self.width = w;
        self.dims.2 = left;
        self.dims.3 = right;
    }

    pub fn set_height(&mut self, h: i16) {
        let mut h = h;
        if h < 0 { h = h.abs(); }
        let sh = self.screen_size.1;
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
        let sh = self.screen_size.1;
        let mh = sh / 2;
        let h = self.height;
        let (top, bot) = (mh - h / 2 - vo, mh + h / 2 - vo);
        self.voffset = vo;
        self.dims.0 = top;
        self.dims.1 = bot;
    }

    pub fn set_screensize(&mut self) {
        self.screen_size = size();
    }
}
