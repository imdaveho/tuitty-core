// This module provides the Dispatcher which is a single object that handles
// user input and supports multithreaded programs through message passing.

use std::{
    thread,
    collections::HashMap,
    io::{ Error, ErrorKind },
    time::{ SystemTime, UNIX_EPOCH, Duration },
    sync::{
        Arc, Mutex,
        atomic::{ AtomicBool, AtomicUsize, Ordering },
        mpsc::{ channel, Sender, Receiver, TryRecvError, SendError, RecvError }
    },
};

use crate::terminal::actions::execute;
use crate::common::DELAY;

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
    notifier: Sender<Cmd>,
}

impl EventHandle {
    pub fn pause(&self) -> Result<(), SendError<Cmd>> {
        self.notifier.send(Cmd::Pause(self.id))
    }

    pub fn resume(&self) -> Result<(), SendError<Cmd>> {
        self.notifier.send(Cmd::Resume(self.id))
    }

    pub fn stop(&self) -> Result<(), SendError<Cmd>> {
        self.notifier.send(Cmd::Stop(self.id))
    }

    pub fn lock(&self) -> Result<(), SendError<Cmd>> {
        self.notifier.send(Cmd::Lock(self.id))
    }

    pub fn unlock(&self) -> Result<(), SendError<Cmd>> {
        self.notifier.send(Cmd::Unlock)
    }

    #[cfg(unix)]
    pub fn poll_async(&self) -> Option<u8> {
        self.notifier.send(Cmd::Next)
            .expect("Error notifying dispatch to fetch next user input");
        match self.event_rx.try_recv() {
            Ok(byte) => Some(byte),
            Err(_) => None,
        }
    }

    #[cfg(windows)]
    pub fn poll_async(&self) -> Option<InputEvent> {
        self.notifier.send(Cmd::Next)
            .expect("Error notifying dispatch to fetch next user input");
        match self.event_rx.try_recv() {
            Ok(evt) => Some(evt),
            Err(_) => None,
        }
    }

    #[cfg(unix)]
    pub fn poll_sync(&self) -> Result<u8, RecvError> {
        self.notifier.send(Cmd::Next)
            .expect("Error notifying dispatch to fetch next user input");
        self.event_rx.recv()
    }

    #[cfg(windows)]
    pub fn poll_sync(&self) -> Result<InputEvent, RecvError> {
        self.notifier.send(Cmd::Next)
            .expect("Error notifying dispatch to fetch next user input");
        self.event_rx.recv()
    }
}


pub struct Dispatcher {
    // Handle user events.
    input_handle: Option<thread::JoinHandle<()>>,
    #[cfg(unix)]
    input_rx: Option<Arc<Mutex<Receiver<u8>>>>,
    #[cfg(windows)]
    input_rx: Option<Arc<Mutex<Receiver<InputEvent>>>>,
    // Handle emitting events from input_rx.
    locked_id: Arc<AtomicUsize>,
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
            locked_id: Arc::new(AtomicUsize::new(0)),
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
            thread::sleep(Duration::from_millis(DELAY));
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
                thread::sleep(Duration::from_millis(DELAY));
            }
        }))}

        return self;
    }
    
    pub fn dispatch(&mut self) -> &mut Dispatcher {
        // Setup channels to handle terminal commands.
        let (_, rx) = channel();
        let empty_rx = Arc::new(Mutex::new(rx));
        let input_rx = match &self.input_rx {
            Some(arc) => arc.clone(),
            None => empty_rx.clone(),
        };
        let (notifier, observer) = channel();
        self.notifier = Some(notifier);
        let emitters = self.emitters.clone();
        let emitters_err = "Error obtaining emitter registry lock";
        let is_running = self.is_running.clone();
        let locked_id = self.locked_id.clone();
        // Begin dispatcher event loop.
        self.event_handle = Some(thread::spawn(move || loop {
            if !is_running.load(Ordering::SeqCst) { break }
            let cmd = match observer.try_recv() {
                Ok(cmd) => cmd,
                Err(e) => match e {
                    TryRecvError::Empty => Cmd::Continue,
                    TryRecvError::Disconnected => break
                }
            };
            let input_rx = input_rx.lock()
                .expect("Error obtaining input_rx lock");
            // let store = Store::new(); <-- this will hold State.
            match cmd {
                Cmd::Continue => (),
                Cmd::Next => match input_rx.try_recv() {
                    Ok(ev) => {
                        let mut registry = emitters.lock()
                            .expect(emitters_err);
                        // Emitter registry clean up.
                        registry.retain(|_, emitter| emitter.is_running );
                        match locked_id.load(Ordering::SeqCst) {
                            0 => {
                                for (_, emitter) in registry.iter() {
                                    if emitter.is_paused { continue }
                                    if emitter.event_tx.send(ev)
                                        .is_err() { break }
                                }
                            },
                            id => match registry.get(&id) {
                                Some(emitter) => if emitter.event_tx.send(ev)
                                    .is_err() { break },
                                None => locked_id.store(0, Ordering::SeqCst)
                            }
                        }
                    },
                    Err(e) => match e {
                        TryRecvError::Empty => (),
                        TryRecvError::Disconnected => break
                    }
                },
                Cmd::Pause(id) => {
                    let mut registry = emitters.lock()
                        .expect(emitters_err);
                    registry.entry(id).and_modify(|emitter| emitter.is_paused = true );
                },
                Cmd::Resume(id) => {
                    let mut registry = emitters.lock()
                        .expect(emitters_err);
                    registry.entry(id).and_modify(|emitter| emitter.is_paused = false );
                },
                Cmd::Stop(id) => {
                    let mut registry = emitters.lock()
                        .expect(emitters_err);
                    registry.entry(id).and_modify(|emitter| emitter.is_running = false );
                },
                Cmd::Lock(id) => {
                    match locked_id.load(Ordering::SeqCst) {
                        0 => locked_id.store(id, Ordering::SeqCst),
                        _ => continue,
                    }
                },
                Cmd::Unlock => {
                    match locked_id.load(Ordering::SeqCst) {
                        0 => continue,
                        _ => locked_id.store(0, Ordering::SeqCst),
                    }
                },
                Cmd::Execute(a) => execute(a),
            }
            thread::sleep(Duration::from_millis(DELAY));
        }));
        
        return self;
    }

    pub fn shutdown(&mut self) {
        self.is_running.store(false, Ordering::SeqCst);
        if let Some(t) = self.input_handle.take() { let _ = t.join(); }
        if let Some(t) = self.event_handle.take() { let _ = t.join(); }
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
                    if key == 0 { continue }
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

    pub fn spawn(&self) -> Result<EventHandle, Error> {
        let err_msg = "Error obtaining emitter registry lock";
        let (event_tx, event_rx) = channel();
        let id = self.randomish();
        let mut registry = self.emitters.lock().expect(err_msg);
        registry.insert(id, Emitter {
            event_tx: event_tx,
            is_paused: false,
            is_running: true,
        });

        match &self.notifier {
            Some(notifier) => {
                Ok(EventHandle { 
                    event_rx: event_rx,
                    id: id,
                    notifier: notifier.clone(),
                })
            },
            None => Err(Error::new(ErrorKind::InvalidData, "Missing notifier Sender")),
        }
    }
} 