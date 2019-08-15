//! Implements platform specific functions to style text output to the terminal.
use super::{
    Color, TtyResult, TtyErrorKind, Error, Result, 
    Termios, Handle, ConsoleInfo
};
use winapi::um::wincon::{
    SetConsoleTextAttribute,
    FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_INTENSITY,
    BACKGROUND_RED, BACKGROUND_GREEN, BACKGROUND_BLUE, BACKGROUND_INTENSITY,
};



fn fg_color_val(color: Color) -> u16 {
    match color {
        Color::Black => 0,
        Color::DarkGrey => FOREGROUND_INTENSITY,
        Color::Red => FOREGROUND_INTENSITY | FOREGROUND_RED,
        Color::DarkRed => FOREGROUND_RED,
        Color::Green => FOREGROUND_INTENSITY | FOREGROUND_GREEN,
        Color::DarkGreen => FOREGROUND_GREEN,
        Color::Yellow => FOREGROUND_INTENSITY | FOREGROUND_GREEN | FOREGROUND_RED,
        Color::DarkYellow => FOREGROUND_GREEN | FOREGROUND_RED,
        Color::Blue => FOREGROUND_INTENSITY | FOREGROUND_BLUE,
        Color::DarkBlue => FOREGROUND_BLUE,
        Color::Magenta => FOREGROUND_INTENSITY | FOREGROUND_RED | FOREGROUND_BLUE,
        Color::DarkMagenta => FOREGROUND_RED | FOREGROUND_BLUE,
        Color::Cyan => FOREGROUND_INTENSITY | FOREGROUND_GREEN | FOREGROUND_BLUE,
        Color::DarkCyan => FOREGROUND_GREEN | FOREGROUND_BLUE,
        Color::White => FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE,
        Color::Grey => FOREGROUND_INTENSITY | FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE,

        Color::Reset => <u16>::max_value(), // max_value will signal using `Termios.color` on Windows
        Color::Rgb { r: _, g: _, b: _ } => <u16>::max_value(), // unsupported
        Color::AnsiValue(_val) => <u16>::max_value(), // unsupported
    }
}

fn bg_color_val(color:Color) -> u16 {
    match color {
        Color::Black => 0,
        Color::DarkGrey => BACKGROUND_INTENSITY,
        Color::Red => BACKGROUND_INTENSITY | BACKGROUND_RED,
        Color::DarkRed => BACKGROUND_RED,
        Color::Green => BACKGROUND_INTENSITY | BACKGROUND_GREEN,
        Color::DarkGreen => BACKGROUND_GREEN,
        Color::Yellow => BACKGROUND_INTENSITY | BACKGROUND_GREEN | BACKGROUND_RED,
        Color::DarkYellow => BACKGROUND_GREEN | BACKGROUND_RED,
        Color::Blue => BACKGROUND_INTENSITY | BACKGROUND_BLUE,
        Color::DarkBlue => BACKGROUND_BLUE,
        Color::Magenta => BACKGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_BLUE,
        Color::DarkMagenta => BACKGROUND_RED | BACKGROUND_BLUE,
        Color::Cyan => BACKGROUND_INTENSITY | BACKGROUND_GREEN | BACKGROUND_BLUE,
        Color::DarkCyan => BACKGROUND_GREEN | BACKGROUND_BLUE,
        Color::White => BACKGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE,
        Color::Grey => BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE,

        Color::Reset => <u16>::max_value(), // max_value will signal using `Termios.color` on Windows
        Color::Rgb { r: _, g: _, b: _ } => <u16>::max_value(), // unsupported
        Color::AnsiValue(_val) => <u16>::max_value(), // unsupported
    }
}

pub fn _set_fg(fg_color: Color) -> TtyResult<()> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;

    let mut color: u16;
    let attrs = info.attributes();
    let fg = fg_color_val(fg_color);
    let bg = attrs & 0x0070;
    color = fg | bg;

    if (attrs & BACKGROUND_INTENSITY as u16) != 0 {
        color = color | BACKGROUND_INTENSITY as u16;
    }

    unsafe {
        if !(SetConsoleTextAttribute(handle.0, color) == 0) {
            return Err(TtyErrorKind::IoError(Error::last_os_error()));
        }
    }
    Ok(())
}

pub fn _set_bg(bg_color: Color) -> TtyResult<()> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;

    let mut color: u16;
    let attrs = info.attributes();
    let fg = attrs & 0x0007;
    let bg = bg_color_val(bg_color);
    color = fg | bg;

    if (attrs & FOREGROUND_INTENSITY as u16) != 0 {
        color = color | FOREGROUND_INTENSITY as u16;
    }

    unsafe {
        if !(SetConsoleTextAttribute(handle.0, color) == 0) {
            return Err(TtyErrorKind::IoError(Error::last_os_error()));
        }
    }
    Ok(())
}

pub fn _set_attr(_: u16) -> TtyResult<()> {
    // (imdaveho) TODO: need to implement windows attrs:
    // * Bold = 1,
    // * Dim = 2,
    // * Underline = 4,
    // * Reverse = 7,
    // * Hide = 8,
    Ok(())
}

pub fn _reset() -> TtyResult<()> {
    // (imdaveho) TODO: this should be at `Tty` level 
    // to be able to use `Termios.color`
    Ok(())
}