// Unix specific functions that parse ANSI escape sequences from the stdin
// bytestream and map to the proper input event.

use std::io::{ Error, ErrorKind, Result };
use crate::common::enums::{InputEvent, MouseEvent, MouseButton, KeyEvent};


// Reference: redox-os/termion/blob/master/src/event.rs
pub fn parse_event<I>(item: u8, iter: &mut I) -> Result<InputEvent>
where I: Iterator<Item = u8> {
    let error = Error::new(
        ErrorKind::Other,
        "Could not parse an input event",
    );
    let input_event = match item {
        // Match: ESC character
        b'\x1B' => {
            let a = iter.next();
            match a {
                Some(b'O') => {
                    match iter.next() {
                        // F1-F4
                        Some(val @ b'P'..=b'S') => {
                            InputEvent::Keyboard(KeyEvent::F(1 + val - b'P'))
                        }
                        _ => return Err(error),
                    }
                }
                Some(b'[') => {
                    // CSI sequence
                    parse_csi(iter)
                }
                Some(b'\x1B') => InputEvent::Keyboard(KeyEvent::Esc),
                Some(c) => {
                    // since each element is u8 and
                    // char is u32, confirm valid utf8
                    let ch = parse_utf8_char(c, iter);
                    InputEvent::Keyboard(KeyEvent::Alt(ch?))
                }
                // Without anything following \x1B, this is
                // simply a press of the ESC key
                None => InputEvent::Keyboard(KeyEvent::Esc),
            }
        } // End: ESC character
        // Match: Newline OR Carriage Return
        b'\n' | b'\r' => InputEvent::Keyboard(KeyEvent::Char('\n')),
        // Match: TAB
        b'\t' => InputEvent::Keyboard(KeyEvent::Char('\t')),
        // Match: BACKSPACE
        b'\x7F' => InputEvent::Keyboard(KeyEvent::Backspace),
        // Match: ???
        c @ b'\x01'..=b'\x1A' => {
            InputEvent::Keyboard(KeyEvent::Ctrl(
                (c as u8 - 0x1 + b'a') as char))
        }
        // Match: ???
        c @ b'\x1C'..=b'\x1F' => {
            InputEvent::Keyboard(KeyEvent::Ctrl(
                (c as u8 - 0x1C + b'4') as char))
        }
        // Match: Null
        b'\0' => InputEvent::Keyboard(KeyEvent::Null),
        // Match: char
        c => {
            // since each element is u8 and
            // char is u32, confirm valid utf8
            let ch = parse_utf8_char(c, iter);
            InputEvent::Keyboard(KeyEvent::Char(ch?))
        }
    };
    return Ok(input_event);
}

fn parse_csi<I>(iter: &mut I) -> InputEvent
where I: Iterator<Item = u8> {
    match iter.next() {
        Some(b'[') => match iter.next() {
            // NOTE (@imdaveho): cannot find when this occurs;
            // having another '[' after ESC[ not a likely scenario
            Some(val @ b'A'..=b'E') => {
                InputEvent::Keyboard(KeyEvent::F(1 + val - b'A'))
            }
            _ => InputEvent::Unknown,
        },
        Some(b'D') => InputEvent::Keyboard(KeyEvent::Left),
        Some(b'C') => InputEvent::Keyboard(KeyEvent::Right),
        Some(b'A') => InputEvent::Keyboard(KeyEvent::Up),
        Some(b'B') => InputEvent::Keyboard(KeyEvent::Dn),
        Some(b'H') => InputEvent::Keyboard(KeyEvent::Home),
        Some(b'F') => InputEvent::Keyboard(KeyEvent::End),
        Some(b'Z') => InputEvent::Keyboard(KeyEvent::BackTab),
        // Match: X10 mouse encoding
        Some(b'M') => {
            // X10 emulation mouse encoding:
            // ESC [ CB Cx Cy (6 characters only).
            // (imdaveho) NOTE: cannot find documentation on this
            let mut next = || iter.next().unwrap();

            let cb = next() as i8 - 32;
            // (1, 1) are the coords for upper left.
            let cx = next().saturating_sub(32) as i16;
            let cy = next().saturating_sub(32) as i16;

            InputEvent::Mouse(match cb & 0b11 {
                0 => {
                    if cb & 0x40 != 0 {
                        MouseEvent::Press(MouseButton::WheelUp, cx, cy)
                    } else {
                        MouseEvent::Press(MouseButton::Left, cx, cy)
                    }
                }
                1 => {
                    if cb & 0x40 != 0 {
                        MouseEvent::Press(MouseButton::WheelDn, cx, cy)
                    } else {
                        MouseEvent::Press(MouseButton::Middle, cx, cy)
                    }
                }
                2 => MouseEvent::Press(MouseButton::Right, cx, cy),
                3 => MouseEvent::Release(cx, cy),
                _ => MouseEvent::Unknown,
            })
        } // End X10 mouse encoding
        // Match: xterm mouse handling
        // ESC [ < Cb ; Cx ; Cy (;) (M or m)
        Some(b'<') => {
            let mut buf = Vec::new();
            let mut c = iter.next().unwrap();
            while match c {
                b'm' | b'M' => false,
                _ => true,
            } {
                buf.push(c);
                c = iter.next().unwrap();
            }
            let str_buf = String::from_utf8(buf).unwrap();
            let nums = &mut str_buf.split(';');

            let cb = nums.next().unwrap().parse::<i16>().unwrap();
            let cx = nums.next().unwrap().parse::<i16>().unwrap();
            let cy = nums.next().unwrap().parse::<i16>().unwrap();

            let event = match cb {
                0..=2 | 64..=65 => {
                    let btn = match cb {
                        0 => MouseButton::Left,
                        1 => MouseButton::Middle,
                        2 => MouseButton::Right,
                        64 => MouseButton::WheelUp,
                        65 => MouseButton::WheelDn,
                        _ => unreachable!(),
                    };
                    match c {
                        b'M' => MouseEvent::Press(btn, cx, cy),
                        b'm' => MouseEvent::Release(cx, cy),
                        _ => MouseEvent::Unknown,
                    }
                }
                32 => MouseEvent::Hold(cx, cy),
                3 => MouseEvent::Release(cx, cy),
                _ => MouseEvent::Unknown,
            };
            match event {
                MouseEvent::Unknown => InputEvent::Unknown,
                _ => InputEvent::Mouse(event),
            }
        } // End xterm mouse handling
        // Match: Numbered escape code.
        Some(c @ b'0'..=b'9') => {
            let mut buf = Vec::new();
            buf.push(c);
            let mut character = iter.next().unwrap();

            // The final byte of a CSI sequence can be in the range 64-126, so
            // let's keep reading anything else.
            while character < 64 || character > 126 {
                buf.push(character);
                character = iter.next().unwrap();
            }

            match character {
                // rxvt mouse encoding:
                // ESC [ Cb ; Cx ; Cy ; M
                b'M' => {
                    let str_buf = String::from_utf8(buf).unwrap();

                    let nums: Vec<i16> = str_buf
                        .split(';')
                        .map(|n| n.parse().unwrap())
                        .collect();

                    let cb = nums[0];
                    let cx = nums[1];
                    let cy = nums[2];

                    let event = match cb {
                        32 => MouseEvent::Press(MouseButton::Left, cx, cy),
                        33 => MouseEvent::Press(MouseButton::Middle, cx, cy),
                        34 => MouseEvent::Press(MouseButton::Right, cx, cy),
                        35 => MouseEvent::Release(cx, cy),
                        64 => MouseEvent::Hold(cx, cy),
                        96 | 97 => {
                            MouseEvent::Press(MouseButton::WheelUp, cx, cy)
                        }
                        _ => MouseEvent::Unknown,
                    };

                    InputEvent::Mouse(event)
                } // End rxvt mouse encoding
                // Special key code.
                b'~' => {
                    let str_buf = String::from_utf8(buf).unwrap();

                    // This CSI sequence can be a list of
                    // semicolon-separated numbers.
                    let nums: Vec<u8> = str_buf
                        .split(';')
                        .map(|n| n.parse().unwrap())
                        .collect();

                    if nums.is_empty() {
                        return InputEvent::Unknown;
                    }

                    // (redux-os) TODO: handle multiple values for key
                    // modifiers (ex: values [3, 2] means Shift+Delete)
                    if nums.len() > 1 {
                        return InputEvent::Unknown;
                    }

                    match nums[0] {
                        1 | 7 => InputEvent::Keyboard(KeyEvent::Home),
                        2 => InputEvent::Keyboard(KeyEvent::Insert),
                        3 => InputEvent::Keyboard(KeyEvent::Delete),
                        4 | 8 => InputEvent::Keyboard(KeyEvent::End),
                        5 => InputEvent::Keyboard(KeyEvent::PageUp),
                        6 => InputEvent::Keyboard(KeyEvent::PageDn),
                        v @ 11..=15 => {
                            InputEvent::Keyboard(KeyEvent::F(v - 10))
                        }
                        v @ 17..=21 => {
                            InputEvent::Keyboard(KeyEvent::F(v - 11))
                        }
                        v @ 23..=24 => {
                            InputEvent::Keyboard(KeyEvent::F(v - 12))
                        }
                        _ => InputEvent::Unknown,
                    }
                } // End Special key code
                // (imdaveho) TODO: not in Termion, modified by
                // TimonPost, refer to TimonPost/crossterm to identify
                // reference for below, but this replaces the need for:
                // _ = InputEvent::Unknown,
                e => match (buf.last().unwrap(), e) {
                    (53, 65) => InputEvent::Keyboard(KeyEvent::CtrlUp),
                    (53, 66) => InputEvent::Keyboard(KeyEvent::CtrlDn),
                    (53, 67) => InputEvent::Keyboard(KeyEvent::CtrlRight),
                    (53, 68) => InputEvent::Keyboard(KeyEvent::CtrlLeft),
                    (50, 65) => InputEvent::Keyboard(KeyEvent::ShiftUp),
                    (50, 66) => InputEvent::Keyboard(KeyEvent::ShiftDn),
                    (50, 67) => InputEvent::Keyboard(KeyEvent::ShiftRight),
                    (50, 68) => InputEvent::Keyboard(KeyEvent::ShiftLeft),
                    _ => InputEvent::Unknown,
                }
            }
        } // End Numbered escape code.
        _ => InputEvent::Unknown,
    }
}

fn parse_utf8_char<I>(c: u8, iter: &mut I) -> Result<char>
where I: Iterator<Item = u8> {
    let error = Error::new(
        ErrorKind::Other,
        "Input character is not valid UTF-8",
    );

    if c.is_ascii() {
        Ok(c as char)
    } else {
        let mut bytes: Vec<u8> = Vec::with_capacity(4);
        bytes.push(c);

        while let Some(next) = iter.next() {
            bytes.push(next);
            if let Ok(st) = std::str::from_utf8(&bytes) {
                return Ok(st.chars().next().unwrap());
            }
            if bytes.len() >= 4 {
                return Err(error);
            }
        }

        return Err(error);
    }
}