use std::io::Result;
use libc::termios as Termios;
use crate::actions::ansi::*;
use crate::common::enums::{ Clear, Style, Color };


pub struct Term { mode: Termios }

impl Term {
    pub fn new() -> Result<Self> {
        Ok(Self { mode: output::get_mode()? })
    }

    // CURSOR FUNCTIONS
    pub fn goto(col: i16, row: i16) {
        let (mut col, mut row) = (col, row);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }
        output::prints(&cursor::goto(col, row));
    }

    pub fn up(n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_up(n));
    }

    pub fn down(n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_down(n));
    }

    pub fn left(n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_left(n));
    }

    pub fn right(n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_right(n));
    }

    pub fn query_pos() {
        output::printf(&cursor::pos());
    }

    pub fn pos_raw() -> Result<(i16, i16)> {
        // Where is the cursor?
        // Use `ESC [ 6 n`.
        let mut stdout = std::io::stdout();
        let stdin = std::io::stdin();

        // Write command
        std::io::Write::write_all(&mut stdout, b"\x1B[6n")?;
            // .expect("Error writing cursor report");
        std::io::Write::flush(&mut stdout)?;
            // .expect("Error flushing cursor report");
        std::io::BufRead::read_until(&mut stdin.lock(), b'[', &mut vec![])?;
            // .expect("Error reading cursor report");
        let mut rows = vec![];
        std::io::BufRead::read_until(&mut stdin.lock(), b';', &mut rows)?;
            // .expect("Error reading cursor row");
        let mut cols = vec![];
        std::io::BufRead::read_until(&mut stdin.lock(),  b'R', &mut cols)?;
            // .expect("Error reading cursor col");
        // Remove delimiter
        rows.pop(); cols.pop();

        let parsed_rows: i16 = rows
            .into_iter()
            .map(|b| (b as char))
            .fold(String::new(), |mut acc, n| {
                acc.push(n);
                acc
            })
            .parse()?;
            // .expect("Error parsing row position");
        let parsed_cols: i16 = cols
            .into_iter()
            .map(|b| (b as char))
            .fold(String::new(), |mut acc, n| {
                acc.push(n);
                acc
            })
            .parse()?;
            // .expect("Error parsing col position");

        (parsed_cols.saturating_sub(1) , parsed_rows.saturating_sub(1))
    }

    pub fn hide_cursor() {
        output::prints(&cursor::hide_cursor());
    }

    pub fn show_cursor() {
        output::prints(&cursor::show_cursor());
    }

    // SCREEN FUNCTIONS
    pub fn clear(method: Clear) {
        output::prints(&screen::clear(method));
    }

    pub fn size() -> (i16, i16) {
        screen::size()
    }

    pub fn resize(w: i16, h: i16) {
        output::printf(&screen::resize(w, h));
    }

    pub fn enable_alt() {
        output::printf(&screen::enable_alt());
    }

    pub fn disable_alt() {
        output::printf(&screen::disable_alt());
    }

    // OUTPUT FUNCTIONS
    pub fn prints(content: &str) -> Result<()> {
        output::prints(content)
    }

    pub fn printf(content: &str) -> Result<()> {
        output::printf(content)
    }

    pub fn flush() -> Result<()> {
        output::flush()
    }

    pub fn raw() -> Result<()> {
        output::enable_raw()
    }

    pub fn cook(initial: &Termios) -> Result<()> {
        output::set_mode(initial)
    }

    // MOUSE FUNCTIONS
    pub fn enable_mouse() {
        output::prints(&mouse::enable_mouse_mode());
    }

    pub fn disable_mouse() {
        output::prints(&mouse::disable_mouse_mode());
    }

    // STYLE FUNCTIONS
    pub fn set_fx(effects: u32) {
        output::prints(&style::set_style(Style::Fx(effects)));
    }

    pub fn set_fg(color: Color) {
        output::prints(&style::set_style(Style::Fg(color)));
    }

    pub fn set_bg(color: Color) {
        output::prints(&style::set_style(Style::Bg(color)));
    }

    pub fn set_styles(fg: Color, bg: Color, fx: u32) {
        output::prints(&style::set_styles(fg, bg, fx));
    }

    pub fn reset_styles() {
        output::prints(&style::reset());
    }

    // CONFIG FUNCTIONS
    // pub fn get_mode() -> Result<Termios> {
    //     output::get_mode()
    // }

    pub fn init_data(&self) -> Termios {
        self.mode
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        self.mode = 0;
    }
}