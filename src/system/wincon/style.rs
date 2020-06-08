// Windows Console API specific functions to style text output to the terminal.

use std::io::{Error, Result};
use winapi::um::wincon::{
    SetConsoleTextAttribute,
    // WriteConsoleOutputAttribute, COORD,
    COMMON_LVB_UNDERSCORE as UNDERLINE,
    COMMON_LVB_REVERSE_VIDEO as REVERSE,
    FOREGROUND_RED as FG_RED,
    FOREGROUND_GREEN as FG_GREEN,
    FOREGROUND_BLUE as FG_BLUE,
    FOREGROUND_INTENSITY as FG_INTENSE,
    BACKGROUND_RED as BG_RED,
    BACKGROUND_GREEN as BG_GREEN,
    BACKGROUND_BLUE as BG_BLUE,
    BACKGROUND_INTENSITY as BG_INTENSE,
};
// use winapi::shared::minwindef::WORD;
use super::handle::{Handle, ConsoleInfo};
use crate::common::enums::{Style, Color, Effect};


const RESET: u16 = 0xFFFF;
const IGNORE: u16 = 0xFFF0;


pub fn reset(reset_style: u16, conout: &Handle) -> Result<()> {
    unsafe {
        if SetConsoleTextAttribute(conout.0, reset_style) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn set_style(
    style: Style, reset_style: u16, conout: &Handle
) -> Result<()> {
    let info = ConsoleInfo::of(&conout)?;
    let current = info.attributes();
    // let (just_fg, just_bg, just_fx) = (
    //     current & 0x000f, current & 0x00f0, current & 0xdf00);
    let updated: u16 = match style {
        Style::Fg(c) => {
            into_fg(c, current, reset_style)
        },
        Style::Bg(c) => {
            into_bg(c, current, reset_style)
        },
        Style::Fx(f) => {
            into_fx(f, current)
        }
    };
    unsafe {
        if SetConsoleTextAttribute(conout.0, updated) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn set_styles(
    fg: Color, bg: Color, fx: u32, 
    reset_style: u16, conout: &Handle
) -> Result<()> {
    set_style(Style::Fg(fg), reset_style, conout)?;
    set_style(Style::Bg(bg), reset_style, conout)?;
    set_style(Style::Fx(fx), reset_style, conout)?;
    Ok(())
}


pub fn into_fg(color: Color, current: u16, reset: u16) -> u16 {
    let mut attrib = match color {
        Color::Black => 0,
        Color::DarkGrey => FG_INTENSE,
        Color::Red => FG_RED | FG_INTENSE,
        Color::DarkRed => FG_GREEN,
        Color::Green => FG_GREEN | FG_INTENSE,
        Color::DarkGreen => FG_GREEN,
        Color::Yellow => FG_RED | FG_GREEN | FG_INTENSE,
        Color::DarkYellow => FG_RED | FG_GREEN,
        Color::Blue => FG_BLUE | FG_INTENSE,
        Color::DarkBlue => FG_BLUE,
        Color::Magenta => FG_RED | FG_BLUE | FG_INTENSE,
        Color::DarkMagenta => FG_RED | FG_BLUE,
        Color::Cyan => FG_GREEN | FG_BLUE | FG_INTENSE,
        Color::DarkCyan => FG_GREEN | FG_BLUE,
        Color::White => FG_RED | FG_GREEN | FG_BLUE,
        Color::Grey => FG_RED | FG_GREEN | FG_BLUE | FG_INTENSE,
        Color::Reset => RESET,
        Color::Rgb{r: _, g: _, b: _} => IGNORE,
        Color::AnsiValue(_) => IGNORE,
    };
    if attrib == RESET {
        attrib = reset & 0x000f;
    }
    if attrib == IGNORE {
        attrib = current & 0x000f;
    }
    // (imdaveho) NOTE: We need to isolate Colors in Windows
    // because Color attributes mix. So if you previously had
    // Color::Red and you wanted Color::Blue, that would end up
    // as Color::Magenta if you didn't first clear it out
    // attrib | just_bg | just_fx
    attrib | current & !0x000f
}

pub fn into_bg(color: Color, current: u16, reset: u16) -> u16 {
    let mut attrib = match color {
        Color::Black => 0,
        Color::DarkGrey => BG_INTENSE,
        Color::Red => BG_RED | BG_INTENSE,
        Color::DarkRed => BG_GREEN,
        Color::Green => BG_GREEN | BG_INTENSE,
        Color::DarkGreen => BG_GREEN,
        Color::Yellow => BG_RED | BG_GREEN | BG_INTENSE,
        Color::DarkYellow => BG_RED | BG_GREEN,
        Color::Blue => BG_BLUE | BG_INTENSE,
        Color::DarkBlue => BG_BLUE,
        Color::Magenta => BG_RED | BG_BLUE | BG_INTENSE,
        Color::DarkMagenta => BG_RED | BG_BLUE,
        Color::Cyan => BG_GREEN | BG_BLUE | BG_INTENSE,
        Color::DarkCyan => BG_GREEN | BG_BLUE,
        Color::White => BG_RED | BG_GREEN | BG_BLUE,
        Color::Grey => BG_RED | BG_GREEN | BG_BLUE | BG_INTENSE,
        Color::Reset => RESET,
        Color::Rgb{r: _, g: _, b: _} => IGNORE,
        Color::AnsiValue(_) => IGNORE,
    };
    if attrib == RESET {
        attrib = reset & 0x00f0;
    }
    if attrib == IGNORE {
        attrib = current & 0x00f0;
    }
    // (imdaveho) NOTE: We need to isolate Colors in Windows
    // because Color attributes mix. So if you previously had
    // Color::Red and you wanted Color::Blue, that would end up
    // as Color::Magenta if you didn't first clear it out
    // just_fg | attrib | just_fx
    attrib | current & !0x00f0
    
}

pub fn into_fx(fx: u32, current: u16) -> u16 {
    let mut attrib = current;
    let available_effects = [
        Effect::Reset, 
        Effect::Bold, 
        Effect::Dim, 
        Effect::Underline, 
        Effect::Reverse, 
        Effect::Hide
    ];
    for effect in &available_effects {
        if (fx & *effect as u32) != 0 {
            match *effect {
                Effect::Bold => attrib |= FG_INTENSE,
                Effect::Dim => attrib &= !FG_INTENSE,
                Effect::Underline => attrib |= UNDERLINE,
                Effect::Reverse => attrib |= REVERSE,
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
                    attrib = updated_fg | current & !0x000f
                },
                Effect::Reset => attrib = current & !0xdf00,
            }
        }
    }
    attrib
}