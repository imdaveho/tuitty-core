// This module exposes terminal functions for Ansi and Windows Console.
mod ansi;
#[cfg(windows)]
mod wincon;


#[cfg(unix)]
pub mod posix {
    use std::io::Result;
    use super::ansi::*;
    use crate::common::enums::{ Clear, Style, Color };
    pub use libc::termios as Termios;

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

    pub fn pos() {
        // TODO: rename to query_pos()
        output::printf(&cursor::pos());
    }

    pub fn pos_raw() -> (i16, i16) {
        // Where is the cursor?
        // Use `ESC [ 6 n`.
        let mut stdout = std::io::stdout();
        let stdin = std::io::stdin();

        // Write command
        std::io::Write::write_all(&mut stdout, b"\x1B[6n")
            .expect("Error writing cursor report");
        std::io::Write::flush(&mut stdout)
            .expect("Error flushing cursor report");
        std::io::BufRead::read_until(&mut stdin.lock(), b'[', &mut vec![])
            .expect("Error reading cursor report");
        let mut rows = vec![];
        std::io::BufRead::read_until(&mut stdin.lock(), b';', &mut rows)
            .expect("Error reading cursor row");
        let mut cols = vec![];
        std::io::BufRead::read_until(&mut stdin.lock(),  b'R', &mut cols)
            .expect("Error reading cursor col");
        // remove delimiter
        rows.pop(); cols.pop();

        let parsed_rows: i16 = rows
            .into_iter()
            .map(|b| (b as char))
            .fold(String::new(), |mut acc, n| {
                acc.push(n);
                acc
            })
            .parse().expect("Error parsing row position");
        let parsed_cols: i16 = cols
            .into_iter()
            .map(|b| (b as char))
            .fold(String::new(), |mut acc, n| {
                acc.push(n);
                acc
            })
            .parse().expect("Error parsing col position");

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
    pub fn prints(content: &str) {
        output::prints(content);
    }

    pub fn printf(content: &str) {
        output::printf(content);
    }

    pub fn flush() {
        output::flush();
    }

    pub fn raw() -> Result<()> {
        // let err_msg = "Error enabling raw mode";
        output::enable_raw()
    }

    pub fn cook(initial: &Termios) -> Result<()> {
        // let err_msg = "Error disabling raw mode";
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
    pub fn get_mode() -> Result<Termios> {
        // let err_msg = "Error fetching Termios"
        output::get_mode()
    }


    // // UNIFIED STRUCT W/ ACTIONS
    // pub struct Term { mode: Termios }
    // impl Term {
    //     pub fn new() -> Self {
    //         Self { mode: get_mode() }
    //     }
    //     pub fn goto(&self, col: i16, row: i16) { goto(col, row) }
    //     pub fn up(&self, n: i16) { up(n) }
    //     pub fn down(&self, n: i16) { down(n) }
    //     pub fn left(&self, n: i16) { left(n) }
    //     pub fn right(&self, n: i16) { right(n) }
    //     pub fn query_pos(&self) { pos() }
    //     pub fn pos_raw(&self) -> (i16, i16) { pos_raw() }
    //     pub fn hide_cursor(&self) { hide_cursor() }
    //     pub fn show_cursor(&self) { show_cursor() }
    //     pub fn clear(&self, method: Clear) { clear(method) }
    //     pub fn size(&self) -> (i16, i16) { size() }
    //     pub fn resize(&self, w: i16, h: i16) { resize(w, h) }
    //     pub fn enable_alt(&self) { enable_alt() }
    //     pub fn disable_alt(&self) { disable_alt() }
    //     pub fn prints(&self, content: &str) { prints(content) }
    //     pub fn printf(&self, content: &str) { printf(content) }
    //     pub fn flush(&self) { flush() }
    //     pub fn raw(&self) { raw() }
    //     pub fn cook(&self) { cook(&self.mode) }
    //     pub fn enable_mouse(&self) { enable_mouse() }
    //     pub fn disable_mouse(&self) { disable_mouse() }
    //     pub fn set_fx(&self, effects: u32) { set_fx(effects) }
    //     pub fn set_fg(&self, color: Color) { set_fg(color) }
    //     pub fn set_bg(&self, color: Color) { set_bg(color) }
    //     pub fn set_styles(&self, fg: Color, bg: Color, fx: u32) {
    //         set_styles(fg, bg, fx)
    //     }
    //     pub fn reset_styles(&self) { reset_styles() }
    // }

}


#[cfg(windows)]
pub mod win32 {
    use std::io::Result;
    use super::{ ansi, wincon };
    use crate::common::enums::{ Clear, Style, Color };
    pub use super::wincon::handle::{ Handle, ConsoleInfo };

    // CURSOR FUNCTIONS
    pub fn goto(col: i16, row: i16, vte: bool) -> Result<()> {
        let (mut col, mut row) = (col, row);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }

        if vte { ansi::output::prints(
            &ansi::cursor::goto(col, row)); return Ok(()) }
        // let err_msg = "Error setting the cursor position";
        wincon::cursor::goto(col, row)
    }

    pub fn up(n: i16, vte: bool) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if vte { ansi::output::prints(
            &ansi::cursor::move_up(n)); return Ok(()) }
        // let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_up(n)
    }

    pub fn down(n: i16, vte: bool) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if vte { ansi::output::prints(
            &ansi::cursor::move_down(n)); return Ok(()) }
        // let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_down(n)
    }

    pub fn left(n: i16, vte: bool) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if vte { ansi::output::prints(
            &ansi::cursor::move_left(n)); return Ok(()) }
        // let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_left(n)
    }

    pub fn right(n: i16, vte: bool) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if vte { ansi::output::prints(
            &ansi::cursor::move_right(n)); return Ok(()) }
        // let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_right(n)
    }

    pub fn pos() -> Result<(i16, i16)> {
        // let err_msg = "Error getting cursor positions";
        wincon::cursor::pos()
    }

    pub fn hide_cursor(vte: bool) -> Result<()> {
        if vte { ansi::output::prints(
            &ansi::cursor::hide_cursor()); return Ok(()) }
        // let err_msg = "Error setting cursor visibility to 0";
        wincon::cursor::hide_cursor()
    }

    pub fn show_cursor(vte: bool) -> Result<()> {
        if vte { ansi::output::prints(
            &ansi::cursor::show_cursor()); return Ok(()) }
        // let err_msg = "Error setting cursor visibility to 100";
        wincon::cursor::show_cursor()
    }

    // SCREEN FUNCTIONS
    pub fn clear(method: Clear, vte: bool) -> Result<()> {
        if vte { ansi::output::prints(
            &ansi::screen::clear(method)); return Ok(()) }
        // let err_msg = "Error clearing the screen";
        wincon::screen::clear(method)
    }

    pub fn size() -> Result<(i16, i16)> {
        // let err_msg = "Error getting screen size"
        wincon::screen::size()
    }

    pub fn resize(w: i16, h: i16, vte: bool) -> Result<()> {
        if vte { ansi::output::prints(
            &ansi::screen::resize(w, h)); return Ok(()) }
        // let err_msg = "Error resizing the screen";
        wincon::screen::resize(w, h)
    }

    pub fn enable_alt(screen: &Handle, initial: &u32, vte: bool) -> Result<()> {
        if vte { ansi::output::printf(
            &ansi::screen::enable_alt()); return Ok(()) }
        // let err_msg = "Error initializing alternate screen settings";
        screen.set_mode(initial);
        // let err_msg = "Error showing the alternate screen"
        screen.show()
    }

    pub fn disable_alt(vte: bool) -> Result<()> {
        if vte { ansi::output::printf(
            &ansi::screen::disable_alt()); return Ok(()) }
        // let err_msg = "Error switching back to $STDOUT";
        wincon::screen::disable_alt()
    }

    // OUTPUT FUNCTIONS
    pub fn prints(content: &str, vte: bool) -> Result<()> {
        if vte { ansi::output::prints(content); return Ok(()) }
        // let err_msg = "Error writing to console";
        wincon::output::prints(content)
    }

    pub fn printf(content: &str, vte: bool) -> Result<()> {
        if vte { ansi::output::printf(content); return Ok(()) }
        // let err_msg = "Error writing to console";
        wincon::output::prints(content)
    }

    pub fn flush(vte: bool) {
        if vte { ansi::output::flush(); return }
        () // (imdaveho) NOTE: Win32 flush is simply a no-op.
    }

    pub fn raw() -> Result<()> {
        // let err_msg = "Error enabling raw mode";
        wincon::output::enable_raw()
    }

    pub fn cook() -> Result<()> {
        // let err_msg = "Error disabling raw mode";
        wincon::output::disable_raw()
    }

    // MOUSE FUNCTIONS
    pub fn enable_mouse(vte: bool) -> Result<()> {
        if vte { ansi::output::prints(
            &ansi::mouse::enable_mouse_mode()); return Ok(()) }
        // let err_msg = "Error enabling mouse mode";
        wincon::mouse::enable_mouse_mode()
    }

    pub fn disable_mouse(vte: bool) -> Result<()> {
        if vte { ansi::output::prints(
            &ansi::mouse::disable_mouse_mode()); return Ok(()) }
        let err_msg = "Error disabling mouse mode";
        wincon::mouse::disable_mouse_mode()
    }

    // STYLE FUNCTIONS
    pub fn set_fx(effects: u32, vte: bool) -> Result<()> {
        if vte { ansi::output::prints(
            &ansi::style::set_style(Style::Fx(effects))); return Ok(()) }
        // let err_msg = "Error setting console text attributes";
        wincon::style::set_style(Style::Fx(effects), 0)
    }

    pub fn set_fg(color: Color, reset: u16, vte: bool) -> Result<()> {
        if vte { ansi::output::prints(
            &ansi::style::set_style(Style::Fg(color))); return Ok(()) }
        // let err_msg = "Error setting console foreground";
        wincon::style::set_style(Style::Fg(color), reset)
    }

    pub fn set_bg(color: Color, reset: u16, vte: bool) -> Result<()> {
        if vte { ansi::output::prints(
            &ansi::style::set_style(Style::Bg(color))); return Ok(()) }
        // let err_msg = "Error setting console background";
        wincon::style::set_style(Style::Bg(color), reset)
    }

    pub fn set_styles(fg: Color, bg: Color, fx: u32, reset: u16, vte: bool) -> Result<()> {
        if vte { ansi::output::prints(
            &ansi::style::set_styles(fg, bg, fx)); return Ok(()) }
        // let err_msg = "Error setting multiple console styles";
        wincon::style::set_styles(fg, bg, fx, reset)
    }

    pub fn reset_styles(reset: u16, vte: bool) -> Result<()> {
        if vte { ansi::output::prints(
            &ansi::style::reset()); return Ok(()) }
        // let err_msg = "Error unsetting console styles";
        wincon::style::reset(reset)
    }

    // CONFIG FUNCTIONS
    pub fn get_mode() -> Result<u32> {
        // let err_msg = "Error fetching mode from $STDOUT"
        wincon::output::get_mode()
    }

    pub fn get_attrib() -> Result<u16> {
        // let conout_err = "Error fetching $CONOUT";
        let handle = Handle::conout()?;
        // let coninfo_err = "Error fetching ConsoleInfo from $CONOUT";
        let attrib = ConsoleInfo::of(&handle)?.attributes();
        // let conout_err = "Error closing $CONOUT";
        handle.close()?;
        Ok(attrib)
    }

    // Windows Console API specific. Allows you to update the text
    // styles without having to re-print. 
    pub fn set_attrib(word: u16, length: u32, coord: (i16, i16)) -> Result<()> {
        // let err_msg = "Error setting console output attributes";
        wincon::style::set_attribute(word, length, coord)
    }

    pub fn is_ansi_enabled() -> bool {
        const TERMS: [&'static str; 15] = [
            "xterm",  // xterm, PuTTY, Mintty
            "rxvt",   // RXVT
            "eterm",  // Eterm
            "screen", // GNU screen, tmux
            "tmux",   // tmux
            "vt100", "vt102", "vt220", "vt320",   // DEC VT series
            "ansi",    // ANSI
            "scoansi", // SCO ANSI
            "cygwin",  // Cygwin, MinGW
            "linux",   // Linux console
            "konsole", // Konsole
            "bvterm",  // Bitvise SSH Client
        ];

        let matched_terms = match std::env::var("TERM") {
            Ok(val) => val != "dumb" || TERMS.contains(&val.as_str()),
            Err(_) => false,
        };

        if matched_terms {
            return true
        } else {
            let enable_vt = 0x0004;
            let handle = match Handle::stdout() {
                Ok(h) => h,
                Err(_) => return false,
            };
            let mode = match handle.get_mode() {
                Ok(m) => m,
                Err(_) => return false,
            };
            match handle.set_mode(&(mode | enable_vt)) {
                Ok(_) => { handle.close(); return true },
                Err(_) => { handle.close(); return false }
            }
        }
    }


//     // UNIFIED STRUCT W/ ACTIONS
//     pub struct Term {
//         mode: u32,
//         #[cfg(windows)]
//         reset: ConsoleInfo,
//         #[cfg(windows)]
//         conout: Handle,
//         //conin: Handle,
//         cfg: bool
//     }
//     impl Term {
//         pub fn new() -> Self {
//             let conout = Handle::conout()
//                 .expect("Error fetching CONOUT$");
//             let reset = ConsoleInfo::of(&conout)
//                 .expect("Error fetching ConsoleInfo from CONOUT$");
//             Self {
//                 mode: get_mode(),
//                 reset, conout,
//                 cfg: is_ansi_enabled()
//             }
//         }
//         pub fn goto(&self, col: i16, row: i16) { goto(col, row, self.cfg) }
//         pub fn up(&self, n: i16) { up(n, self.cfg) }
//         pub fn down(&self, n: i16) { down(n, self.cfg) }
//         pub fn left(&self, n: i16) { left(n, self.cfg) }
//         pub fn right(&self, n: i16) { right(n, self.cfg) }
//         // TODO TODO TODO - should pass in self.conout ???
//         pub fn pos_raw(&self) -> (i16, i16) {
//             let info = ConsoleInfo::of(&self.conout)
//                 .expect("Error fetching cursor position from CONOUT$");
//             info.cursor_pos()
//         }
//         pub fn hide_cursor() { () }

//     }
}
