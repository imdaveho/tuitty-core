// Windows Console API specific functions that wrap the Reader type
// and parses user input.

use std::{
    char, thread, io::{ Error, ErrorKind, Result },
    sync::atomic::Ordering,
    time::Duration,
};
use winapi::um::winnt::INT;

use super::parser::read_input_events;
use super::{ SyncReader, AsyncReader };
use crate::common::enums::{ InputEvent, KeyEvent };


extern "C" { fn _getwche() -> INT; }


pub fn read_char() -> Result<char> {
    // _getwch is without echo and _getwche is with echo
    let pressed_char = unsafe { _getwche() };

    // we could return error but maybe option to keep listening until valid
    // character is inputted.
    if pressed_char == 0 || pressed_char == 0xe0 {
        return Err(Error::new(ErrorKind::Other,
            "Given input char is not a valid char, mostly occurs
            when pressing special keys",
        ));
    }

    match char::from_u32(pressed_char as u32) {
        Some(c) => {
            return Ok(c);
        }
        None => Err(Error::new(
            ErrorKind::Other,
            "Could not parse given input to char",
        )),
    }
}

pub fn read_async() -> AsyncReader {
    AsyncReader::new(Box::new(move |event_tx, kill_switch| loop {
        for i in read_input_events().unwrap().1 {
            if event_tx.send(i).is_err() {
                return;
            }
        }

        if kill_switch.load(Ordering::SeqCst) {
            return;
        }
        // (imdaveho) NOTE: what dis?
        thread::sleep(Duration::from_millis(1));
    }))
}

pub fn read_until_async(delimiter: u8) -> AsyncReader {
    AsyncReader::new(Box::new(move |event_tx, kill_switch| loop {
        for event in read_input_events().unwrap().1 {
            if let InputEvent::Keyboard(KeyEvent::Char(key)) = event {
                if (key as u8) == delimiter {
                    return;
                }
            }

            if kill_switch.load(Ordering::SeqCst) {
                return;
            } else {
                if event_tx.send(event).is_err() {
                    return;
                }
            }
            // (imdaveho) TODO: Test if necessary.
            thread::sleep(Duration::from_millis(1));
        }
    }))
}

pub fn read_sync() -> SyncReader {
    SyncReader
}