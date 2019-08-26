// Windows Console API specific functions to style text output to the terminal.

use winapi::um::wincon::{
    SetConsoleTextAttribute,
    FOREGROUND_RED as FG_RED, FOREGROUND_GREEN as FG_GREEN,
    FOREGROUND_BLUE as FG_BLUE, FOREGROUND_INTENSITY as FG_INTENSITY,
    BACKGROUND_RED as BG_RED, BACKGROUND_GREEN as BG_GREEN,
    BACKGROUND_BLUE as BG_BLUE, BACKGROUND_INTENSITY as BG_INTENSITY,
    COMMON_LVB_UNDERSCORE, COMMON_LVB_REVERSE_VIDEO,
};
use super::{
    Color, Error, Result, Style,
    Handle, ConsoleInfo, TextStyle,
};


type Fg = u16;
type Bg = u16;
type Attrs = u16;


pub fn set_fg(color: Color, reset: u16) -> Result<()> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;
    let curr_at = info.attributes();
    let mut attr: Attrs = _stylize(Style::Fg(color), curr_at);
    if attr ==  <u16>::max_value() {
        // Reset Fg from Original
        attr = (reset & 0x000f)
        | (curr_at & 0x00f0)
        | (curr_at & 0xdf00);
    }
    unsafe {
        if SetConsoleTextAttribute(handle.0, attr) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn set_bg(color: Color, reset: u16) -> Result<()> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;
    let curr_at = info.attributes();
    let mut attr: Attrs = _stylize(Style::Bg(color), curr_at);
    if attr == <u16>::max_value() {
        // Reset Fg from stored attrs
        attr = (curr_at & 0x000f)
        | (reset & 0x00f0)
        | (curr_at & 0xdf00);
    }
    unsafe {
        if SetConsoleTextAttribute(handle.0, attr) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn set_tx(text_style: TextStyle) -> Result<()> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;
    let curr_at = info.attributes();
    let attr: Attrs = _stylize(Style::Tx(text_style), curr_at);
    unsafe {
        if SetConsoleTextAttribute(handle.0, attr) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn set_all(fg: &str, bg: &str, tx: &str, reset: u16) -> Result<()> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;
    let curr_at = info.attributes();
    // Start with getting only the Fg Attributes.
    let (fg_attr, bg_attr, mut attrs): (u16, u16, u16);
    match fg {
        "reset" => fg_attr = reset & 0x000f,
        _ => {
            let mask_fg = 0x000f;
            let attr: Fg = _stylize(Style::Fg(Color::from(fg)), curr_at);
            if attr == <u16>::max_value() {
                // Return existing Fg w/o changes.
                fg_attr = curr_at & mask_fg
            } else { fg_attr = attr & mask_fg }
        }
    }
    // Then getting only the Bg Attributes.
    match bg {
        "reset" => bg_attr = reset & 0x00f0,
        _ => {
            let mask_bg = 0x00f0;
            let attr: Bg = _stylize(Style::Bg(Color::from(bg)), curr_at);
            if attr == <u16>::max_value() {
                // Return existing Bg without changes.
                bg_attr = curr_at & mask_bg
            } else { bg_attr = attr & mask_bg }
        }
    }

    // The tx param is should be a comma separated string.
    let tx_arr: Vec<&str> = tx.split(',').map(|t| t.trim()).collect();
    // Combine Fg and Bg into the remaining and additively
    // apply each text style.
    attrs = fg_attr | bg_attr;
    // Dim may be the only attribute that is diminutive.
    // So if there is a "dim" found, that needs to persist.
    let mut dimmed = false;
    for s in tx_arr.iter() {
        match *s {
            "bold" | "underline" | "reverse" | "hide" => {
                attrs = _stylize(Style::Tx(TextStyle::from(*s)), attrs);
                if dimmed {
                    attrs &= !FOREGROUND_INTENSITY
                }
            }
            "dim" => {
                attrs = _stylize(Style::Tx(TextStyle::from(*s)), attrs);
                dimmed = true;
            }
            _ => {
                let at = _stylize(Style::Tx(TextStyle::from(*s)), curr_at);
                attrs = fg_attr | bg_attr | at;
                break;
            }
        }
    }
    // Finally apply the cobined styles.
    unsafe {
        if SetConsoleTextAttribute(handle.0, attrs) == 0 {
            return Err(Error::last_os_error());
        }
    }

    Ok(())
}

pub fn reset(reset: u16) -> Result<()> {
    let handle = Handle::conout()?;
    unsafe {
        if SetConsoleTextAttribute(handle.0, reset) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}


fn _stylize(style: Style, at: u16) -> Attrs {
    let attr: u16;
    let (mask_fg, mask_bg, mask_tx) = (0x000f, 0x00f0, 0xdf00);
    match style {
        Style::Fg(c) => {
            let fg = _match_fg(c);
            if fg == <u16>::max_value() {
                return <u16>::max_value();
            }
            let bg = at & mask_bg;
            let tx = at & mask_tx;
            attr = fg | bg | tx;
        }
        Style::Bg(c) => {
            let bg = _match_bg(c);
            if bg == <u16>::max_value() {
                return <u16>::max_value();
            }
            let fg = at & mask_fg;
            let tx = at & mask_tx;
            attr = fg | bg | tx;
        }
        Style::Tx(t) => {
            let tx = _match_tx(t, at);
            let fg = at & mask_fg;
            let bg = at & mask_bg;
            attr = fg | bg | tx;
        }
    }

    attr
}

fn _match_fg(color: Color) -> Fg {
    // Returns JUST the FOREGROUND attribute.
    match color {
        Color::Black => 0,
        Color::DarkGrey => FG_INTENSITY,
        Color::Red => FG_INTENSITY | FG_RED,
        Color::DarkRed => FG_RED,
        Color::Green => FG_INTENSITY | FG_GREEN,
        Color::DarkGreen => FG_GREEN,
        Color::Yellow => FG_INTENSITY | FG_GREEN | FG_RED,
        Color::DarkYellow => FG_GREEN | FG_RED,
        Color::Blue => FG_INTENSITY | FG_BLUE,
        Color::DarkBlue => FG_BLUE,
        Color::Magenta => FG_INTENSITY | FG_RED | FG_BLUE,
        Color::DarkMagenta => FG_RED | FG_BLUE,
        Color::Cyan => FG_INTENSITY | FG_GREEN | FG_BLUE,
        Color::DarkCyan => FG_GREEN | FG_BLUE,
        Color::White => FG_RED | FG_GREEN | FG_BLUE,
        Color::Grey => FG_INTENSITY | FG_RED | FG_GREEN | FG_BLUE,

        // max_value will signal using `Termios.color` on Windows
        Color::Reset => <u16>::max_value(),

        Color::Rgb { r: _, g: _, b: _ } => <u16>::max_value(), // unsupported
        Color::AnsiValue(_val) => <u16>::max_value(), // unsupported
    }
}

fn _match_bg(color: Color) -> Bg {
    // Returns JUST the BACKGROUND attribute.
    match color {
        Color::Black => 0,
        Color::DarkGrey => BG_INTENSITY,
        Color::Red => BG_INTENSITY | BG_RED,
        Color::DarkRed => BG_RED,
        Color::Green => BG_INTENSITY | BG_GREEN,
        Color::DarkGreen => BG_GREEN,
        Color::Yellow => BG_INTENSITY | BG_GREEN | BG_RED,
        Color::DarkYellow => BG_GREEN | BG_RED,
        Color::Blue => BG_INTENSITY | BG_BLUE,
        Color::DarkBlue => BG_BLUE,
        Color::Magenta => BG_INTENSITY | BG_RED | BG_BLUE,
        Color::DarkMagenta => BG_RED | BG_BLUE,
        Color::Cyan => BG_INTENSITY | BG_GREEN | BG_BLUE,
        Color::DarkCyan => BG_GREEN | BG_BLUE,
        Color::White => BG_INTENSITY | BG_RED | BG_GREEN | BG_BLUE,
        Color::Grey => BG_RED | BG_GREEN | BG_BLUE,

        // max_value will signal using `Termios.color` on Windows
        Color::Reset => <u16>::max_value(),

        Color::Rgb { r: _, g: _, b: _ } => <u16>::max_value(), // unsupported
        Color::AnsiValue(_val) => <u16>::max_value(), // unsupported
    }
}

fn _match_tx(text_style: TextStyle, at: u16) -> Attrs {
    // Returns Fg, Bg, and Tx attributes. Since
    // text styling is additive we will apply and
    // return the existing attributes as a whole.
    match text_style {
        TextStyle::Bold => at | FG_INTENSITY,
        TextStyle::Dim => at & !FG_INTENSITY,
        TextStyle::Underline => at | COMMON_LVB_UNDERSCORE,
        TextStyle::Reverse => at | COMMON_LVB_REVERSE_VIDEO,
        TextStyle::Hide => {
            // Get the BG color.
            let (mask_fg, mask_bg) = (0x000f, 0x00f0);
            let bg = at & mask_bg;
            // FOREGROUND and BACKGROUND color differ by 4 bits;
            // to convert from 0x0020 (BG Green) to 0x0002 (FG Green),
            // shift right 4 bits. By making the FOREGROUND color the
            // same as the BACKGROUND color, effectively you hide the
            // printed content.
            let fg = bg >> 4;
            // Since we identified the new FOREGROUND, we include it
            // and remove it from the current attributes. The BACK-
            // GROUND should remain the same within the current attrs.
            fg | (at & !mask_fg)
        },
        TextStyle::Reset => {
            let mask_tx = 0xdf00;
            // Since Windows Attributes are "additive", we can simply
            // unmask all of them if Attribute::Reset.
            (at & !mask_tx)
        },
    }
}