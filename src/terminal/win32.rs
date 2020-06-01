use std::io::{ Result, Error, ErrorKind };
use crate::actions::{ ansi, wincon };
use crate::common::enums::{ Clear, Style, Color };
use wincon::handle::{ Handle, ConsoleInfo };


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
        let mode = wincon::output::get_mode()?;
        let ansi = is_ansi_enabled();
        let conout = Handle::conout()?;
        let reset = ConsoleInfo::of(&conout)?.attributes();
        let conin = Handle::conin()?;
        let altout = None;
        Ok(Self{ mode, reset, conout, conin, altout, ansi })
    }

    // CURSOR FUNCTIONS    
    pub fn goto(&self, col: i16, row: i16) -> Result<()> { 
        let (mut col, mut row) = (col, row);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }

        if self.ansi { ansi::output::prints(
            &ansi::cursor::goto(col, row)); return Ok(()) }
        // let err_msg = "Error setting the cursor position";
        wincon::cursor::goto(col, row, &self.conout)
    }

    pub fn up(&self, n: i16) -> Result<()> { 
        let mut n = n;
        if n < 0 { n = n.abs() }

        if self.ansi { ansi::output::prints(
            &ansi::cursor::move_up(n)); return Ok(()) }
        // let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_up(n, &self.conout)
    }

    pub fn down(&self, n: i16) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if self.ansi { ansi::output::prints(
            &ansi::cursor::move_down(n)); return Ok(()) }
        // let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_down(n, &self.conout)
    }

    pub fn left(&self, n: i16) -> Result<()> {
        let mut n = n;
        if n < 0 { n = n.abs() }

        if self.ansi { ansi::output::prints(
            &ansi::cursor::move_left(n)); return Ok(()) }
        // let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_left(n, &self.conout)
    }
    
    pub fn right(&self, n: i16) -> Result<()> { 
        let mut n = n;
        if n < 0 { n = n.abs() }

        if self.ansi { ansi::output::prints(
            &ansi::cursor::move_right(n)); return Ok(()) }
        // let err_msg = format!("Error moving the cursor up by {}", n);
        wincon::cursor::move_right(n, &self.conout)
    }
    
    pub fn pos(&self) -> Result<(i16, i16)> {
        // let err_msg = "Error getting cursor position";
        wincon::cursor::pos(&self.conout)
    }

    pub fn hide_cursor(&self) -> Result<()> {
        if self.ansi { ansi::output::prints(
            &ansi::cursor::hide_cursor()); return Ok(()) }
        // let err_msg = "Error setting cursor visibility to 0";
        wincon::cursor::hide_cursor(&self.conout)
    }

    pub fn show_cursor(&self) -> Result<()> {
        if self.ansi { ansi::output::prints(
            &ansi::cursor::show_cursor()); return Ok(()) }
        // let err_msg = "Error setting cursor visibility to 100";
        wincon::cursor::show_cursor(&self.conout)
    }

    // SCREEN FUNCTIONS
    pub fn clear(&self, method: Clear) -> Result<()> {
        if self.ansi { ansi::output::prints(
            &ansi::screen::clear(method)); return Ok(()) }
        // let err_msg = "Error clearing the screen";
        wincon::screen::clear(method, &self.conout)
    }
    
    pub fn size(&self) -> Result<(i16, i16)> {
        // let err_msg = "Error getting screen size"
        wincon::screen::size(&self.conout)
    }
    
    pub fn resize(&self, w: i16, h: i16) -> Result<()> {
        if self.ansi { ansi::output::prints(
            &ansi::screen::resize(w, h)); return Ok(()) }
        // let err_msg = "Error resizing the screen";
        wincon::screen::resize(w, h, &self.conout)
    }
    
    pub fn enable_alt(&mut self) -> Result<()> {
        if self.altout.is_none() {
            self.altout = Some(Handle::buffer()?)
        }
        match &self.altout {
            Some(screen) => {
                if self.ansi { ansi::output::printf(
                    &ansi::screen::enable_alt()); return Ok(()) }
                // let err_msg = "Error setting alternate screen mode";
                screen.set_mode(&self.mode)?;
                // let err_msg = "Error showing the alternate screen";
                screen.show()?;
                self.conout.close()?;
                self.conout = Handle::conout()?;
                Ok(())
            },
            None => Err(Error::new(ErrorKind::Other, 
                "Could not enable the alternate screen."))
        }
    }

    pub fn disable_alt(&mut self) -> Result<()> {
        if self.ansi { ansi::output::printf(
            &ansi::screen::disable_alt()); return Ok(()) }
        // let err_msg = "Error switching back to $STDOUT";
        wincon::screen::disable_alt()?;
        self.conout.close()?;
        self.conout = Handle::conout()?;
        Ok(())
    }

    // OUTPUT FUNCTIONS
    pub fn prints(&self, content: &str) -> Result<()> {
        if self.ansi { ansi::output::prints(content); return Ok(()) }
        // let err_msg = "Error writing to console";
        wincon::output::prints(content, &self.conout)
    }

    pub fn printf(&self, content: &str) -> Result<()> {
        if self.ansi { ansi::output::printf(content); return Ok(()) }
        // let err_msg = "Error writing to console";
        wincon::output::prints(content, &self.conout)
    }

    pub fn flush(&self) {
        if self.ansi { ansi::output::flush(); return }
        // (imdaveho) NOTE: Win32 flush is simply a no-op.
        ()
    }

    pub fn raw(&self) -> Result<()> {
        // let err_msg = "Error enabling raw mode";
        wincon::output::enable_raw(&self.conout)
    }

    pub fn cook(&self) -> Result<()> {
        // let err_msg = "Error disabling raw mode";
        wincon::output::disable_raw(&self.conout)
    }

    // MOUSE FUNCTIONS
    pub fn enable_mouse(&self) -> Result<()> {
        if self.ansi { ansi::output::prints(
            &ansi::mouse::enable_mouse_mode()); return Ok(()) }
        // let err_msg = "Error enabling mouse mode";
        wincon::mouse::enable_mouse_mode(&self.conin)
    }

    pub fn disable_mouse(&self) -> Result<()> {
        if self.ansi { ansi::output::prints(
            &ansi::mouse::disable_mouse_mode()); return Ok(()) }
        // let err_msg = "Error disabling mouse mode";
        wincon::mouse::disable_mouse_mode(&self.conin)
    }

    // STYLE FUNCTIONS
    pub fn set_fx(&self, effects: u32) -> Result<()> {
        if self.ansi { ansi::output::prints(
            &ansi::style::set_style(Style::Fx(effects))); return Ok(()) }
        // let err_msg = "Error setting console text attributes";
        wincon::style::set_style(Style::Fx(effects), 0, &self.conout)
    }

    pub fn set_fg(&self, color: Color) -> Result<()> {
        if self.ansi { ansi::output::prints(
            &ansi::style::set_style(Style::Fg(color))); return Ok(()) }
        // let err_msg = "Error setting console foreground";
        wincon::style::set_style(Style::Fg(color), self.reset, &self.conout)
    }

    pub fn set_bg(&self, color: Color) -> Result<()> {
        if self.ansi { ansi::output::prints(
            &ansi::style::set_style(Style::Bg(color))); return Ok(()) }
        // let err_msg = "Error setting console background";
        wincon::style::set_style(Style::Bg(color), self.reset, &self.conout)
    }

    pub fn set_styles(&self, fg: Color, bg: Color, fx: u32) -> Result<()> {
        if self.ansi { ansi::output::prints(
            &ansi::style::set_styles(fg, bg, fx)); return Ok(()) }
        // let err_msg = "Error setting multiple console styles";
        wincon::style::set_styles(fg, bg, fx, self.reset, &self.conout)
    }

    pub fn reset_styles(&self) -> Result<()> {
        if self.ansi { ansi::output::prints(
            &ansi::style::reset()); return Ok(()) }
        // let err_msg = "Error unsetting console styles";
        wincon::style::reset(self.reset, &self.conout)
    }

    // CONFIG FUNCTIONS
    pub fn get_mode(&self) -> Result<u32> {
        wincon::output::get_mode()
    }
    
    pub fn get_attrib(&self) -> Result<u16> {
        Ok(ConsoleInfo::of(&self.conout)?.attributes())
    }

    pub fn init_data(&self) -> (u32, u16, bool) {
        (self.mode, self.reset, self.ansi)
    }

    // Windows Console API specific. Allows you to update the text
    // styles without having to re-print. 
    pub fn set_attrib(
        &self, word: u16, length: u32, coord: (i16, i16)
    ) -> Result<()> {
        // let err_msg = "Error setting console output attributes";
        wincon::style::set_attribute(word, length, coord)
    }

    // TERM STRUCT SPECIFIC
    pub fn with(&mut self, mode: u32, reset: u16, ansi: bool) {
        self.mode = mode;
        self.reset = reset;
        self.ansi = ansi;
    }

    pub fn close(&mut self) -> Result<()> {
        // Revert back to original settings.
        self.disable_alt()?;
        self.reset_styles()?;
        // Clean up Handles.
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