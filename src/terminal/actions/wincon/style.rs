// Windows Console API specific functions to style text output to the terminal.

use std::io::{Error, Result};
use winapi::um::wincon::SetConsoleTextAttribute;
use super::handle::{Handle, ConsoleInfo};
use crate::common::enums::{
    Style, Color,
    foreground, background, effects
};


pub fn reset(reset_style: u16) -> Result<()> {
    let handle = Handle::conout()?;
    unsafe {
        if SetConsoleTextAttribute(handle.0, reset_style) == 0 {
            return Err(Error::last_os_error());
        }
    }
    // handle.close()?;
    Ok(())
}

pub fn set_style(style: Style, reset_style: u16) -> Result<()> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;
    let current = info.attributes();
    // let (just_fg, just_bg, just_fx) = (
    //     current & 0x000f, current & 0x00f0, current & 0xdf00);
    let updated: u16 = match style {
        Style::Fg(c) => {
            foreground(c, current, reset_style)
        },
        Style::Bg(c) => {
            background(c, current, reset_style)
        },
        Style::Fx(f) => {
            effects(f, current)
        }
    };
    unsafe {
        if SetConsoleTextAttribute(handle.0, updated) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn set_styles(fg: Color, bg: Color, fx: u32, reset_style: u16) -> Result<()> {
    set_style(Style::Fg(fg), reset_style)?;
    set_style(Style::Bg(bg), reset_style)?;
    set_style(Style::Fx(fx), reset_style)?;
    Ok(())
}