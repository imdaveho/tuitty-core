// This module provides the Dispatcher which is a single object that handles
// user input and supports multithreaded programs through message passing.

use std::{
    thread, collections::HashMap,
    time::{ SystemTime, UNIX_EPOCH, Duration },
    sync::{
        Arc, Mutex, atomic::{ AtomicBool, AtomicUsize, Ordering },
        mpsc::{ channel, Sender, Receiver, TryRecvError, SendError }},
};
use crate::common::{ enums::{ Cmd, Action::{*, self}, InputEvent }, DELAY };

#[cfg(unix)]
pub mod unix;
#[cfg(unix)]
use std::{ fs, io::{ Read, BufReader } };
#[cfg(unix)]
use crate::terminal::actions::posix;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use crate::terminal::actions::win32;


pub struct EventHandle {
    #[cfg(unix)]
    event_rx: Receiver<u8>,
    #[cfg(windows)]
    event_rx: Receiver<InputEvent>,

    id: usize,
    signal_tx: Sender<Cmd>,
}

impl EventHandle {
    pub fn suspend(&self) {
        let _ = self.signal_tx.send(Cmd::Suspend(self.id));
    }

    pub fn transmit(&self) {
        let _ = self.signal_tx.send(Cmd::Transmit(self.id));
    }

    pub fn stop(&self) {
        let _ = self.signal_tx.send(Cmd::Stop(self.id));
    }

    pub fn lock(&self) {
        let _ = self.signal_tx.send(Cmd::Lock(self.id));
    }

    pub fn unlock(&self) {
        let _ = self.signal_tx.send(Cmd::Unlock);
    }

    #[cfg(unix)]
    pub fn poll_async(&self) -> Option<InputEvent> {
        let mut iterator = self.event_rx.try_iter();
        match iterator.next() {
            Some(ch) => {
                let parsed_evt = unix::parser::parse_event(
                    ch, &mut iterator);
                if let Ok(evt) = parsed_evt {
                    Some(evt)
                } else { None }
            }
            None => None,
        }
    }

    #[cfg(unix)]
    pub fn poll_latest_async(&self) -> Option<InputEvent> {
        let mut iterator = self.event_rx.try_iter();
        let mut result = Vec::with_capacity(8);
        while let Some(ch) = iterator.next() {
            let parsed_evt = unix::parser::parse_event(ch, &mut iterator);
            if let Ok(evt) = parsed_evt { result.push(evt) }
            else { continue }
        }
        result.pop()
    }

    #[cfg(windows)]
    pub fn poll_async(&self) -> Option<InputEvent> {
        let mut iterator = self.event_rx.try_iter();
        iterator.next()
    }

    #[cfg(windows)]
    pub fn poll_latest_async(&self) -> Option<InputEvent> {
        let mut iterator = self.event_rx.try_iter();
        let mut result = Vec::with_capacity(8);
        while let Some(evt) = iterator.next() {
            result.push(evt)
        }
        result.pop()
    }

    #[cfg(unix)]
    pub fn poll_sync(&self) -> Option<InputEvent> {
        let mut iterator = self.event_rx.iter();
        match iterator.next() {
            Some(ch) => {
                let parsed_evt = unix::parser::parse_event(
                    ch, &mut iterator);
                if let Ok(evt) = parsed_evt {
                    Some(evt)
                } else { None }
            }
            None => None,
        }
    }

    #[cfg(windows)]
    pub fn poll_sync(&self) -> Option<InputEvent> {
        let mut iterator = self.event_rx.iter();
        iterator.next()
    }

    pub fn signal(&self, action: Action) {
        let _ = self.signal_tx.send(Cmd::Signal(action));
    }
}


struct EventEmitter {
    #[cfg(unix)]
    event_tx: Sender<u8>,
    #[cfg(windows)]
    event_tx: Sender<InputEvent>,

    is_suspend: bool,
    is_running: bool,
}


pub struct Dispatcher {
    // Read /dev/tty to send keyboard and mouse events.
    input_handle: Option<thread::JoinHandle<()>>,
    // Broadcast to all spawned EventHandles.
    lock_owner: Arc<AtomicUsize>,
    emitters: Arc<Mutex<HashMap<usize, EventEmitter>>>,
    // Process signals from spawned EventHandles.
    signal_tx: Sender<Cmd>,
    signal_handle: Option<thread::JoinHandle<()>>,
    // Handle graceful shutdown and clean up.
    is_running: Arc<AtomicBool>,
}

impl Dispatcher {
    pub fn init() -> Dispatcher {
        // Initialize struct fields.
        let emitters = Arc::new(Mutex::new(HashMap::with_capacity(8)));
        let is_running = Arc::new(AtomicBool::new(true));
        let lock_owner = Arc::new(AtomicUsize::new(0));
        // Setup Arc's to move into thread.
        let emitters_arc = emitters.clone();
        let is_running_arc = is_running.clone();
        let lock_owner_arc = lock_owner.clone();

        // Fetch terminal state in main thread.
        #[cfg(unix)]
        let initial = posix::get_mode();
        #[cfg(windows)]
        let initial = win32::get_mode();
        #[cfg(windows)]
        let default = win32::get_attrib();

        // Start signal loop.
        let (signal_tx, signal_rx) = channel();
        let signal_handle = thread::spawn(move || {
            // Windows *mut c_void cannot be safely moved into thread. So
            // we create it within the thread.
            #[cfg(windows)]
            let screen = Handle::buffer()
                .expect("Error creating alternate Console buffer");
            #[cfg(windows)]
            let vte = is_ansi_enabled();
            let lock_err = "Error obtaining emitters lock";
            while is_running_arc.load(Ordering::SeqCst) {
                // Include minor delay so the thread isn't blindly using CPU.
                thread::sleep(Duration::from_millis(DELAY));
                // Handle signal commands.
                match signal_rx.try_recv() {
                    Ok(cmd) => match cmd {
                        Cmd::Continue => (),
                        Cmd::Suspend(id) => {
                            let mut roster = emitters_arc.lock()
                                .expect(lock_err);
                            roster.entry(id)
                                .and_modify(|tx: &mut EventEmitter| {
                                    tx.is_suspend = true
                                });
                        },
                        Cmd::Transmit(id) => {
                            let mut roster = emitters_arc.lock()
                                .expect(lock_err);
                            roster.entry(id)
                                .and_modify(|tx: &mut EventEmitter| {
                                    tx.is_suspend = false
                                });
                        },
                        Cmd::Stop(id) => {
                            let mut roster = emitters_arc.lock()
                                .expect(lock_err);
                            roster.entry(id)
                                .and_modify(|tx: &mut EventEmitter| {
                                    tx.is_running = false
                                });
                        },
                        Cmd::Lock(id) => {
                            match lock_owner_arc.load(Ordering::SeqCst) {
                                0 => lock_owner_arc
                                    .store(id, Ordering::SeqCst),
                                _ => continue,
                            }
                        },
                        Cmd::Unlock => {
                            match lock_owner_arc.load(Ordering::SeqCst) {
                                0 => continue,
                                _ => lock_owner_arc
                                    .store(0, Ordering::SeqCst),
                            }
                        },
                        Cmd::Signal(a) => match a {
                            // CURSOR
                            Goto(col, row) => {
                                #[cfg(unix)]
                                posix::goto(col, row);
                                #[cfg(windows)]
                                win32::goto(col, row, vte);
                            },
                            Up(n) => {
                                #[cfg(unix)]
                                posix::up(n);
                                #[cfg(windows)]
                                win32::up(n, vte);
                            },
                            Down(n) => {
                                #[cfg(unix)]
                                posix::down(n);
                                #[cfg(windows)]
                                win32::down(n, vte);
                            },
                            Left(n) => {
                                #[cfg(unix)]
                                posix::left(n);
                                #[cfg(windows)]
                                win32::left(n, vte);
                            },
                            Right(n) => {
                                #[cfg(unix)]
                                posix::right(n);
                                #[cfg(windows)]
                                win32::right(n, vte);
                            },
                            // SCREEN/OUTPUT
                            Clear(clr) => {
                                #[cfg(unix)]
                                posix::clear(clr);
                                #[cfg(windows)]
                                win32::clear(clr, vte);
                            },
                            Prints(s) => {
                                #[cfg(unix)]
                                posix::prints(&s);
                                #[cfg(windows)]
                                win32::prints(&s, vte);
                            },
                            Printf(s) => {
                                #[cfg(unix)]
                                posix::printf(&s);
                                #[cfg(windows)]
                                win32::printf(&s, vte);
                            },
                            Flush => {
                                #[cfg(unix)]
                                posix::flush();
                                #[cfg(windows)]
                                win32::flush(vte);
                            },
                            // STYLE
                            SetFx(ef) => {
                                #[cfg(unix)]
                                posix::set_fx(ef);
                                #[cfg(windows)]
                                win32::set_fx(ef, vte);
                            },
                            SetFg(c) => {
                                #[cfg(unix)]
                                posix::set_fg(c);
                                #[cfg(windows)]
                                win32::set_fg(c, default, vte);
                            },
                            SetBg(c) => {
                                #[cfg(unix)]
                                posix::set_bg(c);
                                #[cfg(windows)]
                                win32::set_bg(c, default, vte);
                            },
                            SetStyles(f, b, e) => {
                                #[cfg(unix)]
                                posix::set_styles(f, b, e);
                                #[cfg(windows)]
                                win32::set_styles(f, b, e, default, vte);
                            },
                            ResetStyles => {
                                #[cfg(unix)]
                                posix::reset_styles();
                                #[cfg(windows)]
                                win32::reset_style(default, vte);
                            },
                            // STATEFUL/MODES
                            Resize(w, h) => {
                                #[cfg(unix)]
                                posix::resize(w, h);
                                #[cfg(windows)]
                                win32::resize(w, h, vte);
                            },
                            HideCursor => {
                                #[cfg(unix)]
                                posix::hide_cursor();
                                #[cfg(windows)]
                                win32::hide_cursor(vte);
                            },
                            ShowCursor => {
                                #[cfg(unix)]
                                posix::show_cursor();
                                #[cfg(windows)]
                                win32::show_cursor(vte);
                            },
                            EnableMouse => {
                                #[cfg(unix)]
                                posix::enable_mouse();
                                #[cfg(windows)]
                                win32::enable_mouse(vte);
                            },
                            DisableMouse => {
                                #[cfg(unix)]
                                posix::disable_mouse();
                                #[cfg(windows)]
                                win32::disable_mouse(vte);
                            },
                            EnableAlt => {
                                #[cfg(unix)]
                                posix::enable_alt();
                                #[cfg(windows)]
                                win32::enable_alt(&screen, &initial, vte);
                            },
                            DisableAlt => {
                                #[cfg(unix)]
                                posix::disable_alt();
                                #[cfg(windows)]
                                win32::disable_alt(vte);
                            },
                            Raw => {
                                #[cfg(unix)]
                                posix::raw();
                                #[cfg(windows)]
                                win32::raw();
                            },
                            Cook => {
                                #[cfg(unix)]
                                posix::cook(&initial);
                                #[cfg(windows)]
                                win32::cook();
                            }
                        }
                    },
                    Err(e) => match e {
                        TryRecvError::Empty => (),
                        TryRecvError::Disconnected => is_running_arc
                            .store(false, Ordering::SeqCst),
                    }
                }
            }
            // Close the alternate screen.
            #[cfg(windows)]
            let _ = screen.close();
        });

        Dispatcher {
            input_handle: None,
            emitters: emitters,
            lock_owner: lock_owner,
            signal_tx: signal_tx,
            signal_handle: Some(signal_handle),
            is_running: is_running,
        }
    }

    pub fn listen(&mut self) -> EventHandle {
        // Do not duplicate threads.
        // If input handle exists don't do anything.
        if let Some(_) = self.input_handle { return self.spawn() }

        // Setup input channel and Arc's to move to thread.
        let is_running = self.is_running.clone();
        let lock_owner = self.lock_owner.clone();
        let emitters_arc = self.emitters.clone();
        let lock_err = "Error obtaining emitters lock";

        // Begin reading user input.
        #[cfg(unix)] {
        self.input_handle = Some(thread::spawn(move || {
            while is_running.load(Ordering::SeqCst) {
                let tty = BufReader::new(fs::OpenOptions::new()
                    .read(true).write(true).open("/dev/tty")
                    .expect("Error opening /dev/tty"));
                for byte in tty.bytes() {
                    if !is_running.load(Ordering::SeqCst) { break }
                    let b = byte.expect("Error reading byte from /dev/tty");
                    // Emitters clean up.
                    let mut roster = emitters_arc.lock().expect(lock_err);
                    if !roster.is_empty() {
                        roster.retain( |_, tx: &mut EventEmitter| {
                            tx.is_running
                        })
                    }
                    // Push user input event.
                    match lock_owner.load(Ordering::SeqCst) {
                        0 => {
                            for (_, tx) in roster.iter() {
                                if tx.is_suspend { continue }
                                let _ = tx.event_tx.send(b);
                            }
                        },
                        id => match roster.get(&id) {
                            Some(tx) => { let _ = tx.event_tx.send(b); },
                            None => lock_owner.store(0, Ordering::SeqCst),
                        }
                    }
                }
                thread::sleep(Duration::from_millis(DELAY));
            }
        }))}

        #[cfg(windows)] {
        self.input_handle = Some(thread::spawn(move || {
            while is_running.load(Ordering::SeqCst) {
                let (_, evts) = windows::parser::read_input_events()
                    .expect("Error reading console input");
                for evt in evts {
                    // Emitters clean up.
                    let mut roster = emitters_arc.lock().expect(lock_err);
                    if !roster.is_empty() {
                        roster.retain( |_, tx: &mut EventEmitter| {
                            tx.is_running
                        })
                    }
                    // Push user input event.
                    match lock_owner.load(Ordering::SeqCst) {
                        0 => {
                            for (_, tx) in roster.iter() {
                                if tx.is_suspend { continue }
                                let _ = tx.event_tx.send(evt);
                            }
                        },
                        id => match roster.get(&id) {
                            Some(tx) => { let _ = tx.event_tx.send(evt); },
                            None => lock_owner.store(0, Ordering::SeqCst),
                        }
                    }
                }
                thread::sleep(Duration::from_millis(DELAY));
            }
        }))}

        self.spawn()
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

    pub fn spawn(&self) -> EventHandle {
        let err_msg = "Error obtaining emitter registry lock";
        let (event_tx, event_rx) = channel();
        let id = self.randomish();
        let mut roster = self.emitters.lock().expect(err_msg);
        roster.insert(id, EventEmitter {
            event_tx: event_tx,
            is_suspend: false,
            is_running: true,
        });

        EventHandle {
            event_rx: event_rx,
            id: id,
            signal_tx: self.signal_tx.clone(),
        }
    }

    pub fn signal(&self, action: Action) -> Result<(), SendError<Cmd>> {
        self.signal_tx.send(Cmd::Signal(action))
    }

    fn shutdown(&mut self) -> std::thread::Result<()> {
        self.is_running.store(false, Ordering::SeqCst);
        // if let Some(t) = self.input_handle.take() { t.join()? }
        let lock_err = "Error obtaining emitters lock";
        let mut roster = self.emitters.lock().expect(lock_err);
        roster.clear();
        if let Some(t) = self.signal_handle.take() { t.join()? }
        Ok(())
    }
}

impl Drop for Dispatcher {
    fn drop(&mut self) {
        self.shutdown().expect("Error on shutdown")
    }
}
