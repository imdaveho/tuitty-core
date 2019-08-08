//! Implements platform specific functions to style text output to the terminal.
use crate::{csi, write_cout};
use super::{Color, Style, Attribute, TtyResult, Write};


fn stylize(style: Style) -> String {
    let mut ansi_value = String::new();

    let color: Color;

    match style {
        Style::Fg(_color) => {
            if _color == Color::Reset {
                ansi_value.push_str("39");
                return ansi_value;
            } else {
                ansi_value.push_str("38;");
                color = _color;
            }
        }
        Style::Bg(_color) => {
            if _color == Color::Reset {
                ansi_value.push_str("49");
                return ansi_value;
            } else {
                ansi_value.push_str("48;");
                color = _color;
            }
        }
        Style::Attr(_attr) => {
            ansi_value.push_str(_attr.to_string().as_str());
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

pub fn _set_fg(fg_color: Color) -> TtyResult<()> {
    write_cout!(&format!(
        csi!("{}m"),
        stylize(Style::Fg(fg_color)),
    ))?;
    Ok(())
}

pub fn _set_bg(bg_color: Color) -> TtyResult<()> {
    write_cout!(&format!(
        csi!("{}m"),
        stylize(Style::Bg(bg_color)),
    ))?;
    Ok(())
}

pub fn _set_attr(attr: Attribute) -> TtyResult<()> {
    write_cout!(&format!(
        csi!("{}m"),
        stylize(Style::Attr(attr)),
    ))?;
    Ok(())
}

pub fn _reset() -> TtyResult<()> {
    write_cout!(csi!("0m"))?;
    Ok(())
}
