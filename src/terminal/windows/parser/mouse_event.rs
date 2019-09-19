use winapi::shared::minwindef::DWORD;
use winapi::um::wincon::MOUSE_EVENT_RECORD;
use super::{ ControlKeyState, EventFlags };


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct MouseEventRecord {
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
pub enum ButtonState {
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