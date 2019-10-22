// This module provides the Dispatcher which is a single object that handles
// user input and supports multithreaded programs through message passing.

use std::{
    thread,
    collections::HashMap,
    time::{ SystemTime, UNIX_EPOCH, Duration },
    sync::{
        Arc, Mutex,
        atomic::{ AtomicBool, Ordering },
        mpsc::{ channel, Sender, Receiver, TryRecvError, SendError }
    },
};

use crate::{
    common::{ DELAY, enums::{ Cmd, Action::{*, self}, InputEvent } },
    terminal::actions::ansi::{ AnsiAction, AnsiTerminal as Terminal },
};

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use std::{ fs, io::{ Read, BufReader } };
#[cfg(unix)]
use crate::terminal::actions::ansi::unix::get_mode;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use crate::terminal::actions::wincon::{
    Win32Action, is_ansi_enabled, Win32Console as Console,
    windows::{ Handle, ConsoleInfo, get_mode },
};


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
    signal_tx: Sender<Cmd>,
}

impl EventHandle {
    pub fn pause(&self) -> Result<(), SendError<Cmd>> {
        self.signal_tx.send(Cmd::Pause(self.id))
    }

    pub fn resume(&self) -> Result<(), SendError<Cmd>> {
        self.signal_tx.send(Cmd::Resume(self.id))
    }

    pub fn stop(&self) -> Result<(), SendError<Cmd>> {
        self.signal_tx.send(Cmd::Stop(self.id))
    }

    pub fn lock(&self) -> Result<(), SendError<Cmd>> {
        self.signal_tx.send(Cmd::Lock(self.id))
    }

    pub fn unlock(&self) -> Result<(), SendError<Cmd>> {
        self.signal_tx.send(Cmd::Unlock)
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

    #[cfg(windows)]
    pub fn poll_async(&self) -> Option<InputEvent> {
        match self.event_rx.try_recv() {
            Ok(evt) => Some(evt),
            Err(_) => None,
        }
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
    pub fn poll_sync(&self) -> Result<InputEvent, RecvError> {
        self.signal_tx.send(Cmd::Next)
            .expect("Error notifying dispatch to fetch next user input");
        self.event_rx.recv()
    }

    pub fn signal(&self, action: Action) -> Result<(), SendError<Cmd>> {
        self.signal_tx.send(Cmd::Signal(action))
    }
}


pub struct Dispatcher {
    // Handle user events.
    #[cfg(unix)]
    input_tx: Sender<u8>,
    #[cfg(windows)]
    input_tx: Sender<InputEvent>,
    input_handle: Option<thread::JoinHandle<()>>,
    // Handle event broadcast.
    emitters: Arc<Mutex<HashMap<usize, Emitter>>>,
    // Handle comnmands to manipulate the terminal.
    event_handle: Option<thread::JoinHandle<()>>,
    signal_tx: Sender<Cmd>,
    // Handle graceful shutdown and clean up.
    is_running: Arc<AtomicBool>,
}

impl Dispatcher {
    pub fn init() -> Dispatcher {
        // Fetch terminal state at the start when Dispatcher first inits.
        #[cfg(windows)]
        let is_winapi: bool = !is_ansi_enabled();
        #[cfg(windows)]
        let original_mode = get_mode()
            .expect("Error fetching mode from $STDOUT");
        #[cfg(unix)]
        let original_mode = get_mode()
            .expect("Error fetching Termios");
        #[cfg(windows)]
        let reset_style: u16 = ConsoleInfo::of(
            &Handle::conout()
                .expect("Error fetching $CONOUT"))
            .expect("Error fetching ConsoleInfo from $CONOUT")
            .attributes();
        // Initialize struct fields.
        let emitters = Arc::new(Mutex::new(HashMap::with_capacity(8)));
        let is_running = Arc::new(AtomicBool::new(true));
        let (input_tx, input_rx) = channel();
        let (signal_tx, signal_rx) = channel();
        // Setup Arcs to move into thread.
        let is_running_arc = is_running.clone();
        let emitters_arc = emitters.clone();
        // Start event loop.
        let event_handle = thread::spawn(move || {
            let mut lock_owner = 0;
            let lock_err = "Error obtaining emitters lock";
            // Windows *mut c_void cannot be safely moved into thread. So
            // we create it within the thread.
            #[cfg(windows)]
            let alternate = Handle::buffer()
                .expect("Error creating alternate Console buffer");
            loop {
                if !is_running_arc.load(Ordering::SeqCst) { break }
                // Include minor delay so the thread isn't blindly using CPU.
                thread::sleep(Duration::from_millis(DELAY));
                // Push user input events if the self.listen() has been called.
                match input_rx.try_recv() {
                    Ok(m) => {
                        let mut roster = emitters_arc.lock().expect(lock_err);
                        // Emitters clean up.
                        if !roster.is_empty() {
                            roster.retain(|_, tx: &mut Emitter| tx.is_running);
                        }
                        // Push user input event.
                        match lock_owner {
                            0 => {
                                for (_, tx) in roster.iter() {
                                    if tx.is_paused { continue }
                                    // (imdaveho) TODO: Handle the Err state?
                                    // Reason: used to be .is_err() { break }
                                    // but this breaks the for loop not thread
                                    let _ = tx.event_tx.send(m);
                                }
                            },
                            id => match roster.get(&id) {
                                Some(tx) => if tx.event_tx.send(m)
                                    .is_err() { break },
                                None => lock_owner = 0,
                            }
                        }
                    },
                    Err(e) => match e {
                        TryRecvError::Empty => (),
                        TryRecvError::Disconnected => break
                    }
                }
                // Handle signal commands.
                match signal_rx.try_recv() {
                    Ok(cmd) => match cmd {
                        Cmd::Continue => (),
                        Cmd::Pause(id) => {
                            let mut roster = emitters_arc.lock()
                                .expect(lock_err);
                            roster.entry(id)
                                .and_modify(|tx| tx.is_paused = true );
                        },
                        Cmd::Resume(id) => {
                            let mut roster = emitters_arc.lock()
                                .expect(lock_err);
                            roster.entry(id)
                                .and_modify(|tx| tx.is_paused = false );
                        },
                        Cmd::Stop(id) => {
                            let mut roster = emitters_arc.lock()
                                .expect(lock_err);
                            roster.entry(id)
                                .and_modify(|tx| tx.is_running = false );
                        },
                        Cmd::Lock(id) => {
                            match lock_owner {
                                0 => lock_owner = id,
                                _ => continue,
                            }
                        },
                        Cmd::Unlock => {
                            match lock_owner {
                                0 => continue,
                                _ => lock_owner = 0,
                            }
                        },
                        Cmd::Signal(a) => match a {
                            // CURSOR
                            Goto(col, row) => {
                                #[cfg(windows)] { if is_winapi {
                                    Console::goto(col, row); continue
                                }}
                                Terminal::goto(col, row)
                            },
                            Up(n) => {
                                #[cfg(windows)] { if is_winapi {
                                    Console::up(n); continue
                                }}
                                Terminal::up(n)
                            },
                            Down(n) => {
                                #[cfg(windows)] { if win32 {
                                    Console::down(n); continue
                                }}
                                Terminal::down(n)
                            },
                            Left(n) => {
                                #[cfg(windows)] { if winapi {
                                    Console::left(n); continue
                                }}
                                Terminal::left(n)
                            },
                            Right(n) => {
                                #[cfg(windows)] { if winapi {
                                    Console::right(n); continue
                                }}
                                Terminal::right(n)
                            },
                            // SCREEN/OUTPUT
                            Clear(clr) => {
                                #[cfg(windows)] { if winapi {
                                    Console::clear(clr); continue
                                }}
                                Terminal::clear(clr)
                            },
                            Prints(s) => {
                                #[cfg(windows)] { if winapi {
                                    Console::prints(&s); continue
                                }}
                                Terminal::prints(&s)
                            },
                            Printf(s) => {
                                #[cfg(windows)] { if winapi {
                                    Console::printf(&s); continue
                                }}
                                Terminal::printf(&s)
                            },
                            Flush => {
                                #[cfg(windows)] { if winapi {
                                    Console::flush(); continue
                                }}
                                Terminal::flush()
                            },
                            // STYLE
                            SetFx(efx) => {
                                #[cfg(windows)] { if winapi {
                                    Console::set_fx(efx); continue
                                }}
                                Terminal::set_fx(efx)
                            },
                            SetFg(c) => {
                                #[cfg(windows)] { if winapi {
                                    Console::set_fg(c, reset_style); continue
                                }}
                                Terminal::set_fg(c)
                            },
                            SetBg(c) => {
                                #[cfg(windows)] { if winapi {
                                    Console::set_bg(c, reset_style); continue
                                }}
                                Terminal::set_bg(c)
                            },
                            SetStyles(f, b, e) => {
                                #[cfg(windows)] { if winapi {
                                    Console::set_styles(f, b, e, reset_style);
                                    continue
                                }}
                                Terminal::set_styles(f, b, e)
                            },
                            ResetStyles => {
                                #[cfg(windows)] { if winapi {
                                    Console::reset_styles(reset_style);
                                    continue
                                }}
                                Terminal::reset_styles()
                            },
                            // STATEFUL/MODES
                            Resize(w, h) => {
                                #[cfg(windows)] { if winapi {
                                    Console::resize(w, h); continue
                                }}
                                Terminal::resize(w, h)
                            },
                            HideCursor => {
                                #[cfg(windows)] { if winapi {
                                    Console::hide_cursor(); continue
                                }}
                                Terminal::hide_cursor()
                            },
                            ShowCursor => {
                                #[cfg(windows)] { if winapi {
                                    Console::show_cursor(); continue
                                }}
                                Terminal::show_cursor()
                            },
                            EnableMouse => {
                                #[cfg(windows)] { if winapi {
                                    Console::enable_mouse(); continue
                                }}
                                Terminal::enable_mouse()
                            },
                            DisableMouse => {
                                #[cfg(windows)] { if winapi {
                                    Console::disable_mouse(); continue
                                }}
                                Terminal::disable_mouse()
                            },
                            EnableAlt => {
                                #[cfg(windows)] { if winapi {
                                    Console::enable_alt(
                                        &alternate, &original_mode); continue
                                }}
                                Terminal::enable_alt()
                            },
                            DisableAlt => {
                                #[cfg(windows)] { if winapi {
                                    Console::disable_alt(); continue
                                }}
                                Terminal::disable_alt()
                            },
                            #[cfg(unix)]
                            Raw => Terminal::raw(),
                            #[cfg(unix)]
                            Cook => Terminal::cook(&original_mode),
                            #[cfg(windows)]
                            Raw => Console::raw(),
                            #[cfg(windows)]
                            Cook => Console::cook(),
                        }
                    },
                    Err(e) => match e {
                        TryRecvError::Empty => (),
                        TryRecvError::Disconnected => break
                    }
                }
            }
        });

        Dispatcher {
            input_handle: None,
            input_tx: input_tx,
            emitters: emitters,
            event_handle: Some(event_handle),
            signal_tx: signal_tx,
            is_running: is_running,
        }
    }

    pub fn listen(&mut self) -> &mut Dispatcher {
        // Send the input_tx Sender to listen for user input.
        let input_tx = self.input_tx.clone();
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

    pub fn shutdown(&mut self) -> std::thread::Result<()> {
        self.is_running.store(false, Ordering::SeqCst);
        if let Some(t) = self.input_handle.take() { t.join()? }
        if let Some(t) = self.event_handle.take() { t.join()? }
        Ok(())
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
        roster.insert(id, Emitter {
            event_tx: event_tx,
            is_paused: false,
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
}
