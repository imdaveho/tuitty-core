// This module provides the Dispatcher which is a single object that handles
// user input and supports multithreaded programs through message passing.

use std::{
    thread,
    io::Result,
    collections::HashMap,
    time::{ SystemTime, UNIX_EPOCH, Duration },
    sync::{
        Arc, Mutex,
        atomic::{ AtomicBool, Ordering },
        mpsc::{ channel, Sender, Receiver, TryRecvError }
    },
};

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use std::{ fs, io::{ Read, BufReader } };

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use crate::common::enums::{ InputEvent, Cmd };


struct Emitter {
    #[cfg(unix)]
    event_tx: Sender<u8>,
    #[cfg(windows)]
    event_tx: Sender<InputEvent>,

    is_paused: bool,
    is_running: bool,
}


pub struct EventHandle {
    #[cfg(unix)]
    event_rx: Receiver<u8>,
    #[cfg(windows)]
    event_rx: Receiver<InputEvent>,

    id: usize,
    notifier: Option<Sender<Cmd>>,
}


pub struct Dispatcher {
    // Handle user events.
    input_handle: Option<thread::JoinHandle<()>>,
    #[cfg(unix)]
    input_rx: Option<Arc<Mutex<Receiver<u8>>>>,
    #[cfg(windows)]
    input_rx: Option<Arc<Mutex<Receiver<InputEvent>>>>,
    // Handle emitting events from input_rx.
    emitters: Arc<Mutex<HashMap<usize, Emitter>>>,
    // Handle comnmands to manipulate the terminal.
    event_handle: Option<thread::JoinHandle<()>>,
    notifier: Option<Sender<Cmd>>,
    // Handle graceful shutdown and clean up.
    is_running: Arc<AtomicBool>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher {
            input_handle: None,
            input_rx: None,
            emitters: Arc::new(Mutex::new(HashMap::with_capacity(8))),
            event_handle: None,
            notifier: None,
            is_running: Arc::new(AtomicBool::new(true))
        }
    }

    pub fn listen(&mut self) -> &mut Dispatcher {
        // Setup channels to listen for user input.
        let (input_tx, input_rx) = channel();
        self.input_rx = Some(Arc::new(Mutex::new(input_rx)));
        let is_running = self.is_running.clone();
        // Begin reading user input.
        #[cfg(unix)] {
        self.input_handle = Some(thread::spawn(move || loop {
            if !is_running.load(Ordering::SeqCst) { break }
            let tty = BufReader::new(fs::OpenOptions::new()
                .read(true).write(true).open("/dev/tty")
                .expect("Error opening /dev/tty"));
            for byte in tty.bytes() {
                let b = byte.expect("Error reading byte from /dev/tty");
                if input_tx.send(b).is_err() { break }
            }
            thread::sleep(Duration::from_millis(1));
        }))}
        #[cfg(windows)] {
        self.input_handle = Some(thread::spawn(move || loop {
            if !is_running.load(Ordering::SeqCst) { break }
            loop {
                if !is_running.load(Ordering::SeqCst) { break }
                let (_, evts) = windows::parser::read_input_events()
                    .expect("Error reading console input");
                for evt in evts {
                    if input_tx.send(evt).is_err() { break }
                }
                thread::sleep(Duration::from_millis(1));
            }
        }))}

        return self;
    }
    
    pub fn dispatch(&mut self) -> &mut Dispatcher {
        // Setup channels to handle terminal commands.
        let input_rx = match &self.input_rx {
            Some(arc) => arc.clone(),
            None => return self,
        };
        let (notifier, observer) = channel();
        self.notifier = Some(notifier);
        let emitters = self.emitters.clone();
        let emitters_err = "Error obtaining emitter registry lock";
        // Begin dispatcher event loop.
        self.event_handle = Some(thread::spawn(move || loop {
            let cmd = match observer.try_recv() {
                Ok(cmd) => cmd,
                Err(e) => match e {
                    TryRecvError::Empty => Cmd::Continue,
                    TryRecvError::Disconnected => break
                }
            };
            let input_rx = input_rx.lock()
                .expect("Error obtaining input_rx lock");
            match cmd {
                Cmd::Continue => (),
                Cmd::Emit => match input_rx.try_recv() {
                    Ok(ev) => {
                        let mut registry = emitters.lock()
                            .expect(emitters_err);
                        // Emitter registry clean up.
                        registry.retain(|_, emitter| emitter.is_running );
                        for (_, emitter) in registry.iter() {
                            if emitter.is_paused { continue }
                            if emitter.event_tx.send(ev).is_err() { break }
                        }
                    },
                    Err(e) => match e {
                        TryRecvError::Empty => (),
                        TryRecvError::Disconnected => break
                    }
                },
                Cmd::Execute(a) => {

                }
            }
            thread::sleep(Duration::from_millis(1));
        }));
        
        return self;
    }

    fn randomish(&self) -> usize {
        match self.emitters.lock() {
            Ok(senders) => {
                let mut key: usize;
                loop {
                    key = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Error fetching duration since 1970")
                        .subsec_nanos() as usize;
                    if !senders.contains_key(&key) { break }
                }
                return key;
            },
            Err(_) => {
                return SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Error fetching duration since 1970")
                        .subsec_nanos() as usize;
            }
        }
    }

    pub fn spawn(&self) -> EventHandle {
        let err_msg = "Error obtaining emitter registry lock";
        let (event_tx, event_rx) = channel();
        let id = self.randomish();
        let mut registry = self.emitters.lock().expect(err_msg);
        registry.insert(id, Emitter {
            event_tx: event_tx,
            is_paused: false,
            is_running: true,
        });
        let notifier = self.notifier.clone();
        
        EventHandle { 
            event_rx: event_rx,
            id: id,
            notifier: notifier,
        }
    }

    // pub fn pause(&self, rx: &EventListener) {
    //     let err_msg = "Error obtaining lock for the dispatch registry";
    //     let mut senders = self.registry.lock().expect(err_msg);
    //     if let Some(em) = senders.get_mut(&rx.lookup) {
    //         em.sending = false;
    //     }
    // }

    // pub fn resume(&self, rx: &EventListener) {
    //     let err_msg = "Error obtaining lock for the dispatch registry";
    //     let mut senders = self.registry.lock().expect(err_msg);
    //     if let Some(em) = senders.get_mut(&rx.lookup) {
    //         em.sending = true;
    //     }
    // }

    // pub fn close(&self, rx: &EventListener) {
    //     let err_msg = "Error obtaining lock for the dispatch registry";
    //     let mut senders = self.registry.lock().expect(err_msg);
    //     if let Some(em) = senders.get_mut(&rx.lookup) {
    //         em.shutdown = true;
    //     }
    // }

    // pub fn shutdown(&self) {
    //     self.shutdown.store(true, Ordering::SeqCst);
    // }

    // pub fn stop(&self) -> Arc<AtomicBool> {
    //     self.shutdown.clone()
    // }
}