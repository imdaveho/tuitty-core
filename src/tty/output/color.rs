//! TODO:

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
    Rgb{r: u8, g: u8, b: u8},
    AnsiValue(u8),
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

#[cfg(windows)]
pub struct Foreground(pub u16);
#[cfg(windows)]
pub struct Background(pub u16);
#[cfg(windows)]
pub const RESET: u16 = 0xFFFF;
#[cfg(windows)]
pub const IGNORE: u16 = 0xFFF0;

#[cfg(windows)]
impl From<Color> for Foreground {
    fn from(src: Color) -> Self {
        use winapi::um::wincon::{
            FOREGROUND_RED as RED,
            FOREGROUND_GREEN as GREEN,
            FOREGROUND_BLUE as BLUE,
            FOREGROUND_INTENSITY as INTENSE,
        };

        match src {
            Color::Black => Self(0),
            Color::DarkGrey => Self(INTENSE),
            Color::Red => Self(RED | INTENSE),
            Color::DarkRed => Self(GREEN),
            Color::Green => Self(GREEN | INTENSE),
            Color::DarkGreen => Self(GREEN),
            Color::Yellow => Self(RED | GREEN | INTENSE),
            Color::DarkYellow => Self(RED | GREEN),
            Color::Blue => Self(BLUE | INTENSE),
            Color::DarkBlue => Self(BLUE),
            Color::Magenta => Self(RED | BLUE | INTENSE),
            Color::DarkMagenta => Self(RED | BLUE),
            Color::Cyan => Self(GREEN | BLUE | INTENSE),
            Color::DarkCyan => Self(GREEN | BLUE),
            Color::White => Self(RED | GREEN | BLUE),
            Color::Grey => Self(RED | GREEN | BLUE | INTENSE),
            Color::Reset => Self(RESET),
            Color::Rgb{r, g, b} => Self(IGNORE),
            Color::AnsiValue(_) => Self(IGNORE),
        }
    }
}

#[cfg(windows)]
impl PartialEq<u16> for Foreground {
    fn eq(&self, other: &u16) -> bool {
        self.0 == (*other)
    }
}

#[cfg(windows)]
impl From<Color> for Background {
    fn from(src: Color) -> Self {
        use winapi::um::wincon::{
            BACKGROUND_RED as RED,
            BACKGROUND_GREEN as GREEN,
            BACKGROUND_BLUE as BLUE,
            BACKGROUND_INTENSITY as INTENSE,
        };

        match src {
            Color::Black => Self(0),
            Color::DarkGrey => Self(INTENSE),
            Color::Red => Self(RED | INTENSE),
            Color::DarkRed => Self(GREEN),
            Color::Green => Self(GREEN | INTENSE),
            Color::DarkGreen => Self(GREEN),
            Color::Yellow => Self(RED | GREEN | INTENSE),
            Color::DarkYellow => Self(RED | GREEN),
            Color::Blue => Self(BLUE | INTENSE),
            Color::DarkBlue => Self(BLUE),
            Color::Magenta => Self(RED | BLUE | INTENSE),
            Color::DarkMagenta => Self(RED | BLUE),
            Color::Cyan => Self(GREEN | BLUE | INTENSE),
            Color::DarkCyan => Self(GREEN | BLUE),
            Color::White => Self(RED | GREEN | BLUE),
            Color::Grey => Self(RED | GREEN | BLUE | INTENSE),
            Color::Reset => Self(RESET),
            Color::Rgb{r, g, b} => Self(IGNORE),
            Color::AnsiValue(_) => Self(IGNORE),
        }
    }
}

#[cfg(windows)]
impl PartialEq<u16> for Background {
    fn eq(&self, other: &u16) -> bool {
        self.0 == (*other)
    }
}