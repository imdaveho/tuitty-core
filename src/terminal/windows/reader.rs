use std::{
    thread, sync::{
        atomic::{AtomicBool, Ordering},
        Arc, mpsc::{Receiver, Sender, channel},
    },
};
use super::parser::read_single_event;
use crate::common::enums::InputEvent;


pub struct AsyncReader {
    event_rx: Receiver<InputEvent>,
    shutdown: Arc<AtomicBool>,
}

impl AsyncReader {
    // Construct a new instance of the `AsyncReader`.
    // The reading will immediately start when calling this function.
    pub fn new(function: Box<dyn Fn(
        &Sender<InputEvent>, &Arc<AtomicBool>
    ) + Send>) -> AsyncReader {
        let shutdown_handle = Arc::new(AtomicBool::new(false));

        let (event_tx, event_rx) = channel();
        let thread_shutdown = shutdown_handle.clone();

        thread::spawn(move || loop {
            function(&event_tx, &thread_shutdown);
        });

        AsyncReader {
            event_rx,
            shutdown: shutdown_handle,
        }
    }

    // Stop the input event reading.
    //
    // You don't necessarily have to call this function because it will
    // automatically be called when this reader goes out of scope.
    //
    // - Background thread will be closed.
    // - This will consume the handle you won't be able to restart the reading
    //   with this handle, create a new `AsyncReader` instead.
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

    // Check if there are input events to read.
    //
    // It will return `None` when nothing is there to read, `Some(InputEvent)`
    // if there are events to read.
    //
    // - This is **not** a blocking call.
    // - When calling this method rapidly after each other the reader might not
    //   have read a full byte sequence of some pressed key.
    //
    // Make sure that you have some delay of a few ms when calling this method.
    fn next(&mut self) -> Option<Self::Item> {
        let mut iterator = self.event_rx.try_iter();
        iterator.next()
    }
}

pub struct SyncReader;

impl Iterator for SyncReader {
    type Item = InputEvent;

    // Read input from the user.
    //
    // If there are no keys pressed this will be a blocking call until there are.
    // This will return `None` in case of a failure and `Some(InputEvent) in
    // case of an occurred input event.`
    fn next(&mut self) -> Option<Self::Item> {
        read_single_event().unwrap_or(None)
    }
}
