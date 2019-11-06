// This module exposes terminal functions for Ansi and Windows Console.

mod ansi;
#[cfg(windows)]
mod wincon;


#[cfg(unix)]
pub mod posix {
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
        output::printf(&cursor::pos());
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

    pub fn raw() {
        output::enable_raw().expect("Error enabling raw mode");
    }

    pub fn cook(initial: &Termios) {
        output::set_mode(initial).expect("Error disabling raw mode");
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
    pub fn get_mode() -> Termios {
        output::get_mode().expect("Error fetching Termios")
    }
}


#[cfg(windows)]
pub mod win32 {
    use super::{ ansi, wincon };
    use crate::common::enums::{ Clear, Style, Color };
    pub use super::wincon::handle::{ Handle, ConsoleInfo };

    // CURSOR FUNCTIONS
    pub fn goto(col: i16, row: i16, vte: bool) {
        let (mut col, mut row) = (col, row);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }

        if vte { ansi::output::prints(
            &ansi::cursor::goto(col, row)); return }
        let err_msg = "Error setting the cursor position";
        wincon::cursor::goto(col, row)
            .expect(err_msg);
    }

    pub fn up(n: i16, vte: bool) {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if vte { ansi::output::prints(
            &ansi::cursor::move_up(n)); return }
        let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_up(n).expect(&err_msg);
    }

    pub fn down(n: i16, vte: bool) {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if vte { ansi::output::prints(
            &ansi::cursor::move_down(n)); return }
        let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_down(n).expect(&err_msg);
    }

    pub fn left(n: i16, vte: bool) {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if vte { ansi::output::prints(
            &ansi::cursor::move_left(n)); return }
        let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_left(n).expect(&err_msg);
    }

    pub fn right(n: i16, vte: bool) {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if vte { ansi::output::prints(
            &ansi::cursor::move_right(n)); return }
        let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_right(n).expect(&err_msg);
    }

    pub fn pos() -> (i16, i16) {
        let err_msg = "Error getting cursor positions";
        wincon::cursor::pos().expect(err_msg)
    }

    pub fn hide_cursor(vte: bool) {
        if vte { ansi::output::prints(
            &ansi::cursor::hide_cursor()); return }
        let err_msg = "Error setting cursor visibility to 0";
        wincon::cursor::hide_cursor().expect(err_msg);
    }

    pub fn show_cursor(vte: bool) {
        if vte { ansi::output::prints(
            &ansi::cursor::show_cursor()); return }
        let err_msg = "Error setting cursor visibility to 100";
        wincon::cursor::show_cursor().expect(err_msg);
    }

    // SCREEN FUNCTIONS
    pub fn clear(method: Clear, vte: bool) {
        if vte { ansi::output::prints(
            &ansi::screen::clear(method)); return }
        let err_msg = "Error clearing the screen";
        wincon::screen::clear(method).expect(err_msg);
    }

    pub fn size() -> (i16, i16) {
        wincon::screen::size()
    }

    pub fn resize(w: i16, h: i16, vte: bool) {
        if vte { ansi::output::prints(
            &ansi::screen::resize(w, h)); return }
        let err_msg = "Error resizing the screen";
        wincon::screen::resize(w, h).expect(err_msg);
    }

    pub fn enable_alt(screen: &Handle, initial: &u32, vte: bool) {
        if vte { ansi::output::printf(
            &ansi::screen::enable_alt()); return }
        let err_msg = "Error initializing alternate screen settings";
        screen.set_mode(initial).expect(err_msg);
        screen.show().expect("Error showing the alternate screen");
    }

    pub fn disable_alt(vte: bool) {
        if vte { ansi::output::printf(
            &ansi::screen::disable_alt()); return }
        let err_msg = "Error switching back to $STDOUT";
        wincon::screen::disable_alt().expect(err_msg);
    }

    // OUTPUT FUNCTIONS
    pub fn prints(content: &str, vte: bool) {
        if vte { ansi::output::prints(content); return }
        let err_msg = "Error writing to console";
        wincon::output::prints(content).expect(err_msg);
    }

    pub fn printf(content: &str, vte: bool) {
        if vte { ansi::output::printf(content); return }
        let err_msg = "Error writing to console";
        wincon::output::prints(content).expect(err_msg);
    }

    pub fn flush(vte: bool) {
        if vte { ansi::output::flush(); return }
        () // (imdaveho) NOTE: Win32 flush is simply a no-op.
    }

    pub fn raw() {
        let err_msg = "Error enabling raw mode";
        wincon::output::enable_raw().expect(err_msg);
    }

    pub fn cook() {
        let err_msg = "Error disabling raw mode";
        wincon::output::disable_raw().expect(err_msg);
    }

    // MOUSE FUNCTIONS
    pub fn enable_mouse(vte: bool) {
        if vte { ansi::output::prints(
            &ansi::mouse::enable_mouse_mode()); return }
        let err_msg = "Error enabling mouse mode";
        wincon::mouse::enable_mouse_mode().expect(err_msg);
    }

    pub fn disable_mouse(vte: bool) {
        if vte { ansi::output::prints(
            &ansi::mouse::disable_mouse_mode()); return }
        let err_msg = "Error disabling mouse mode";
        wincon::mouse::disable_mouse_mode().expect(err_msg);
    }

    // STYLE FUNCTIONS
    pub fn set_fx(effects: u32, vte: bool) {
        if vte { ansi::output::prints(
            &ansi::style::set_style(Style::Fx(effects))); return }
        let err_msg = "Error setting console text attributes";
        wincon::style::set_style(Style::Fx(effects), 0).expect(err_msg);
    }

    pub fn set_fg(color: Color, reset: u16, vte: bool) {
        if vte { ansi::output::prints(
            &ansi::style::set_style(Style::Fg(color))); return }
        let err_msg = "Error setting console foreground";
        wincon::style::set_style(Style::Fg(color), reset).expect(err_msg);
    }

    pub fn set_bg(color: Color, reset: u16, vte: bool) {
        if vte { ansi::output::prints(
            &ansi::style::set_style(Style::Bg(color))); return }
        let err_msg = "Error setting console background";
        wincon::style::set_style(Style::Bg(color), reset).expect(err_msg);
    }

    pub fn set_styles(fg: Color, bg: Color, fx: u32, reset: u16, vte: bool) {
        if vte { ansi::output::prints(
            &ansi::style::set_styles(fg, bg, fx)); return }
        let err_msg = "Error setting multiple console styles";
        wincon::style::set_styles(fg, bg, fx, reset).expect(err_msg);
    }

    pub fn reset_styles(reset: u16, vte: bool) {
        if vte { ansi::output::prints(
            &ansi::style::reset()); return }
        let err_msg = "Error unsetting console styles";
        wincon::style::reset(reset).expect(err_msg);
    }

    // CONFIG FUNCTIONS
    pub fn get_mode() -> u32 {
        wincon::output::get_mode().expect("Error fetching mode from $STDOUT")
    }

    pub fn get_attrib() -> u16 {
        let conout_err = "Error fetching $CONOUT";
        let coninfo_err = "Error fetching ConsoleInfo from $CONOUT";
        ConsoleInfo::of(&Handle::conout().expect(conout_err))
            .expect(coninfo_err).attributes()
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
                Ok(_) => return true,
                Err(_) => return false,
            }
        }
    }
}
