use std::io::{ Result, Error, ErrorKind };
use libc::termios as Termios;
use crate::actions::ansi::*;
use crate::common::enums::{ Clear, Style, Color };


pub struct Term { mode: Termios }

impl Term {
    pub fn new() -> Result<Self> {
        Ok(Self { mode: output::get_mode()? })
    }

    // CURSOR FUNCTIONS
    pub fn goto(&self, col: i16, row: i16) -> Result<()> {
        let (mut col, mut row) = (col, row);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }
        output::prints(&cursor::goto(col, row))
    }

    pub fn up(&self, n: i16) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_up(n))
    }

    pub fn down(&self, n: i16) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_down(n))
    }

    pub fn left(&self, n: i16) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_left(n))
    }

    pub fn right(&self, n: i16) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_right(n))
    }

    pub fn query_pos(&self) -> Result<()> {
        output::printf(&cursor::pos())
    }

    pub fn raw_pos(&self) -> Result<(i16, i16)> {
        // Where is the cursor?
        // Use `ESC [ 6 n`.
        let mut stdout = std::io::stdout();
        let stdin = std::io::stdin();

        // Write command
        std::io::Write::write_all(&mut stdout, b"\x1B[6n")?;
        std::io::Write::flush(&mut stdout)?;
        std::io::BufRead::read_until(&mut stdin.lock(), b'[', &mut vec![])?;
        let mut rows = vec![];
        std::io::BufRead::read_until(&mut stdin.lock(), b';', &mut rows)?;
        let mut cols = vec![];
        std::io::BufRead::read_until(&mut stdin.lock(),  b'R', &mut cols)?;
        // Remove delimiter
        rows.pop(); cols.pop();

        let parsed_rows = match rows
            .into_iter()
            .map(|b| (b as char))
            .fold(String::new(), |mut acc, n| {
                acc.push(n); acc
            }).parse::<i16>() {
                Ok(i) => i,
                Err(e) => {
                    let err_msg = format!(
                        "Error parsing into i16. {:?}", e);
                    return Err(Error::new(ErrorKind::Other, err_msg));
                }
            };

        let parsed_cols = match cols
            .into_iter()
            .map(|b| (b as char))
            .fold(String::new(), |mut acc, n| {
                acc.push(n); acc
            }).parse::<i16>() {
                Ok(i) => i,
                Err(e) => {
                    let err_msg = format!(
                        "Error parsing into i16. {:?}", e);
                    return Err(Error::new(ErrorKind::Other, err_msg));
                }
            };

        Ok((parsed_cols.saturating_sub(1) , parsed_rows.saturating_sub(1)))
    }

    pub fn hide_cursor(&self) -> Result<()> {
        output::prints(&cursor::hide_cursor())
    }

    pub fn show_cursor(&self) -> Result<()> {
        output::prints(&cursor::show_cursor())
    }

    // SCREEN FUNCTIONS
    pub fn clear(&self, method: Clear) -> Result<()> {
        output::prints(&screen::clear(method))
    }

    pub fn size(&self) -> (i16, i16) {
        screen::size()
    }

    pub fn resize(&self, w: i16, h: i16) -> Result<()> {
        output::printf(&screen::resize(w, h))
    }

    pub fn enable_alt(&self) -> Result<()> {
        output::printf(&screen::enable_alt())
    }

    pub fn disable_alt(&self) -> Result<()> {
        output::printf(&screen::disable_alt())
    }

    // OUTPUT FUNCTIONS
    pub fn prints(&self, content: &str) -> Result<()> {
        output::prints(content)
    }

    pub fn printf(&self, content: &str) -> Result<()> {
        output::printf(content)
    }

    pub fn flush(&self) -> Result<()> {
        output::flush()
    }

    pub fn raw(&self) -> Result<()> {
        output::enable_raw()
    }

    pub fn cook(&self) -> Result<()> {
        output::set_mode(&self.mode)
    }

    // MOUSE FUNCTIONS
    pub fn enable_mouse(&self) -> Result<()> {
        output::prints(&mouse::enable_mouse_mode())
    }

    pub fn disable_mouse(&self) -> Result<()> {
        output::prints(&mouse::disable_mouse_mode())
    }

    // STYLE FUNCTIONS
    pub fn set_fx(&self, effects: u32) -> Result<()> {
        output::prints(&style::set_style(Style::Fx(effects)))
    }

    pub fn set_fg(&self, color: Color) -> Result<()> {
        output::prints(&style::set_style(Style::Fg(color)))
    }

    pub fn set_bg(&self, color: Color) -> Result<()> {
        output::prints(&style::set_style(Style::Bg(color)))
    }

    pub fn set_styles(&self, fg: Color, bg: Color, fx: u32) -> Result<()> {
        output::prints(&style::set_styles(fg, bg, fx))
    }

    pub fn reset_styles(&self) -> Result<()> {
        output::prints(&style::reset())
    }

    // CONFIG FUNCTIONS
    // pub fn get_mode() -> Result<Termios> {
    //     output::get_mode()
    // }

    pub fn init_data(&self) -> Termios {
        self.mode
    }
}
