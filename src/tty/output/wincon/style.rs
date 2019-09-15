// Windows Console API specific functions to style text output to the terminal.

use super::{
    Color, Error, Result, 
    Handle, ConsoleInfo,
    Style, Style::*,
    Effect, Effect::*,
    Foreground, Background, 
    RESET, IGNORE
};

use winapi::um::wincon::{
    SetConsoleTextAttribute,
    FOREGROUND_INTENSITY as INTENSE,
    COMMON_LVB_UNDERSCORE as UNDERLINE,
    COMMON_LVB_REVERSE_VIDEO as REVERSE
};


type ConsoleAttribute = u16;

pub struct ConsoleOutput(pub ConsoleAttribute);

impl ConsoleOutput {
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
        let current: ConsoleAttribute = info.attributes();
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
                for effect in &fxs {
                    if (f & *effect as u32) != 0 {
                        match *effect {
                            Effect::Bold => updates |= INTENSE,
                            Effect::Dim => updates &= !INTENSE,
                            Effect::Underline => updates |= UNDERLINE,
                            Effect::Reverse => updates |= REVERSE,
                            Effect::Hide => {
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
                            Effect::Reset => updates = current & !0xdf00,
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

    pub fn set_styles(&self, fgcol: Color, bgcol: Color, effects: u32) -> Result<()> {
        self.set_style(Fg(fgcol))?;
        self.set_style(Bg(bgcol))?;
        self.set_style(Fx(effects))?;
        Ok(())
    }
}