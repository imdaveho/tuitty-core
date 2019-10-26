// Unix specific functions that parse ANSI escape sequences from the stdin
// bytestream and map to the proper input event.

use crate::common::enums::{
    InputEvent::{*, self},
    KeyEvent::*, MouseEvent::*,
    MouseButton, StoreEvent::*,
};


// Reference: redox-os/termion/blob/master/src/event.rs
pub fn parse_event<I>(item: u8, iter: &mut I) -> InputEvent
where I: Iterator<Item = u8> {
    let input_event = match item {
        // Match: ESC character
        b'\x1B' => {
            let a = iter.next();
            match a {
                Some(b'O') => {
                    match iter.next() {
                        // F1-F4
                        Some(val @ b'P'..=b'S') => {
                            Keyboard(F(1 + val - b'P'))
                        },
                        _ => Unsupported,
                    }
                }
                Some(b'[') => parse_csi(iter),
                Some(b'\x1B') => Keyboard(Esc),
                Some(c) => match parse_utf8_char(c, iter) {
                        Some(ch) => Keyboard(Alt(ch)),
                        None => Unsupported,
                },
                // Without anything following \x1B, this is
                // simply a press of the ESC key
                None => Keyboard(Esc),
            }
        },
        // Match: Newline OR Carriage Return
        b'\r' | b'\n' => Keyboard(Enter),
        // Match: TAB
        b'\t' => Keyboard(Tab),
        // Match: BACKSPACE
        b'\x7F' => Keyboard(Backspace),
        // Match: ???
        c @ b'\x01'..=b'\x1A' => {
            Keyboard(Ctrl(
                (c as u8 - 0x1 + b'a') as char))
        }
        // Match: ???
        c @ b'\x1C'..=b'\x1F' => {
            Keyboard(Ctrl(
                (c as u8 - 0x1C + b'4') as char))
        }
        // Match: Null
        b'\0' => Keyboard(Null),
        // Match: char
        c => match parse_utf8_char(c, iter) {
                Some(ch) => Keyboard(Char(ch)),
                None => Unsupported,
        }
    };
    return input_event;
}

fn parse_csi<I>(iter: &mut I) -> InputEvent
where I: Iterator<Item = u8> {
    let input_event = match iter.next() {
        Some(b'[') => match iter.next() {
            // NOTE (@imdaveho): cannot find when this occurs;
            // having another '[' after ESC[ not a likely scenario
            Some(val @ b'A'..=b'E') => {
                Keyboard(F(1 + val - b'A'))
            }
            _ => Unsupported,
        },
        Some(b'D') => Keyboard(Left),
        Some(b'C') => Keyboard(Right),
        Some(b'A') => Keyboard(Up),
        Some(b'B') => Keyboard(Down),
        Some(b'H') => Keyboard(Home),
        Some(b'F') => Keyboard(End),
        Some(b'Z') => Keyboard(BackTab),
        // Match: X10 mouse encoding
        Some(b'M') => {
            // X10 emulation mouse encoding:
            // ESC [ CB Cx Cy (6 characters only).
            // (imdaveho) NOTE: cannot find documentation on this
            // let mut next = || iter.next().unwrap();

            // let cb = *next() as i8 -32;
            let cb = match iter.next() {
                Some(n) => n as i8 - 32,
                None => 4
            };
            // (1, 1) are the coords for upper left.
            // Subtract 1 to keep it synced with cursor
            // let cx = next().saturating_sub(32) as i16 - 1;
            let cx = match iter.next() {
                Some(n) => n.saturating_sub(32) as i16 - 1,
                None => return Unsupported,
            };
            // let cy = next().saturating_sub(32) as i16 - 1;
            let cy = match iter.next() {
                Some(n) => n.saturating_sub(32) as i16 - 1,
                None => return Unsupported,
            };

            Mouse(match cb & 0b11 {
                0 => {
                    if cb & 0x40 != 0 {
                        Press(MouseButton::WheelUp, cx, cy)
                    } else {
                        Press(MouseButton::Left, cx, cy)
                    }
                }
                1 => {
                    if cb & 0x40 != 0 {
                        Press(MouseButton::WheelDown, cx, cy)
                    } else {
                        Press(MouseButton::Middle, cx, cy)
                    }
                }
                2 => Press(MouseButton::Right, cx, cy),
                3 => Release(cx, cy),
                _ => return Unsupported,
            })
        },
        // Match: xterm mouse handling
        // ESC [ < Cb ; Cx ; Cy (;) (M or m)
        Some(b'<') => {
            let mut buf = Vec::new();
            let mut c = match iter.next() {
                Some(ch) => ch,
                None => return Unsupported,
            };
            while match c {
                b'm' | b'M' => false,
                _ => true,
            } {
                buf.push(c);
                c = match iter.next() {
                    Some(ch) => ch,
                    None => return Unsupported,
                }
            }
            let (cb, cx, cy): (i16, i16, i16);
            if let Ok(str_buf) = String::from_utf8(buf) {
                let nums = &mut str_buf.split(';');
                cb = nums.next().unwrap_or("4")
                    .parse().unwrap_or(4);
                // (1, 1) are the coords for upper left.
                // Subtract 1 to keep it synced with cursor
                cx = nums.next().unwrap_or("1")
                    .parse().unwrap_or(1) - 1;
                cy = nums.next().unwrap_or("1")
                    .parse().unwrap_or(1) - 1;
            } else { return Unsupported }

            let event = match cb {
                0..=2 | 64..=65 => {
                    let btn = match cb {
                        0 => MouseButton::Left,
                        1 => MouseButton::Middle,
                        2 => MouseButton::Right,
                        64 => MouseButton::WheelUp,
                        65 => MouseButton::WheelDown,
                        _ => return Unsupported,
                    };
                    match c {
                        b'M' => Press(btn, cx, cy),
                        b'm' => Release(cx, cy),
                        _ => return Unsupported,
                    }
                }
                32 => Hold(cx, cy),
                3 => Release(cx, cy),
                _ => return Unsupported,
            };
            Mouse(event)
        },
        // Match: Numbered escape code.
        Some(c @ b'0'..=b'9') => {
            let mut buf = Vec::new();
            buf.push(c);
            let mut character = match iter.next() {
                Some(ch) => ch,
                None => return Unsupported,
            };

            // The final byte of a CSI sequence can be in the range 64-126, so
            // let's keep reading anything else.
            while character < 64 || character > 126 {
                buf.push(character);
                character = match iter.next() {
                    Some(ch) => ch,
                    None => return Unsupported,
                }
            }

            match character {
                b'M' => parse_csi_rxvt_mouse(buf),
                b'~' => parse_csi_special_key_code(buf),
                b'R' => parse_csi_cursor_position(buf),
                c => parse_csi_modified_arrow_keys(buf, c),
            }
        } // End Numbered escape code.
        _ => Unsupported,
    };

    return input_event;
}

fn parse_utf8_char<I>(c: u8, iter: &mut I) -> Option<char>
where I: Iterator<Item = u8> {
    if c.is_ascii() {
        Some(c as char)
    } else {
        let mut bytes: Vec<u8> = Vec::with_capacity(4);
        bytes.push(c);

        while let Some(ch) = iter.next() {
            bytes.push(ch);
            if let Ok(s) = std::str::from_utf8(&bytes) {
                // return Ok(st.chars().next().unwrap());
                match s.parse() {
                    Ok(chr) => return Some(chr),
                    Err(_) => return None,
                }
            }
            if bytes.len() >= 4 { break }
        }
        return None;
    }
}

fn parse_csi_rxvt_mouse(buf: Vec<u8>) -> InputEvent {
    // rxvt mouse encoding:
    // ESC [ Cb ; Cx ; Cy ; M
    let str_buf = String::from_utf8(buf).unwrap();

    let nums: Vec<i16> = str_buf
        .split(';')
        .map(|n| n.parse().unwrap())
        .collect();

    let cb = nums[0];
    let cx = nums[1];
    let cy = nums[2];

    let event = match cb {
        32 => Press(MouseButton::Left, cx, cy),
        33 => Press(MouseButton::Middle, cx, cy),
        34 => Press(MouseButton::Right, cx, cy),
        35 => Release(cx, cy),
        64 => Hold(cx, cy),
        96 | 97 => Press(MouseButton::WheelUp, cx, cy),
        _ => return Unsupported,
    };
    Mouse(event)
}

fn parse_csi_special_key_code(buf: Vec<u8>) -> InputEvent {
    let str_buf = String::from_utf8(buf).unwrap();

    // This CSI sequence can be a list of
    // semicolon-separated numbers.
    let nums: Vec<u8> = str_buf
        .split(';')
        .map(|n| n.parse().unwrap())
        .collect();

    if nums.is_empty() { return Unsupported }

    // (redux-os) TODO: handle multiple values for key
    // modifiers (ex: values [3, 2] means Shift+Delete)
    if nums.len() > 1 { return Unsupported }

    match nums[0] {
        1 | 7 => Keyboard(Home),
        2 => Keyboard(Insert),
        3 => Keyboard(Delete),
        4 | 8 => Keyboard(End),
        5 => Keyboard(PageUp),
        6 => Keyboard(PageDown),
        v @ 11..=15 => {
            Keyboard(F(v - 10))
        }
        v @ 17..=21 => {
            Keyboard(F(v - 11))
        }
        v @ 23..=24 => {
            Keyboard(F(v - 12))
        }
        _ => Unsupported,
    }
}

fn parse_csi_modified_arrow_keys(buf: Vec<u8>, key: u8) -> InputEvent {
    let modifier = buf.last().unwrap_or(&0);

    match (*modifier, key) {
        (53, 65) => Keyboard(CtrlUp),
        (53, 66) => Keyboard(CtrlDown),
        (53, 67) => Keyboard(CtrlRight),
        (53, 68) => Keyboard(CtrlLeft),
        (50, 65) => Keyboard(ShiftUp),
        (50, 66) => Keyboard(ShiftDown),
        (50, 67) => Keyboard(ShiftRight),
        (50, 68) => Keyboard(ShiftLeft),
        _ => Unsupported,
    }
}

fn parse_csi_cursor_position(buf: Vec<u8>) -> InputEvent {
    // ESC [ Cy ; Cx R
    // Cy - cursor row number (starting from 1)
    // Cx - cursor column number (starting from 1)
    let str_buf = String::from_utf8(buf).unwrap();

    let nums: Vec<i16> = str_buf
        .split(';')
        .map(|n| n.parse().unwrap())
        .collect();

    let row = nums[0] - 1;
    let col = nums[1] - 1;

    Dispatch(Pos(col, row))
}
