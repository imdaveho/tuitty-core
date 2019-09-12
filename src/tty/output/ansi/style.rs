// ANSI specific functions to style text output to the terminal.

use self::Effect::*;
use super::{csi, Style, Style::*};

pub fn set_style(style: Style) -> String {
    match style {
        Fg(c) => {
            if c == Color::Reset {
                return format!(csi!("{}m"), "39");
            }
            format!(csi!("38;{}m"), String::from(c))
        }
        Bg(c) => {
            if c == Color::Reset {
                return format!(csi!("{}m"), "49");
            }
            format!(csi!("48;{}m"), String::from(c))
        }
        Fx(f) => {
            // TODO: run through values like in cells.rs and output single
            // string with all the sequences.
            let masks = [Reset, Bold, Dim, Underline, Reverse, Hide];
            let mut fx_str = String::with_capacity(12);
            for m in &masks {
                if (f & *m as u32) != 0 {
                    let value = (*m as u32 >> 9).trailing_zeros() as u8;
                    fx_str.push_str(&format!(csi!("{}m"), value))
                } else {
                    fx_str.push_str("");
                }
            }
            fx_str
        }
    }
}

pub fn set_styles(fgcol: Color, bgcol: Color, effects: Effects) -> String {
    let fg_str = set_style(Fg(fgcol));
    let bg_str = set_style(Bg(bgcol));
    let fx_str = set_style(Fx(effects));
    format!("{}{}{}", fg_str, bg_str, fx_str)
}

pub fn reset() -> String {
    csi!("0m").to_string()
}


pub type Effects = u32;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Reset,

    Black,
    DarkGrey,

    Red,
    DarkRed,

    Green,
    DarkGreen,

    Yellow,
    DarkYellow,

    Blue,
    DarkBlue,

    Magenta,
    DarkMagenta,

    Cyan,
    DarkCyan,

    White,
    Grey,

    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },

    AnsiValue(u8),
}


impl From<Color> for String {
    fn from(src: Color) -> Self {
        let mut ansi_str = String::with_capacity(7);
        match src {
            Color::Black => ansi_str.push_str("5;0"),
            Color::DarkGrey => ansi_str.push_str("5;8"),
            Color::Red => ansi_str.push_str("5;9"),
            Color::DarkRed => ansi_str.push_str("5;1"),
            Color::Green => ansi_str.push_str("5;10"),
            Color::DarkGreen => ansi_str.push_str("5;2"),
            Color::Yellow => ansi_str.push_str("5;11"),
            Color::DarkYellow => ansi_str.push_str("5;3"),
            Color::Blue => ansi_str.push_str("5;12"),
            Color::DarkBlue => ansi_str.push_str("5;4"),
            Color::Magenta => ansi_str.push_str("5;13"),
            Color::DarkMagenta => ansi_str.push_str("5;5"),
            Color::Cyan => ansi_str.push_str("5;14"),
            Color::DarkCyan => ansi_str.push_str("5;6"),
            Color::White => ansi_str.push_str("5;15"),
            Color::Grey => ansi_str.push_str("5;7"),
            Color::Rgb { r, g, b } => {
                ansi_str.push_str(&format!("2;{};{};{}", r, g, b))
            }
            Color::AnsiValue(val) => {
                ansi_str.push_str(&format!("5;{}", val))
            }
            _ => ansi_str.push_str(""),
        };
        ansi_str
    }
}


#[derive(Clone, Copy, PartialEq)]
pub enum Effect {
    Reset = 1 << (0 + 9),
    Bold = 1 << (1 + 9),
    Dim = 1 << (2 + 9),
    Underline = 1 << (4 + 9),
    Reverse = 1 << (7 + 9),
    Hide = 1 << (8 + 9),
}
