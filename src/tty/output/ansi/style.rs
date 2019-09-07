// ANSI specific functions to style text output to the terminal.

use super::{csi, Color, Format, Style};


pub fn set_fg(color: Color) -> String {
    _stylize(Style::Fg(color))
}

pub fn set_bg(color: Color) -> String {
    _stylize(Style::Bg(color))
}

pub fn set_fmt(format: Format) -> String {
    _stylize(Style::Fmt(format))
}

pub fn set_all(fg: &str, bg: &str, fmts: &str) -> String {
    let fg_str = _stylize(Style::Fg(Color::from(fg)));
    let bg_str = _stylize(Style::Bg(Color::from(bg)));

    // The tx param is should be a comma separated string.
    // TODO: lowercase()
    let fmt_arr: Vec<&str> = fmts.split(',').map(|t| t.trim()).collect();
    let mut dimmed = false;
    let mut fmt_str = String::new();
    for s in fmt_arr.iter() {
        match *s {
            "bold" => {
                if dimmed { continue }
                fmt_str.push_str(&_stylize(Style::Fmt(Format::from(*s))));
            },
            "dim" => {
                dimmed = true;
                fmt_str.push_str(&_stylize(Style::Fmt(Format::from(*s))));
            },
            "reset" => {
                fmt_str.push_str(&_stylize(Style::Fmt(Format::from(*s))));
                break
            },
            _ => {
                fmt_str.push_str(&_stylize(Style::Fmt(Format::from(*s))));
            },
        }
    }

    format!("{}{}{}", fg_str, bg_str, fmt_str)
}

pub fn reset() -> String {
    csi!("0m").to_string()
}


fn _stylize(style: Style) -> String {
    let mut ansi_value = String::new();

    let color: Color;

    match style {
        Style::Fg(c) => {
            if c == Color::Reset {
                ansi_value.push_str("39");
                return format!(csi!("{}m"), ansi_value);
            } else {
                ansi_value.push_str("38;");
                color = c;
            }
        }
        Style::Bg(c) => {
            if c == Color::Reset {
                ansi_value.push_str("49");
                return format!(csi!("{}m"), ansi_value);
            } else {
                ansi_value.push_str("48;");
                color = c;
            }
        }
        Style::Fmt(fmt) => {
            ansi_value.push_str(&fmt.to_string());
            return format!(csi!("{}m"), ansi_value);
        }
    }

    let rgb_val: String;

    let color_val = match color {
        Color::Black => "5;0",
        Color::DarkGrey => "5;8",
        Color::Red => "5;9",
        Color::DarkRed => "5;1",
        Color::Green => "5;10",
        Color::DarkGreen => "5;2",
        Color::Yellow => "5;11",
        Color::DarkYellow => "5;3",
        Color::Blue => "5;12",
        Color::DarkBlue => "5;4",
        Color::Magenta => "5;13",
        Color::DarkMagenta => "5;5",
        Color::Cyan => "5;14",
        Color::DarkCyan => "5;6",
        Color::White => "5;15",
        Color::Grey => "5;7",
        Color::Rgb { r, g, b } => {
            rgb_val = format!("2;{};{};{}", r, g, b);
            rgb_val.as_str()
        }
        Color::AnsiValue(val) => {
            rgb_val = format!("5;{}", val);
            rgb_val.as_str()
        }
        _ => "",
    };

    ansi_value.push_str(color_val);
    format!(csi!("{}m"), ansi_value)
}
