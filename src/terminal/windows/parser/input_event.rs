use winapi::shared::minwindef::{ WORD, DWORD };
use winapi::um::wincon::{ 
    KEY_EVENT, MOUSE_EVENT,
    WINDOW_BUFFER_SIZE_EVENT,
    INPUT_RECORD, INPUT_RECORD_Event,
    FOCUS_EVENT, MENU_EVENT,
};


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