// Unix specific Reader types to handle user input from stdin.

use std::{
    thread, sync::{
        atomic::{AtomicBool, Ordering},
        Arc, mpsc::{Receiver, Sender, channel},
    },
};
use super::parser::parse_event;
use crate::common::enums::{InputEvent, MouseEvent, MouseButton, KeyEvent};


pub struct AsyncReader {
    event_rx: Receiver<u8>,
    shutdown: Arc<AtomicBool>,
}

impl AsyncReader {
    pub fn new(func: Box<dyn Fn(
        &Sender<u8>,
        &Arc<AtomicBool>
    ) + Send>) -> AsyncReader {
        let shutdown_handle = Arc::new(AtomicBool::new(false));

        let (event_tx, event_rx) = channel();
        let thread_shutdown = shutdown_handle.clone();

        thread::spawn(move || loop {
            func(&event_tx, &thread_shutdown);
        });

        AsyncReader {
            event_rx,
            shutdown: shutdown_handle,
        }
    }

    pub fn stop_reading(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}

impl Iterator for AsyncReader {
    type Item = InputEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iterator = self.event_rx.try_iter();

        match iterator.next() {
            Some(char_value) => {
                if let Ok(char_value) = parse_event(char_value, &mut iterator) {
                    Some(char_value)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl Drop for AsyncReader {
    fn drop(&mut self) {
        self.stop_reading();
    }
}


pub struct SyncReader {
    source: Box<std::fs::File>,
    leftover: Option<u8>,
}

impl Iterator for SyncReader {
    type Item = InputEvent;
    // Read input from the user.
    //
    // If there are no keys pressed, this will be a blocking call
    // until there is one.
    // This will return `None` in case of a failure and `Some(InputEvent)`
    // in case of an occurred input event.
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Currently errors are consumed and converted to
        // a `None`. Maybe we shouldn't be doing this?
        let source = &mut self.source;

        if let Some(c) = self.leftover {
            // we have a leftover byte, use it
            self.leftover = None;
            if let Ok(e) = parse_event(c, &mut source.bytes().flatten()) {
                return Some(e);
            } else {
                return None;
            }
        }

        // Here we read two bytes at a time. We need to distinguish
        // between single ESC key presses,
        // and escape sequences (which start with ESC or a x1B byte).
        // The idea is that if this is
        // an escape sequence, we will read multiple bytes (the first
        // byte being ESC) but if this
        // is a single ESC keypress, we will only read a single byte.
        let mut buf = [0u8; 2];

        let res = match source.read(&mut buf) {
            Ok(0) => return None,
            Ok(1) => match buf[0] {
                b'\x1B' => return Some(InputEvent::Keyboard(KeyEvent::Esc)),
                c => {
                    if let Ok(e) = parse_event(
                        c, &mut source
                            .bytes()
                            .flatten()) {
                        return Some(e);
                    } else {
                        return None;
                    }
                }
            },
            Ok(2) => {
                let option_iter = &mut Some(buf[1]).into_iter();
                let iter = option_iter.map(|c| Ok(c)).chain(source.bytes());
                if let Ok(e) = parse_event(buf[0], &mut iter.flatten()) {
                    self.leftover = option_iter.next();
                    Some(e)
                } else {
                    None
                }
            }
            Ok(_) => unreachable!(),
            Err(_) => return None,
            // maybe we should not throw away the error?
        };
        return res;
    }
}