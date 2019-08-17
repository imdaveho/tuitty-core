//! Implements platform specific functions to style text output to the terminal.
use super::{
    Color, Error, Result, Style,
    Handle, ConsoleInfo, TextStyle,
};
use winapi::um::wincon::{
    SetConsoleTextAttribute,
    FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_INTENSITY,
    BACKGROUND_RED, BACKGROUND_GREEN, BACKGROUND_BLUE, BACKGROUND_INTENSITY,
    COMMON_LVB_UNDERSCORE, COMMON_LVB_REVERSE_VIDEO,
};

type Fg = u16;
type Bg = u16;
type Attrs = u16;


fn stylize(style: Style, at: u16) -> Attrs {
    let attr: u16;
    let (mask_fg, mask_bg, mask_tx) = (0x000f, 0x00f0, 0xdf00);
    match style {
        Style::Fg(c) => {
            let fg = match_fg(c);
            if fg == <u16>::max_value() {
                return <u16>::max_value();
            }
            let bg = at & mask_bg;
            let tx = at & mask_tx;
            attr = fg | bg | tx;
        }
        Style::Bg(c) => {
            let bg = match_bg(c);
            if bg == <u16>::max_value() {
                return <u16>::max_value();
            }
            let fg = at & mask_fg;
            let tx = at & mask_tx;
            attr = fg | bg | tx;
        }
        Style::Tx(t) => {
            let tx = match_tx(t, at);
            let fg = at & mask_fg;
            let bg = at & mask_bg;
            attr = fg | bg | tx;
        }
    }
    
    attr
}

fn match_fg(color: Color) -> Fg {
    // Returns JUST the FOREGROUND attribute.
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

fn match_bg(color: Color) -> Bg {
    // Returns JUST the BACKGROUND attribute.
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

fn match_tx(text_style: TextStyle, at: u16) -> Attrs {
    // Returns Fg, Bg, and Tx attributes. Since
    // text styling is additive we will apply and
    // return the existing attributes as a whole.
    match text_style {
        TextStyle::Bold => at | FOREGROUND_INTENSITY, 
        TextStyle::Dim => at & !FOREGROUND_INTENSITY,
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

pub fn _set_fg(color: Color, orig: u16) -> Result<()> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;
    let curr_at = info.attributes();
    let mut attr: Attrs = stylize(Style::Fg(color), curr_at);
    if attr ==  <u16>::max_value() {
        // Reset Fg from Original
        attr = (orig & 0x000f)
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

pub fn _set_bg(color: Color, orig: u16) -> Result<()> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;
    let curr_at = info.attributes();
    let mut attr: Attrs = stylize(Style::Bg(color), curr_at);
    if attr == <u16>::max_value() {
        // Reset Fg from Original
        attr = (curr_at & 0x000f)
        | (orig & 0x00f0)
        | (curr_at & 0xdf00);
    }
    unsafe {
        if SetConsoleTextAttribute(handle.0, attr) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn _set_tx(text_style: TextStyle) -> Result<()> { 
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;
    let curr_at = info.attributes();
    let attr: Attrs = stylize(Style::Tx(text_style), curr_at);
    unsafe {
        if SetConsoleTextAttribute(handle.0, attr) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn _set_all(fg: &str, bg: &str, tx: &str, orig: u16) -> Result<()> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;
    let curr_at = info.attributes();
    // Start with getting only the Fg Attributes.
    let (fg_attr, bg_attr, mut attrs): (u16, u16, u16);
    match fg {
        "reset" => fg_attr = orig & 0x000f,
        _ => {
            let mask_fg = 0x000f;
            let attr: Fg = stylize(Style::Fg(Color::from(fg)), curr_at);
            if attr == <u16>::max_value() {
                // Return existing Fg w/o changes.
                fg_attr = curr_at & mask_fg
            } else { fg_attr = attr & mask_fg }
        }
    }
    // Then getting only the Bg Attributes.
    match bg {
        "reset" => bg_attr = orig & 0x00f0,
        _ => {
            let mask_bg = 0x00f0;
            let attr: Bg = stylize(Style::Bg(Color::from(bg)), curr_at);
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
                attrs = stylize(Style::Tx(TextStyle::from(*s)), attrs);
                if dimmed {
                    attrs &= !FOREGROUND_INTENSITY
                }
            }
            "dim" => {
                attrs = stylize(Style::Tx(TextStyle::from(*s)), attrs);
                dimmed = true;
            }
            _ => {
                let at = stylize(Style::Tx(TextStyle::from(*s)), curr_at);
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