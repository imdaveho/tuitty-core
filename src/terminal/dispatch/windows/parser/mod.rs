// Functions to parse Windows Console inputs and map them to the proper event.

mod input_event;
use input_event::{ 
    InputRecord, InputEventType,
    ControlKeyState, EventFlags
};

mod keyboard_event;
use keyboard_event::KeyEventRecord;

mod mouse_event;
use mouse_event::{ MouseEventRecord, ButtonState };

use std::{ borrow::ToOwned, io::{ Error, Result } };
use winapi::shared::minwindef::DWORD;
use winapi::um::{
    wincon::{
        INPUT_RECORD,
        LEFT_ALT_PRESSED, LEFT_CTRL_PRESSED, RIGHT_ALT_PRESSED,
        RIGHT_CTRL_PRESSED, SHIFT_PRESSED,
    },
    winuser::{
        VK_BACK, VK_CONTROL, VK_DELETE, VK_DOWN, VK_END,
        VK_ESCAPE, VK_F1, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6,
        VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12, VK_HOME,
        VK_INSERT, VK_LEFT, VK_MENU, VK_NEXT, VK_PRIOR,
        VK_RETURN, VK_RIGHT, VK_SHIFT, VK_UP,
    },
    consoleapi::{
        GetNumberOfConsoleInputEvents, ReadConsoleInputW,
    },
};
use crate::terminal::actions::wincon::windows::Handle;
use crate::common::enums::{ InputEvent, KeyEvent, MouseEvent, MouseButton };


pub fn read_single_event() -> Result<Option<InputEvent>> {
    let conin = Handle::conin()?;

    let mut buf_len: DWORD = 0;
    if unsafe {
        GetNumberOfConsoleInputEvents(conin.0, &mut buf_len)
    } == 0 {
        return Err(Error::last_os_error());
    }

    // Fast-skipping all the code below if there is nothing to read at all
     if buf_len == 0 {
        return Ok(None);
    }

    let mut buf: Vec<INPUT_RECORD> = Vec::with_capacity(1);
    let mut size = 0;

    if unsafe {
        ReadConsoleInputW(conin.0, buf.as_mut_ptr(), 1, &mut size)
    } == 0 {
        return Err(Error::last_os_error());
    } else {
        unsafe {
            buf.set_len(1 as usize);
        }
    }

    let input = buf[..(1 as usize)]
            .iter()
            .map(|x| InputRecord::from(*x))
            .collect::<Vec<InputRecord>>()[0]
            .to_owned();
    match input.event_type {
        InputEventType::KeyEvent => {
            let key_event = unsafe {
                KeyEventRecord::from(*input.event.KeyEvent())
            };
            if key_event.key_down {
                return Ok(Some(InputEvent::Keyboard(
                parse_key_event(&key_event))))
            }
            return Ok(None)
        }
        InputEventType::MouseEvent => {
            let mouse_event = unsafe {
                MouseEventRecord::from(*input.event.MouseEvent())
            };
            Ok(Some(InputEvent::Mouse(
            parse_mouse_event(&mouse_event))))
        }
        // TODO implement terminal resize event
        InputEventType::WindowBufferSizeEvent => Ok(None),
        InputEventType::FocusEvent => Ok(None),
        InputEventType::MenuEvent => Ok(None),
    }
}

pub fn read_input_events() -> Result<(u32, Vec<InputEvent>)> {
    let conin = Handle::conin()?;

    let mut buf_len: DWORD = 0;
    if unsafe {
        GetNumberOfConsoleInputEvents(conin.0, &mut buf_len)
    } == 0 {
        return Err(Error::last_os_error());
    }
    // Fast-skipping all the code below if there is nothing to read at all
    if buf_len == 0 {
        return Ok((0, vec![]));
    }

    let mut buf: Vec<INPUT_RECORD> = Vec::with_capacity(buf_len as usize);
    let mut size = 0;

    if unsafe {
        ReadConsoleInputW(conin.0, buf.as_mut_ptr(), buf_len, &mut size)
    } == 0 {
        return Err(Error::last_os_error());
    } else {
        unsafe {
            buf.set_len(buf_len as usize);
        }
    }

    let result = (
        buf_len,
        buf[..(buf_len as usize)]
            .iter()
            .map(|x| InputRecord::from(*x))
            .collect::<Vec<InputRecord>>(),);

    let mut events = Vec::with_capacity(result.0 as usize);

    for input in result.1 {
        match input.event_type {
            InputEventType::KeyEvent => {
                let key_event = unsafe {
                    KeyEventRecord::from(*input.event.KeyEvent())
                };
                if key_event.key_down {
                    let event = InputEvent::Keyboard(
                        parse_key_event(&key_event));
                    events.push(event)
                }
            }
            InputEventType::MouseEvent => {
                let mouse_event = unsafe {
                    MouseEventRecord::from(*input.event.MouseEvent())
                };
                let event = InputEvent::Mouse(
                    parse_mouse_event(&mouse_event));
                events.push(event)
            }
            // TODO implement terminal resize event
            InputEventType::WindowBufferSizeEvent => (),
            InputEventType::FocusEvent => (),
            InputEventType::MenuEvent => (),
        }
    }
    return Ok((result.0, events));
}


fn parse_key_event(kevt: &KeyEventRecord) -> KeyEvent {
    let kcode = kevt.virtual_key_code as i32;
    match kcode {
        // ignore SHIFT | CTRL | ALT or (0x10 | 0x11 | 0x12)
        // standalone presses
        VK_SHIFT | VK_CONTROL | VK_MENU => KeyEvent::Null,
        // (0x08) => vec![b'\x7F']
        VK_BACK => KeyEvent::Backspace,
        // (0x1B) => vec![b'\x1B']
        VK_ESCAPE => KeyEvent::Esc,
        // (0x0D) => vec![b'\n']
        VK_RETURN => KeyEvent::Enter,
        // For F1 - F12, match the key_codes with the byte value
        // format!("\x1BO{}", __)
        // 0x70..=0x73 => b'P'..=b'S' (F1 - F4)
        // 0x74 => b'5' (F5)
        // 0x75..=0x77) => b'7'..=b'9' (F6 - F8)
        // 0x78..=0x79 => b'0'..=b'1' (F9 - F10)
        // 0x7A..=0x7B => b'3'..=b'4' (F11 - F12)
        VK_F1 | VK_F2 | VK_F3 | VK_F4
        | VK_F5 | VK_F6 | VK_F7 | VK_F8
        | VK_F9 | VK_F10 | VK_F11 | VK_F12 => KeyEvent::F((kcode - 111) as u8),
        // 0x25 | 0x26 | 0x27 | 0x28
        // format!("\x1B[{}{}", __, __)
        // first string variable is if CTRL or SHIFT is pressed:
        // if CTRL => 0x35 (53)
        // if SHIFT => 0x32 (50)
        // second string variable is matching the char with
        // left, up, right, down: [b'D', b'A', b'C', b'B']
        VK_LEFT | VK_UP | VK_RIGHT | VK_DOWN => {
            // Modifier Keys (Ctrl, Shift) Support
            let kstate = kevt.control_key_state;
            let ctrl = kstate.has_state(RIGHT_CTRL_PRESSED | LEFT_CTRL_PRESSED);
            let shift = kstate.has_state(SHIFT_PRESSED);

            return match kcode {
                VK_LEFT => {
                    if ctrl { KeyEvent::CtrlLeft }
                    else if shift { KeyEvent::ShiftLeft }
                    else { KeyEvent::Left }
                }
                VK_UP => {
                    if ctrl { KeyEvent::CtrlUp }
                    else if shift { KeyEvent::ShiftUp }
                    else { KeyEvent::Up }
                }
                VK_RIGHT => {
                    if ctrl { KeyEvent::CtrlRight }
                    else if shift { KeyEvent::ShiftRight }
                    else { KeyEvent::Right }
                }
                VK_DOWN => {
                    if ctrl { KeyEvent::CtrlDown }
                    else if shift { KeyEvent::ShiftDown }
                    else { KeyEvent::Down }
                }
                _ => KeyEvent::Null,
            }
        }
        // PAGEUP 0x21 | PAGEDOWN 0x22
        // format!("\x1B[{}~", __)
        // if PAGEUP (b'5') or PAGEDOWN (b'6')
        VK_PRIOR | VK_NEXT => {
            if kcode == VK_PRIOR { KeyEvent::PageUp }
            else if kcode == VK_NEXT { KeyEvent::PageDown }
            else { KeyEvent::Null }
        }
        // END 0x23 | HOME 0x24
        // format!("\x1B[{}", __)
        // if END (b'F') or HOME (b'H')
        VK_END | VK_HOME => {
            if kcode == VK_HOME { KeyEvent::Home }
            else if kcode == VK_END { KeyEvent::End }
            else { KeyEvent::Null }
        }
        // INSERT 0x2D | DELETE 0x2E
        // format!("\x1B[{}~", __)
        // if INSERT (b'2') or DELETE (b'3')
        VK_DELETE => KeyEvent::Delete,
        VK_INSERT => KeyEvent::Insert,
        _ => {
            let alt = LEFT_ALT_PRESSED | RIGHT_ALT_PRESSED;
            let ctrl = LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED;
            let shift = SHIFT_PRESSED;
            // Modifier Keys (Ctrl, Alt, Shift) Support
            let chraw = { (unsafe { *kevt.u_char.UnicodeChar() } as u16) };
            // (imdaveho) NOTE: should there be u16 support?
            // ie. East Asian Characters?
            // if not, then we only consider u8, max: 255
            if chraw < 255 {
                let ch = chraw as u8 as char;
                let kstate = kevt.control_key_state;
                if kstate.has_state(alt) {
                    // hex codes: 0x0002 | 0x0101 | 0x0001
                    // If the ALT key is held down, pressing the A key produces
                    // ALT+A,
                    // which the system does not treat as a character at all,
                    // but rather as a system command. The pressed command is
                    // stored in `virtual_key_code`.
                    let cmd = kevt.virtual_key_code as u8 as char;
                    // format!("\x1B{}", cmd)
                    if (cmd).is_alphabetic() { KeyEvent::Alt(cmd) }
                    else { KeyEvent::Null }
                } else if kstate.has_state(ctrl) {
                    // hex codes: 0x0008 | 0x0104 | 0x0004
                    match chraw as u8 {
                        // alphabet
                        c @ b'\x01'..=b'\x1A' => {
                            KeyEvent::Ctrl((c as u8 - 0x1 + b'a') as char)
                        }
                        // (imdaveho) TODO: what is? 4 - 7?
                        c @ b'\x1C'..=b'\x1F' => {
                            KeyEvent::Ctrl((c as u8 - 0x1C + b'4') as char)
                        }
                        _ => KeyEvent::Null,
                    }
                } else if kstate.has_state(shift) {
                    // Shift + key press, essentially the same as single key
                    // press. Separating to be explicit about the Shift press.
                    if ch == '\t' {
                        // "\x1B[Z".as_bytes().to_vec();
                        KeyEvent::BackTab
                    } else {
                        // KeyEvent::Tab
                        KeyEvent::Char(ch)
                    }
                } else {
                    // 0x000A | 0x0105 | 0x0005 => {
                    //     // TODO: Alt + Ctrl + Key support
                    //     // mainly updating the Alt section of parse_event()
                    //     // and updating parse_utf8_char()
                    //     seq.push(b'\x00');
                    // },
                    // 0x001A | 0x0115 | 0x0015 => {
                    //     // TODO: Alt + Ctrl + Shift Key support
                    //     // mainly updating the Alt section of parse_event()
                    //     // and updating parse_utf8_char()
                    //     seq.push(b'\x00');
                    // }
                    if ch == '\t' {
                        KeyEvent::Tab
                    } else {
                        KeyEvent::Char(ch)
                    }
                }
            } else {
                KeyEvent::Null
            }
        }
    }
}

fn parse_mouse_event(mevt: &MouseEventRecord) -> MouseEvent {
    // NOTE (@imdaveho): xterm emulation takes the digits of the coords and
    // passes them individually as bytes into a buffer; the below cxbs and cybs
    // replicates that and mimicks the behavior; additionally, in xterm, mouse
    // move is only handled when a mouse button is held down (ie. mouse drag)

    // Windows returns (0, 0) for upper/left
    let xpos = mevt.mouse_position.0;
    let ypos = mevt.mouse_position.1;

    match mevt.event_flags {
        EventFlags::PressOrRelease => {
            // Single Click
            match mevt.button_state {
                ButtonState::Release => {
                    // format!("\x1B[<3;{};{};M", xpos, ypos)
                    MouseEvent::Release(xpos, ypos)
                }
                ButtonState::FromLeft1stButtonPressed => {
                    // format!("\x1B[<0;{};{};M", xpos, ypos)
                    MouseEvent::Press(MouseButton::Left, xpos, ypos)
                }
                ButtonState::RightmostButtonPressed => {
                    // format!("\x1B[<2;{};{};M", xpos, ypos)
                    MouseEvent::Press(
                        MouseButton::Right, xpos, ypos)
                }
                ButtonState::FromLeft2ndButtonPressed => {
                    // format!("\x1B[<1;{};{};M", xpos, ypos)
                    MouseEvent::Press(MouseButton::Middle, xpos, ypos)
                }
                _ => MouseEvent::Unknown
            }
        }
        EventFlags::MouseMoved => {
            // Only register when the mouse is not released.
            if mevt.button_state != ButtonState::Release {
                // format!("\x1B[<32;{};{};M", xpos, ypos)
                MouseEvent::Hold(xpos, ypos)
            } else { MouseEvent::Unknown }
        }
        EventFlags::MouseWheeled => {
            if mevt.button_state != ButtonState::Negative {
                // format!("\x1B[<64;{};{};M")
                MouseEvent::Press(MouseButton::WheelUp, xpos, ypos)
            } else {
                // format!("\x1B[<65;{};{};M")
                MouseEvent::Press(MouseButton::WheelDown, xpos, ypos)
            }
        }
        EventFlags::DoubleClick => MouseEvent::Unknown,
        EventFlags::MouseHwheeled => MouseEvent::Unknown,
    }
}
