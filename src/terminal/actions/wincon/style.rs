// Windows Console API specific functions to style text output to the terminal.

use std::io::{Error, Result};
use winapi::um::wincon::{
    COORD,
    SetConsoleTextAttribute, WriteConsoleOutputAttribute
};
use winapi::shared::minwindef::WORD;
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
    handle.close()?;
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
    handle.close()?;
    Ok(())
}

pub fn set_styles(fg: Color, bg: Color, fx: u32, reset_style: u16) -> Result<()> {
    set_style(Style::Fg(fg), reset_style)?;
    set_style(Style::Bg(bg), reset_style)?;
    set_style(Style::Fx(fx), reset_style)?;
    Ok(())
}

// Windows Console API specific. Allows you to update the text
// styles without having to re-print. 
pub fn set_attribute(word: u16, length: u32, coord: (i16, i16)) -> Result<()> {
    let handle = Handle::conout()?;

    let mut count = 0;
    let set: Vec<WORD> = vec![word as WORD; length as usize];
    let ptr: *const WORD = set.as_ptr() as *const WORD;

    unsafe {
        if WriteConsoleOutputAttribute(
            handle.0, 
            ptr, length, 
            COORD {X: coord.0, Y: coord.1}, 
            &mut count) == 0 {
                return Err(Error::last_os_error());
            }
    }
    handle.close()?;
    Ok(())
}