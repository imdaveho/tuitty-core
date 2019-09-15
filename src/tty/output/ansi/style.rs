// ANSI specific functions to style text output to the terminal.

use super::{
    csi,
    Style, Style::*,
    Color, Effect::*,
};

pub fn reset() -> String {
    csi!("0m").to_string()
}

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
            let fxs = [Reset, Bold, Dim, Underline, Reverse, Hide];
            let mut fxcsi = String::with_capacity(12);
            for effect in &fxs {
                if (f & *effect as u32) != 0 {
                    let value = (*effect as u32 >> 9).trailing_zeros() as u8;
                    fxcsi.push_str(&format!(csi!("{}m"), value))
                } else {
                    fxcsi.push_str("");
                }
            }
            fxcsi
        }
    }
}

pub fn set_styles(fgcol: Color, bgcol: Color, effects: u32) -> String {
    let fg_str = set_style(Fg(fgcol));
    let bg_str = set_style(Bg(bgcol));
    let fx_str = set_style(Fx(effects));
    format!("{}{}{}", fg_str, bg_str, fx_str)
}
