// ANSI specific functions to style text output to the terminal.

use super::{csi, Color, Style, TextStyle};


pub fn set_fg(fg_color: Color) -> String {
    format!(
        csi!("{}m"),
        _stylize(Style::Fg(fg_color)),
    )
}

pub fn set_bg(bg_color: Color) -> String {
    format!(
        csi!("{}m"),
        _stylize(Style::Bg(bg_color)),
    )
}

pub fn set_tx(text_style: TextStyle) -> String {
    format!(
        csi!("{}m"),
        _stylize(Style::Tx(text_style)),
    )
}

pub fn set_all(fg: &str, bg: &str, tx: &str) -> String {
    let fg_str = _stylize(Style::Fg(Color::from(fg)));
    let bg_str = _stylize(Style::Bg(Color::from(bg)));

    // The tx param is should be a comma separated string.
    let tx_arr: Vec<&str> = tx.split(',').map(|t| t.trim()).collect();
    let mut dimmed = false;
    let mut tx_str = String::new();
    for s in tx_arr.iter() {
        match *s {
            "bold" => {
                if !dimmed {
                    tx_str.push_str(
                    format!(
                        csi!("{}m"),
                        _stylize(Style::Tx(TextStyle::from(*s)))
                    ).as_str())
                }
            },
            "dim" => {
                tx_str.push_str(
                    format!(
                        csi!("{}m"),
                        _stylize(Style::Tx(TextStyle::from(*s)))
                ).as_str());
                dimmed = true
            },
            "underline" | "reverse" | "hide" => {
                tx_str.push_str(
                    format!(
                        csi!("{}m"),
                        _stylize(Style::Tx(TextStyle::from(*s)))
                ).as_str())
            },
            "" => tx_str.push_str("m"),
            _ => {
                tx_str.push_str(
                    format!(
                        csi!("{}m"),
                        _stylize(Style::Tx(TextStyle::from(*s)))
                ).as_str());
                break
            }
        }
    }

    format!(csi!("{}{}{}"), fg_str, bg_str, tx_str)
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
                ansi_value.push_str("39;");
                return ansi_value;
            } else {
                ansi_value.push_str("38;");
                color = c;
            }
        }
        Style::Bg(c) => {
            if c == Color::Reset {
                ansi_value.push_str("49;");
                return ansi_value;
            } else {
                ansi_value.push_str("48;");
                color = c;
            }
        }
        Style::Tx(t) => {
            ansi_value.push_str(&t.to_string());
            return ansi_value;
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
    ansi_value
}
