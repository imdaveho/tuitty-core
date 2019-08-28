// Functions to parse Windows Console inputs and map them to the proper event.

use winapi::um::{
    wincon::{
        INPUT_RECORD, INPUT_RECORD_Event,
        KEY_EVENT_RECORD_uChar, KEY_EVENT, KEY_EVENT_RECORD,
        MENU_EVENT, FOCUS_EVENT, WINDOW_BUFFER_SIZE_EVENT,
        MOUSE_EVENT, MOUSE_EVENT_RECORD,
        LEFT_ALT_PRESSED, LEFT_CTRL_PRESSED, RIGHT_ALT_PRESSED,
        RIGHT_CTRL_PRESSED, SHIFT_PRESSED,
    },
    winuser::{
        VK_BACK, VK_CONTROL, VK_DELETE, VK_DOWN, VK_END,
        VK_ESCAPE, VK_F1, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6,
        VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12, VK_HOME,
        VK_INSERT, VK_LEFT, VK_MENU, VK_NEXT, VK_PRIOR,
        VK_RETURN, VK_RIGHT, VK_SHIFT, VK_UP,
    }
};
use winapi::shared::minwindef::{DWORD, WORD};
use winapi::um::consoleapi::{
    GetNumberOfConsoleInputEvents, ReadConsoleInputW,
};
use std::borrow::ToOwned;
use super::{
    Error, Result, Handle, InputEvent,
    MouseEvent, MouseButton, KeyEvent
};


pub fn read_single_event() -> Result<Option<InputEvent>> {
    let conin = Handle::conin()?;

    let mut buf_len: DWORD = 0;
    if !(unsafe {
        GetNumberOfConsoleInputEvents(conin.0, &mut buf_len)
    } == 0) {
        return Err(Error::last_os_error());
    }

    // Fast-skipping all the code below if there is nothing to read at all
     if buf_len == 0 {
        return Ok(None);
    }

    let mut buf: Vec<INPUT_RECORD> = Vec::with_capacity(1);
    let mut size = 0;

    if !(unsafe {
        ReadConsoleInputW(conin.0, buf.as_mut_ptr(), 1, &mut size)
    } == 0) {
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
                _parse_key_event(&key_event))))
            }
            return Ok(None)
        }
        InputEventType::MouseEvent => {
            let mouse_event = unsafe {
                MouseEventRecord::from(*input.event.MouseEvent())
            };
            Ok(Some(InputEvent::Mouse(
            _parse_mouse_event(&mouse_event))))
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
    if !(unsafe {
        GetNumberOfConsoleInputEvents(conin.0, &mut buf_len)
    } == 0) {
        return Err(Error::last_os_error());
    }
    // Fast-skipping all the code below if there is nothing to read at all
    if buf_len == 0 {
        return Ok((0, vec![]));
    }

    let mut buf: Vec<INPUT_RECORD> = Vec::with_capacity(buf_len as usize);
    let mut size = 0;

    if !(unsafe {
        ReadConsoleInputW(conin.0, buf.as_mut_ptr(), buf_len, &mut size)
    } == 0) {
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
                        _parse_key_event(&key_event));
                    events.push(event)
                }
            }
            InputEventType::MouseEvent => {
                let mouse_event = unsafe {
                    MouseEventRecord::from(*input.event.MouseEvent())
                };
                let event = InputEvent::Mouse(
                    _parse_mouse_event(&mouse_event));
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


// Describes an input event in the console input buffer.
// These can be read by using the `ReadConsoleInput` or `PeekConsoleInput`,
// or written to the input buffer by using the `WriteConsoleInput` function.
//
// https://docs.microsoft.com/en-us/windows/console/input-record-str
#[derive(Clone)]
struct InputRecord {
    // A handle to the type of input event and the event record.
    pub event_type: InputEventType,
    // The event information. The format of this member depends on
    // the event type specified by the EventType member.
    pub event: INPUT_RECORD_Event,
}

impl From<INPUT_RECORD> for InputRecord {
    fn from(event: INPUT_RECORD) -> Self {
        InputRecord {
            event_type: InputEventType::from(event.EventType),
            event: event.Event,
        }
    }
}


// A handle to the type of input event and the event record in the Event member.
//
// https://docs.microsoft.com/en-us/windows/console/input-record-str#members
#[derive(PartialEq, Debug, Copy, Clone)]
enum InputEventType {
    // The `KEY_EVENT_RECORD` structure with information about a keyboard event.
    KeyEvent = KEY_EVENT as isize,
    // The `MOUSE_EVENT_RECORD` structure with information about a mouse
    // movement or button press event.
    MouseEvent = MOUSE_EVENT as isize,
    // The `WINDOW_BUFFER_SIZE_RECORD` structure with information about the new
    // size of the console screen buffer.
    WindowBufferSizeEvent = WINDOW_BUFFER_SIZE_EVENT as isize,
    // The `FOCUS_EVENT_RECORD` structure. These events are used internally and
    // should be ignored.
    FocusEvent = FOCUS_EVENT as isize,
    // The `MENU_EVENT_RECORD` structure. These events are used internally and
    // should be ignored.
    MenuEvent = MENU_EVENT as isize,
}

impl From<WORD> for InputEventType {
    fn from(event: WORD) -> Self {
        match event {
            KEY_EVENT => InputEventType::KeyEvent,
            MOUSE_EVENT => InputEventType::MouseEvent,
            WINDOW_BUFFER_SIZE_EVENT => InputEventType::WindowBufferSizeEvent,
            FOCUS_EVENT => InputEventType::FocusEvent,
            MENU_EVENT => InputEventType::MenuEvent,
            _ => panic!("Input event type {} does not exist.", event),
        }
    }
}


// Describes a keyboard input event in a console INPUT_RECORD structure.
// https://docs.microsoft.com/en-us/windows/console/key-event-record-str
struct KeyEventRecord {
    // If the key is pressed, this member is TRUE. Otherwise, this member is
    // FALSE (the key is released).
    pub key_down: bool,
    // The repeat count, which indicates that a key is being held down.
    // For example, when a key is held down, you might get five events with this
    // member equal to 1, one event with this member equal to 5, or multiple
    // events with this member greater than or equal to 1.
    pub repeat_count: u16,
    // A virtual-key code that identifies the given key device-independently.
    pub virtual_key_code: WORD,
    // The virtual scan code of the given key that represents the device-
    // dependent value generated by the keyboard hardware.
    pub virtual_scan_code: u16,
    // A union of the following members.
    //
    // - UnicodeChar
    //   Translated Unicode character.
    //
    // - AsciiChar
    //  Translated ASCII character.
    pub u_char: KEY_EVENT_RECORD_uChar,
    /// The state of the control keys.
    pub control_key_state: ControlKeyState,
}

impl From<KEY_EVENT_RECORD> for KeyEventRecord {
    fn from(event: KEY_EVENT_RECORD) -> Self {
        KeyEventRecord {
            key_down: event.bKeyDown == 1,
            repeat_count: event.wRepeatCount,
            virtual_key_code: event.wVirtualKeyCode,
            virtual_scan_code: event.wVirtualScanCode,
            u_char: event.uChar,
            control_key_state: ControlKeyState(event.dwControlKeyState),
        }
    }
}


#[derive(PartialEq, Debug, Copy, Clone)]
struct MouseEventRecord {
    pub mouse_position: (i16, i16),
    pub button_state: ButtonState,
    pub control_key_state: ControlKeyState,
    pub event_flags: EventFlags,
}

impl From<MOUSE_EVENT_RECORD> for MouseEventRecord {
    fn from(event: MOUSE_EVENT_RECORD) -> Self {
        MouseEventRecord {
            mouse_position: (event.dwMousePosition.X, event.dwMousePosition.Y),
            button_state: ButtonState::from(event.dwButtonState),
            control_key_state: ControlKeyState(event.dwControlKeyState),
            event_flags: EventFlags::from(event.dwEventFlags),
        }
    }
}


// The status of the mouse buttons.
// The least significant bit corresponds to the leftmost mouse button.
// The next least significant bit corresponds to the rightmost mouse button.
// The next bit indicates the next-to-leftmost mouse button.
// The bits then correspond left to right to the mouse buttons.
// A bit is 1 if the button was pressed.
//
// https://docs.microsoft.com/en-us/windows/console/mouse-event-record-str
// #members
#[derive(PartialEq, Debug, Copy, Clone)]
enum ButtonState {
    Release = 0x0000,
    // The leftmost mouse button.
    FromLeft1stButtonPressed = 0x0001,
    // The second button from the left. (Middle Button)
    FromLeft2ndButtonPressed = 0x0004,
    // The third button from the left.
    FromLeft3rdButtonPressed = 0x0008,
    // The fourth button from the left.
    FromLeft4thButtonPressed = 0x0010,
    // The rightmost mouse button.
    RightmostButtonPressed = 0x0002,
    // This button state is not recognized.
    Unknown = 0x0021,
    // The wheel was rotated backward, toward the user; this will only be
    // activated for `MOUSE_WHEELED ` from `dwEventFlags`
    Negative = 0x0020,
}

impl From<DWORD> for ButtonState {
    fn from(event: DWORD) -> Self {
        let e = event as i32;

        match e {
            0x0000 => ButtonState::Release,
            0x0001 => ButtonState::FromLeft1stButtonPressed,
            0x0004 => ButtonState::FromLeft2ndButtonPressed,
            0x0008 => ButtonState::FromLeft3rdButtonPressed,
            0x0010 => ButtonState::FromLeft4thButtonPressed,
            0x0002 => ButtonState::RightmostButtonPressed,
            _ if e < 0 => ButtonState::Negative,
            _ => ButtonState::Unknown,
        }
    }
}


#[derive(PartialEq, Debug, Copy, Clone)]
struct ControlKeyState(u32);

impl ControlKeyState {
    pub fn has_state(&self, state: u32) -> bool {
        (state & self.0) != 0
    }
}


// The type of mouse event.
// If this value is zero, it indicates a mouse button being pressed or released.
// Otherwise, this member is one of the following values.
//
// https://docs.microsoft.com/en-us/windows/console/mouse-event-record-str
// #members
#[derive(PartialEq, Debug, Copy, Clone)]
enum EventFlags {
    PressOrRelease = 0x0000,
    // The second click (button press) of a double-click occurred. The first
    // click is returned as a regular button-press event.
    DoubleClick = 0x0002,
    // The horizontal mouse wheel was moved.
    MouseHwheeled = 0x0008,
    // If the high word of the dwButtonState member contains a positive value,
    // the wheel was rotated to the right. Otherwise, the wheel was rotated to
    // the left.
    MouseMoved = 0x0001,
    // A change in mouse position occurred.
    // The vertical mouse wheel was moved, if the high word of the dwButtonState
    // member contains a positive value, the wheel was rotated forward, away
    // from the user. Otherwise, the wheel was rotated backward toward the user.
    MouseWheeled = 0x0004,
}

impl From<DWORD> for EventFlags {
    fn from(event: DWORD) -> Self {
        match event {
            0x0000 => EventFlags::PressOrRelease,
            0x0002 => EventFlags::DoubleClick,
            0x0008 => EventFlags::MouseHwheeled,
            0x0001 => EventFlags::MouseMoved,
            0x0004 => EventFlags::MouseWheeled,
            _ => panic!("Event flag {} does not exist.", event),
        }
    }
}


fn _parse_key_event(kevt: &KeyEventRecord) -> KeyEvent {
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
        VK_RETURN => KeyEvent::Char('\n'),
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
                    KeyEvent::Char(ch)
                }
            } else {
                KeyEvent::Null
            }
        }
    }
}

fn _parse_mouse_event(mevt: &MouseEventRecord) -> MouseEvent {
    // NOTE (@imdaveho): xterm emulation takes the digits of the coords and
    // passes them individually as bytes into a buffer; the below cxbs and cybs
    // replicates that and mimicks the behavior; additionally, in xterm, mouse
    // move is only handled when a mouse button is held down (ie. mouse drag)

    let xpos = mevt.mouse_position.0 + 1;
    let ypos = mevt.mouse_position.1 + 1;

    match mevt.event_flags {
        EventFlags::PressOrRelease => {
            // Single Click
            match mevt.button_state {
                ButtonState::Release => {
                    // format!("\x1B[<3;{};{};M", xpos, ypos)
                    MouseEvent::Release(xpos as u16, ypos as u16)
                }
                ButtonState::FromLeft1stButtonPressed => {
                    // format!("\x1B[<0;{};{};M", xpos, ypos)
                    MouseEvent::Press(
                        MouseButton::Left,
                        xpos as u16,
                        ypos as u16,
                    )
                }
                ButtonState::RightmostButtonPressed => {
                    // format!("\x1B[<2;{};{};M", xpos, ypos)
                    MouseEvent::Press(
                        MouseButton::Right,
                        xpos as u16,
                        ypos as u16,
                    )
                }
                ButtonState::FromLeft2ndButtonPressed => {
                    // format!("\x1B[<1;{};{};M", xpos, ypos)
                    MouseEvent::Press(
                        MouseButton::Middle,
                        xpos as u16,
                        ypos as u16,
                    )
                }
                _ => MouseEvent::Unknown
            }
        }
        EventFlags::MouseMoved => {
            // Only register when the mouse is not released.
            if mevt.button_state != ButtonState::Release {
                // format!("\x1B[<32;{};{};M", xpos, ypos)
                MouseEvent::Hold(xpos as u16, ypos as u16)
            } else { MouseEvent::Unknown }
        }
        EventFlags::MouseWheeled => {
            if mevt.button_state != ButtonState::Negative {
                // format!("\x1B[<64;{};{};M")
                MouseEvent::Press(
                    MouseButton::WheelUp,
                    xpos as u16,
                    ypos as u16,
                )
            } else {
                // format!("\x1B[<65;{};{};M")
                MouseEvent::Press(
                    MouseButton::WheelDown,
                    xpos as u16,
                    ypos as u16,
                )
            }
        }
        EventFlags::DoubleClick => MouseEvent::Unknown,
        EventFlags::MouseHwheeled => MouseEvent::Unknown,
    }
}
