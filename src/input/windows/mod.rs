//! This is a Windows specific implementation for input handling.
use std::{char, thread};
use std::io::{Error, ErrorKind, Result};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::Duration;
use winapi::um::winnt::INT;
use super::{Handle, InputEvent, KeyEvent, TtyResult};
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc,
};


mod parser;
use parser::{read_single_event, read_input_events};


const MOUSE_MODE: u32 = 0x0010 | 0x0080 | 0x0008;

extern "C" {
    fn _getwche() -> INT;
}


pub fn _read_char() -> Result<char> {
    // _getwch is without echo and _getwche is with echo
    let pressed_char = unsafe { _getwche() };

    // we could return error but maybe option to keep listening until valid character is inputted.
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

pub fn _read_async() -> AsyncReader {
    AsyncReader::new(Box::new(move |event_tx, kill_switch| loop {
        for i in read_input_events().unwrap().1 {
            if event_tx.send(i).is_err() {
                return;
            }
        }

        if kill_switch.load(Ordering::SeqCst) {
            return;
        }
        // (imdaveho) NOTE: what is?
        thread::sleep(Duration::from_millis(1));
    }))
}

pub fn _read_until_async(delimiter: u8) -> AsyncReader {
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
            // (imdaveho) NOTE: what is?
            thread::sleep(Duration::from_millis(1));
        }
    }))
}

pub fn _read_sync() -> SyncReader {
    SyncReader
}

pub fn _enable_mouse_mode() -> TtyResult<()> {
    let handle = Handle::conin()?;
    let mode = handle.get_mode()?;
    let mouse_mode = mode | MOUSE_MODE;
    handle.set_mode(&mouse_mode)?;
    Ok(())
}

// pub fn _disable_mouse_mode() -> TtyResult<()> {
//     let handle = Handle::conin()?;
//     let mode = handle.get_mode()?;
//     let mouse_mode = mode & !MOUSE_MODE;
//     handle.set_mode(&mouse_mode)?;
//     Ok(())
// }


pub struct AsyncReader {
    event_rx: Receiver<InputEvent>,
    shutdown: Arc<AtomicBool>,
}

impl AsyncReader {
    /// Construct a new instance of the `AsyncReader`.
    /// The reading will immediately start when calling this function.
    pub fn new(function: Box<Fn(&Sender<InputEvent>, &Arc<AtomicBool>) + Send>) -> AsyncReader {
        let shutdown_handle = Arc::new(AtomicBool::new(false));

        let (event_tx, event_rx) = mpsc::channel();
        let thread_shutdown = shutdown_handle.clone();

        thread::spawn(move || loop {
            function(&event_tx, &thread_shutdown);
        });

        AsyncReader {
            event_rx,
            shutdown: shutdown_handle,
        }
    }

    /// Stop the input event reading.
    ///
    /// You don't necessarily have to call this function because it will automatically be called when this reader goes out of scope.
    ///
    /// # Remarks
    /// - Background thread will be closed.
    /// - This will consume the handle you won't be able to restart the reading with this handle, create a new `AsyncReader` instead.
    pub fn stop_reading(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}

impl Drop for AsyncReader {
    fn drop(&mut self) {
        self.stop_reading();
    }
}

impl Iterator for AsyncReader {
    type Item = InputEvent;

    /// Check if there are input events to read.
    ///
    /// It will return `None` when nothing is there to read, `Some(InputEvent)` if there are events to read.
    ///
    /// # Remark
    /// - This is **not** a blocking call.
    /// - When calling this method to fast after each other the reader might not have read a full byte sequence of some pressed key.
    /// Make sure that you have some delay of a few ms when calling this method.
    fn next(&mut self) -> Option<Self::Item> {
        let mut iterator = self.event_rx.try_iter();
        iterator.next()
    }
}

pub struct SyncReader;

impl Iterator for SyncReader {
    type Item = InputEvent;

    /// Read input from the user.
    ///
    /// If there are no keys pressed this will be a blocking call until there are.
    /// This will return `None` in case of a failure and `Some(InputEvent) in case of an occurred input event.`
    fn next(&mut self) -> Option<Self::Item> {
        read_single_event().unwrap()
    }
}