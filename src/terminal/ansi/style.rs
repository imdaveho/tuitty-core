// Ansi specific functions to colorize and format text in the terminal.

use crate::common::enums::{Style, Color, Effect::*};


pub fn reset() -> String {
    "\x1B[0m".to_string()
}

pub fn set_style(style: Style) -> String {
    match style {
        Style::Fg(c) => {
            if c == Color::Reset {
                return format!("\x1B[{}m", "39");
            }
            format!("\x1B[38;{}m", String::from(c))
        }
        Style::Bg(c) => {
            if c == Color::Reset {
                return format!("\x1B[{}m", "49");
            }
            format!("\x1B[48;{}m", String::from(c))
        }
        Style::Fx(f) => {
            let fxs = [Reset, Bold, Dim, Underline, Reverse, Hide];
            let mut csi = String::with_capacity(12);
            for fx in &fxs {
                if (f & *fx as u32) != 0 {
                    let value = (*fx as u32 >> 9).trailing_zeros() as u8;
                    csi.push_str(&format!("\x1B[{}m"), value)
                } else {
                    csi.push_str("");
                }
            }
            csi
        }
    }
}

pub fn set_styles(fg: Color, bg: Color, fx: u32) -> String {
    format!(
        "{fg}{bg}{fx}", 
        fg=set_style(Style::Fg(fg)),
        bg=set_style(Style::Bg(bg)),
        fx=set_style(Style::Fx(fx)))
}


impl From<Color> for String {
    fn from(src: Color) -> Self {
        match src {
            Color::Black => String::from("5;0"),
            Color::DarkGrey => String::from("5;8"),
            Color::Red => String::from("5;9"),
            Color::DarkRed => String::from("5;1"),
            Color::Green => String::from("5;10"),
            Color::DarkGreen => String::from("5;2"),
            Color::Yellow => String::from("5;11"),
            Color::DarkYellow => String::from("5;3"),
            Color::Blue => String::from("5;12"),
            Color::DarkBlue => String::from("5;4"),
            Color::Magenta => String::from("5;13"),
            Color::DarkMagenta => String::from("5;5"),
            Color::Cyan => String::from("5;14"),
            Color::DarkCyan => String::from("5;6"),
            Color::White => String::from("5;15"),
            Color::Grey => String::from("5;7"),
            Color::Rgb { r, g, b } => {
                String::from(&format!("2;{};{};{}", r, g, b))
            },
            Color::AnsiValue(val) => {
                String::from(&format!("5;{}", val))
            }
            Color::Reset => String::from(""),
        }
    }
}