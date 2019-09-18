// Windows Console API specific functions to style text output to the terminal.

use std::io::{Error, Result};
use winapi::um::wincon::{
    SetConsoleTextAttribute,
    FOREGROUND_INTENSITY as INTENSE,
    COMMON_LVB_UNDERSCORE as UNDERLINE,
    COMMON_LVB_REVERSE_VIDEO as REVERSE
};
use super::handle:{Handle, ConsoleInfo};
use crate::common::enums::{Style, Color, Effect::*};


pub const RESET: u16 = 0xFFFF;
pub const IGNORE: u16 = 0xFFF0;


pub struct ConsoleOutput(u16);

impl ConsoleOutput {
    pub fn new() -> ConsoleOutput {
        ConsoleOutput(
            ConsoleInfo::of(
                &Handle::conout()
                .expect("Error fetching $CONOUT"))
            .expect("Error fetching ConsoleInfo from $CONOUT")
            .attributes()
        )
    }

    pub fn reset(&self) -> Result<()> {
        let handle = Handle::conout()?;
        unsafe {
            if SetConsoleTextAttribute(handle.0, self.0) == 0 {
                return Err(Error::last_os_error());
            }
        }
        Ok(())
    }

    pub fn set_style(&self, style: Style) -> Result<()> {
        let handle = Handle::conout()?;
        let info = ConsoleInfo::of(&handle)?;
        let current = info.attributes();
        // let (just_fg, just_bg, just_fx) = (
        //     current & 0x000f, current & 0x00f0, current & 0xdf00);
        let updated: ConsoleAttribute = match style {
            Style::Fg(c) => {
                let mut updated_fg = Foreground::from(c);
                if updated_fg == RESET { 
                    updated_fg = Foreground(self.0 & 0x000f)
                }
                if updated_fg == IGNORE {
                    updated_fg = Foreground(current & 0x000f)
                }
                // (imdaveho) NOTE: We need to isolate Colors in Windows
                // because Color attributes mix. So if you previously had
                // Color::Red and you wanted Color::Blue, that would end up
                // as Color::Magenta if you didn't first clear it out
                // updated_fg.0 | just_bg | just_fx
                updated_fg.0 | current & !0x000f
            },
            Style::Bg(c) => {
                let mut updated_bg = Background::from(c);
                if updated_bg == RESET {
                    updated_bg = Background(self.0 & 0x00f0)
                }
                if updated_bg == IGNORE {
                    updated_bg = Background(current & 0x00f0)
                }
                // (imdaveho) NOTE: We need to isolate Colors in Windows
                // because Color attributes mix. So if you previously had
                // Color::Red and you wanted Color::Blue, that would end up
                // as Color::Magenta if you didn't first clear it out
                // just_fg | updated_bg.0 | just_fx
                updated_bg.0 | current & !0x00f0
            },
            Style::Fx(f) => {
                let fxs = [Reset, Bold, Dim, Underline, Reverse, Hide];
                let mut updates = current;
                for fx in &fxs {
                    if (f & *fx as u32) != 0 {
                        match *fx {
                            Bold => updates |= INTENSE,
                            Dim => updates &= !INTENSE,
                            Underline => updates |= UNDERLINE,
                            Reverse => updates |= REVERSE,
                            Hide => {
                                // FOREGROUND and BACKGROUND color differ by 4
                                // bits; to convert from 0x0020 (BG Green) to
                                // 0x0002 (FG Green), shift right 4 bits. By
                                // making the FOREGROUND color the same as the
                                // BACKGROUND color, effectively you hide the
                                // printed content.
                                let updated_fg = (current & 0x00f0) >> 4;
                                // Since we identified the new FOREGROUND, we
                                // include it and remove it from the current
                                // attributes. The BACKGROUND should remain the
                                // same within the current attrs.
                                updates = updated_fg | current & !0x000f
                            },
                            Reset => updates = current & !0xdf00,
                        }
                    }
                }
                updates
            },
        };
        unsafe {
            if SetConsoleTextAttribute(handle.0, updated) == 0 {
                return Err(Error::last_os_error());
            }
        }
        Ok(())
    }

    pub fn set_styles(&self, fg: Color, bg: Color, fx: u32) -> Result<()> {
        self.set_style(Fg(fg))?;
        self.set_style(Bg(bg))?;
        self.set_style(Fx(fx))?;
        Ok(())
    }
}


struct Foreground(u16);

impl From<Color> for Foreground {
    fn from(src: Color) -> Self {
        use winapi::um::wincon::{
            FOREGROUND_RED as RED,
            FOREGROUND_GREEN as GREEN,
            FOREGROUND_BLUE as BLUE,
            FOREGROUND_INTENSITY as INTENSE,
        };

        match src {
            Color::Black => Self(0),
            Color::DarkGrey => Self(INTENSE),
            Color::Red => Self(RED | INTENSE),
            Color::DarkRed => Self(GREEN),
            Color::Green => Self(GREEN | INTENSE),
            Color::DarkGreen => Self(GREEN),
            Color::Yellow => Self(RED | GREEN | INTENSE),
            Color::DarkYellow => Self(RED | GREEN),
            Color::Blue => Self(BLUE | INTENSE),
            Color::DarkBlue => Self(BLUE),
            Color::Magenta => Self(RED | BLUE | INTENSE),
            Color::DarkMagenta => Self(RED | BLUE),
            Color::Cyan => Self(GREEN | BLUE | INTENSE),
            Color::DarkCyan => Self(GREEN | BLUE),
            Color::White => Self(RED | GREEN | BLUE),
            Color::Grey => Self(RED | GREEN | BLUE | INTENSE),
            Color::Reset => Self(RESET),
            Color::Rgb{r, g, b} => Self(IGNORE),
            Color::AnsiValue(_) => Self(IGNORE),
        }
    }
}

impl PartialEq<u16> for Foreground {
    fn eq(&self, other: &u16) -> bool {
        self.0 == (*other)
    }
}


struct Background(u16);

impl From<Color> for Background {
    fn from(src: Color) -> Self {
        use winapi::um::wincon::{
            BACKGROUND_RED as RED,
            BACKGROUND_GREEN as GREEN,
            BACKGROUND_BLUE as BLUE,
            BACKGROUND_INTENSITY as INTENSE,
        };

        match src {
            Color::Black => Self(0),
            Color::DarkGrey => Self(INTENSE),
            Color::Red => Self(RED | INTENSE),
            Color::DarkRed => Self(GREEN),
            Color::Green => Self(GREEN | INTENSE),
            Color::DarkGreen => Self(GREEN),
            Color::Yellow => Self(RED | GREEN | INTENSE),
            Color::DarkYellow => Self(RED | GREEN),
            Color::Blue => Self(BLUE | INTENSE),
            Color::DarkBlue => Self(BLUE),
            Color::Magenta => Self(RED | BLUE | INTENSE),
            Color::DarkMagenta => Self(RED | BLUE),
            Color::Cyan => Self(GREEN | BLUE | INTENSE),
            Color::DarkCyan => Self(GREEN | BLUE),
            Color::White => Self(RED | GREEN | BLUE),
            Color::Grey => Self(RED | GREEN | BLUE | INTENSE),
            Color::Reset => Self(RESET),
            Color::Rgb{r, g, b} => Self(IGNORE),
            Color::AnsiValue(_) => Self(IGNORE),
        }
    }
}

impl PartialEq<u16> for Background {
    fn eq(&self, other: &u16) -> bool {
        self.0 == (*other)
    }
}