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

    pub fn query_pos() {
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


    // UNIFIED STRUCT W/ ACTIONS
    pub struct Term { mode: Termios }
    impl Term {
        pub fn new() -> Self {
            Self { mode: get_mode() }
        }
        pub fn goto(&self, col: i16, row: i16) { goto(col, row) }
        pub fn up(&self, n: i16) { up(n) }
        pub fn down(&self, n: i16) { down(n) }
        pub fn left(&self, n: i16) { left(n) }
        pub fn right(&self, n: i16) { right(n) }
        pub fn query_pos(&self) { query_pos() }
        pub fn pos_raw(&self) -> (i16, i16) { pos_raw() }
        pub fn hide_cursor(&self) { hide_cursor() }
        pub fn show_cursor(&self) { show_cursor() }
        pub fn clear(&self, method: Clear) { clear(method) }
        pub fn size(&self) -> (i16, i16) { size() }
        pub fn resize(&self, w: i16, h: i16) { resize(w, h) }
        pub fn enable_alt(&self) { enable_alt() }
        pub fn disable_alt(&self) { disable_alt() }
        pub fn prints(&self, content: &str) { prints(content) }
        pub fn printf(&self, content: &str) { printf(content) }
        pub fn flush(&self) { flush() }
        pub fn raw(&self) { raw() }
        pub fn cook(&self) { cook(&self.mode) }
        pub fn enable_mouse(&self) { enable_mouse() }
        pub fn disable_mouse(&self) { disable_mouse() }
        pub fn set_fx(&self, effects: u32) { set_fx(effects) }
        pub fn set_fg(&self, color: Color) { set_fg(color) }
        pub fn set_bg(&self, color: Color) { set_bg(color) }
        pub fn set_styles(
            &self, fg: Color, bg: Color, fx: u32
        ) { set_styles(fg, bg, fx) }
        pub fn reset_styles(&self) { reset_styles() }
        pub fn get_mode(&self) -> Result<Termios> { get_mode() }
    }

    impl Drop for Term {
        fn drop(&mut self) {
            self.mode = 0;
        }
    }
}


#[cfg(windows)]
pub mod win32 {
    use std::io::{ Result, Error, ErrorKind };
    use super::{ ansi, wincon };
    use crate::common::enums::{ Clear, Style, Color };
    pub use super::wincon::handle::{ Handle, ConsoleInfo };

    // CURSOR FUNCTIONS
    pub fn goto(
        col: i16, row: i16, conout: &Handle, ansi: bool
    ) -> Result<()> {
        let (mut col, mut row) = (col, row);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }

        if ansi { ansi::output::prints(
            &ansi::cursor::goto(col, row)); return Ok(()) }
        // let err_msg = "Error setting the cursor position";
        wincon::cursor::goto(col, row, &conout)
    }

    pub fn up(n: i16, conout: &Handle, ansi: bool) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if ansi { ansi::output::prints(
            &ansi::cursor::move_up(n)); return Ok(()) }
        // let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_up(n, &conout)
    }

    pub fn down(n: i16, conout: &Handle, ansi: bool) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if ansi { ansi::output::prints(
            &ansi::cursor::move_down(n)); return Ok(()) }
        // let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_down(n, &conout)
    }

    pub fn left(n: i16, conout: &Handle, ansi: bool) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if ansi { ansi::output::prints(
            &ansi::cursor::move_left(n)); return Ok(()) }
        // let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_left(n, &conout)
    }

    pub fn right(n: i16, conout: &Handle, ansi: bool) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if ansi { ansi::output::prints(
            &ansi::cursor::move_right(n)); return Ok(()) }
        // let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_right(n, &conout)
    }

    pub fn pos(conout: &Handle) -> Result<(i16, i16)> {
        // let err_msg = "Error getting cursor positions";
        wincon::cursor::pos(&conout)
    }

    pub fn hide_cursor(conout: &Handle, ansi: bool) -> Result<()> {
        if ansi { ansi::output::prints(
            &ansi::cursor::hide_cursor()); return Ok(()) }
        // let err_msg = "Error setting cursor visibility to 0";
        wincon::cursor::hide_cursor(&conout)
    }

    pub fn show_cursor(conout: &Handle, ansi: bool) -> Result<()> {
        if ansi { ansi::output::prints(
            &ansi::cursor::show_cursor()); return Ok(()) }
        // let err_msg = "Error setting cursor visibility to 100";
        wincon::cursor::show_cursor(&conout)
    }

    // SCREEN FUNCTIONS
    pub fn clear(method: Clear, conout: &Handle, ansi: bool) -> Result<()> {
        if ansi { ansi::output::prints(
            &ansi::screen::clear(method)); return Ok(()) }
        // let err_msg = "Error clearing the screen";
        wincon::screen::clear(method, &conout)
    }

    pub fn size(conout: &Handle) -> Result<(i16, i16)> {
        // let err_msg = "Error getting screen size"
        wincon::screen::size(&conout)
    }

    pub fn resize(w: i16, h: i16, conout: &Handle, ansi: bool) -> Result<()> {
        if ansi { ansi::output::prints(
            &ansi::screen::resize(w, h)); return Ok(()) }
        // let err_msg = "Error resizing the screen";
        wincon::screen::resize(w, h, &conout)
    }

    pub fn enable_alt(
        altout: &Handle, initial: &u32, ansi: bool
    ) -> Result<()> {
        if ansi { ansi::output::printf(
            &ansi::screen::enable_alt()); return Ok(()) }
        // let err_msg = "Error initializing alternate screen settings";
        altout.set_mode(initial)?;
        // let err_msg = "Error showing the alternate screen"
        altout.show()
    }

    pub fn disable_alt(ansi: bool) -> Result<()> {
        if ansi { ansi::output::printf(
            &ansi::screen::disable_alt()); return Ok(()) }
        // let err_msg = "Error switching back to $STDOUT";
        wincon::screen::disable_alt()
    }

    // OUTPUT FUNCTIONS
    pub fn prints(content: &str, conout: &Handle, ansi: bool) -> Result<()> {
        if ansi { ansi::output::prints(content); return Ok(()) }
        // let err_msg = "Error writing to console";
        wincon::output::prints(content, &conout)
    }

    pub fn printf(content: &str, conout: &Handle, ansi: bool) -> Result<()> {
        if ansi { ansi::output::printf(content); return Ok(()) }
        // let err_msg = "Error writing to console";
        wincon::output::prints(content, &conout)
    }

    pub fn flush(ansi: bool) {
        if ansi { ansi::output::flush(); return }
        () // (imdaveho) NOTE: Win32 flush is simply a no-op.
    }

    pub fn raw(conout: &Handle) -> Result<()> {
        // let err_msg = "Error enabling raw mode";
        wincon::output::enable_raw(&conout)
    }

    pub fn cook(conout: &Handle) -> Result<()> {
        // let err_msg = "Error disabling raw mode";
        wincon::output::disable_raw(&conout)
    }

    // MOUSE FUNCTIONS
    pub fn enable_mouse(conin: &Handle, ansi: bool) -> Result<()> {
        if ansi { ansi::output::prints(
            &ansi::mouse::enable_mouse_mode()); return Ok(()) }
        // let err_msg = "Error enabling mouse mode";
        wincon::mouse::enable_mouse_mode(&conin)
    }

    pub fn disable_mouse(conin: &Handle, ansi: bool) -> Result<()> {
        if ansi { ansi::output::prints(
            &ansi::mouse::disable_mouse_mode()); return Ok(()) }
        // let err_msg = "Error disabling mouse mode";
        wincon::mouse::disable_mouse_mode(&conin)
    }

    // STYLE FUNCTIONS
    pub fn set_fx(effects: u32, conout: &Handle, ansi: bool) -> Result<()> {
        if ansi { ansi::output::prints(
            &ansi::style::set_style(Style::Fx(effects))); return Ok(()) }
        // let err_msg = "Error setting console text attributes";
        wincon::style::set_style(Style::Fx(effects), 0, &conout)
    }

    pub fn set_fg(
        color: Color, reset: u16, conout: &Handle, ansi: bool
    ) -> Result<()> {
        if ansi { ansi::output::prints(
            &ansi::style::set_style(Style::Fg(color))); return Ok(()) }
        // let err_msg = "Error setting console foreground";
        wincon::style::set_style(Style::Fg(color), reset, &conout)
    }

    pub fn set_bg(
        color: Color, reset: u16, conout: &Handle, ansi: bool
    ) -> Result<()> {
        if ansi { ansi::output::prints(
            &ansi::style::set_style(Style::Bg(color))); return Ok(()) }
        // let err_msg = "Error setting console background";
        wincon::style::set_style(Style::Bg(color), reset, &conout)
    }

    pub fn set_styles(
        fg: Color, bg: Color, fx: u32, 
        reset: u16, conout: &Handle, ansi: bool
    ) -> Result<()> {
        if ansi { ansi::output::prints(
            &ansi::style::set_styles(fg, bg, fx)); return Ok(()) }
        // let err_msg = "Error setting multiple console styles";
        wincon::style::set_styles(fg, bg, fx, reset, &conout)
    }

    pub fn reset_styles(reset: u16, vte: bool) -> Result<()> {
        if vte { ansi::output::prints(
            &ansi::style::reset()); return Ok(()) }
        // let err_msg = "Error unsetting console styles";
        let handle = Handle::conout()?;
        wincon::style::reset(reset, &handle)?;
        handle.close()?;
        Ok(())
    }

    // CONFIG FUNCTIONS
    pub fn get_mode() -> Result<u32> {
        wincon::output::get_mode()
    }

    pub fn get_attrib(conout: &Handle) -> Result<u16> {
        Ok(ConsoleInfo::of(&conout)?.attributes())
    }

    // Windows Console API specific. Allows you to update the text
    // styles without having to re-print. 
    pub fn set_attrib(
        word: u16, length: u32, coord: (i16, i16)
    ) -> Result<()> {
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
                Ok(_) => true,
                Err(_) => false
            }
        }
    }


    // UNIFIED STRUCT W/ ACTIONS
    pub struct Term {
        mode: u32,
        reset: u16,
        conout: Handle,
        conin: Handle,
        altout: Option<Handle>,
        ansi: bool
    }

    impl Term {
        pub fn new() -> Result<Self> {
            let mode = get_mode()?;
            // let ansi = is_ansi_enabled();
            let ansi = false;
            // let err_msg = "Error fetching CONOUT$";
            let conout = Handle::conout()?;
            let reset = get_attrib(&conout)?;
            // let err_msg = "Error fetching CONIN$";
            let conin = Handle::conin()?;
            let altout = None;
            Ok(Self{ mode, reset, conout, conin, altout, ansi })
        }
        // CURSOR FUNCTIONS    
        pub fn goto(&self, col: i16, row: i16) -> Result<()> { 
            goto(col, row, &self.conout, self.ansi) 
        }
        pub fn up(&self, n: i16) -> Result<()> { 
            up(n, &self.conout, self.ansi)
        }
        pub fn down(&self, n: i16) -> Result<()> {
            down(n, &self.conout, self.ansi) 
        }
        pub fn left(&self, n: i16) -> Result<()> {
            left(n, &self.conout, self.ansi)
        }
        pub fn right(&self, n: i16) -> Result<()> { 
            right(n, &self.conout, self.ansi) 
        }
        pub fn pos(&self) -> Result<(i16, i16)> {
            pos(&self.conout)
        }
        pub fn hide_cursor(&self) -> Result<()> {
            hide_cursor(&self.conout, self.ansi)
        }
        pub fn show_cursor(&self) -> Result<()> {
            show_cursor(&self.conout, self.ansi)
        }
        // SCREEN FUNCTIONS
        pub fn clear(&self, method: Clear) -> Result<()> {
            clear(method, &self.conout, self.ansi)
        }
        pub fn size(&self) -> Result<(i16, i16)> {
            size(&self.conout)
        }
        pub fn resize(&self, w: i16, h: i16) -> Result<()> {
            resize(w, h, &self.conout, self.ansi)
        }
        pub fn enable_alt(&mut self) -> Result<()> {
            if self.altout.is_none() {
                self.altout = Some(Handle::buffer()?)
            }
            match &self.altout {
                Some(screen) => {
                    enable_alt(screen, &self.mode, self.ansi)?;
                    self.conout.close()?;
                    self.conout = Handle::conout()?;
                    Ok(())
                },
                None => Err(Error::new(ErrorKind::Other, 
                    "Could not find the alternate screen."))
            }
        }
        pub fn disable_alt(&mut self) -> Result<()> {
            disable_alt(self.ansi)?;
            self.conout.close()?;
            self.conout = Handle::conout()?;
            Ok(())
        }
        // OUTPUT FUNCTIONS
        pub fn prints(&self, content: &str) -> Result<()> {
            prints(content, &self.conout, self.ansi)
        }
        pub fn printf(&self, content: &str) -> Result<()> {
            printf(content, &self.conout, self.ansi)
        }
        pub fn flush(&self) { flush(self.ansi) }
        pub fn raw(&self) -> Result<()> {
            raw(&self.conout)
        }
        pub fn cook(&self) -> Result<()> {
            cook(&self.conout)
        }
        // MOUSE FUNCTIONS
        pub fn enable_mouse(&self) -> Result<()> {
            enable_mouse(&self.conin, self.ansi)
        }
        pub fn disable_mouse(&self) -> Result<()> {
            disable_mouse(&self.conin, self.ansi)
        }
        // STYLE FUNCTIONS
        pub fn set_fx(&self, effects: u32) -> Result<()> {
            set_fx(effects, &self.conout, self.ansi)
        }
        pub fn set_fg(&self, color: Color) -> Result<()> {
            set_fg(color, self.reset, &self.conout, self.ansi)
        }
        pub fn set_bg(&self, color: Color) -> Result<()> {
            set_bg(color, self.reset, &self.conout, self.ansi)
        }
        pub fn set_styles(&self, fg: Color, bg: Color, fx: u32) -> Result<()> {
            set_styles(fg, bg, fx, self.reset, &self.conout, self.ansi)
        }
        pub fn reset_styles(&self) -> Result<()> {
            reset_styles(self.reset, self.ansi)
        }
        // CONFIG FUNCTIONS
        pub fn get_mode(&self) -> Result<u32> {
            get_mode()
        }
        pub fn get_attrib(&self) -> Result<u16> {
            get_attrib(&self.conout)
        }
        pub fn set_attrib(word: u16, length: u32, coord: (i16, i16)) -> Result<()> {
            set_attrib(word, length, coord)
        }
        // TERM STRUCT SPECIFIC
        pub fn with(&mut self, mode: u32, reset: u16, ansi: bool) {
            self.mode = mode;
            self.reset = reset;
            self.ansi = ansi;
        }
        pub fn conout(&mut self, conout: Handle) -> Result<()> {
            self.conout.close()?;
            self.conout = conout;
            Ok(())
        }
        pub fn conin(&mut self, conin: Handle) -> Result<()> {
            self.conin.close()?;
            self.conin = conin;
            Ok(())
        }
        pub fn close(&mut self) -> Result<()> {
            self.conout.close()?; 
            self.conin.close()?;
            if let Some(altout) = &self.altout {
                altout.close()?;
                self.altout = None;
            }
            Ok(())
        }
    }

    impl Drop for Term {
        fn drop(&mut self) {
            self.close().expect("Error closing the terminal.");
        }
    }
}
