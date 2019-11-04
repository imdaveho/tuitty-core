// This module provides the Dispatcher which is a single object that handles
// user input and supports multithreaded programs through message passing.

use std::{
    thread, collections::HashMap,
    time::{ SystemTime, UNIX_EPOCH, Duration },
    sync::{
        Arc, Mutex, atomic::{ AtomicBool, AtomicUsize, Ordering },
        mpsc::{ channel, Sender, Receiver, TryRecvError, SendError }},
};
use crate::common::{
    DELAY, enums::{
        Action::{*, self}, InputEvent::{*, self}, Cmd, State,
        StoreEvent::*, Style, Color::Reset, Effect, Clear
    }
};

use crate::terminal::store::Store;

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
    id: usize,
    event_rx: Receiver<InputEvent>,
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

    pub fn poll_async(&self) -> Option<InputEvent> {
        let mut iterator = self.event_rx.try_iter();
        iterator.next()
    }

    pub fn poll_latest_async(&self) -> Option<InputEvent> {
        let mut iterator = self.event_rx.try_iter();
        let mut result = Vec::with_capacity(8);
        while let Some(evt) = iterator.next() {
            result.push(evt)
        }
        result.pop()
    }

    pub fn poll_sync(&self) -> Option<InputEvent> {
        let mut iterator = self.event_rx.iter();
        iterator.next()
    }

    // (imdaveho) TODO: convert to specific methods,
    // or ignore certain methods that shouldn't run, like Pos.
    pub fn signal(&self, action: Action) {
        let _ = self.signal_tx.send(Cmd::Signal(action));
    }

    pub fn coord(&self) -> (i16, i16) {
        let _ = self.signal_tx.send(
            Cmd::Request(State::Coord(self.id)));
        let mut iter = self.event_rx.iter();
        loop {
            if let Some(Dispatch(Coord(col, row))) = iter.next() {
                return (col, row)
            }
        }
    }

    pub fn size(&self) -> (i16, i16) {
        let _ = self.signal_tx.send(
            Cmd::Request(State::Size(self.id)));
        let mut iter = self.event_rx.iter();
        loop {
            if let Some(Dispatch(Size(w, h))) = iter.next() {
                return (w, h)
            }
        }
        // (imdvaeho) TODO: this should still call the top
        // since it will pull from the store. But there will
        // have to be a sync with store call to get updated
        // size esp. on SIGWINCH.
        // #[cfg(unix)]
        // let (w, h) = posix::size();
        // #[cfg(windows)]
        // let (w, h) = win32::size();
        // (w, h)
    }

    pub fn syspos(&self) -> (i16, i16) {
        let _ = self.signal_tx.send(
            Cmd::Request(State::SysPos(self.id)));
        let mut iter = self.event_rx.iter();
        loop {
            if let Some(Dispatch(SysPos(col, row))) = iter.next() {
                return (col, row)
            }
        }
    }

    pub fn getch(&self) -> String {
        let _ = self.signal_tx.send(
            Cmd::Request(State::GetCh(self.id)));
        let mut iter = self.event_rx.iter();
        loop {
            if let Some(Dispatch(GetCh(s))) = iter.next() {
                return s
            }
        }
    }
}


struct EventEmitter {
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

        // Fetch terminal default state in main thread.
        #[cfg(unix)]
        let (initial, col, row, tab_size) = fetch_defaults();

        // TODO: Windows --
        // #[cfg(windows)]
        // let initial = win32::get_mode();
        // #[cfg(windows)]
        // let default = win32::get_attrib();

        // #[cfg(unix)]
        // let tab_size = sys_tab_size(&initial);
        // // TODO: Windows?

        // Start signal loop.
        let (signal_tx, signal_rx) = channel();
        let signal_handle = thread::spawn(move || {
            // Initialize the Store.
            let mut store = Store::new();
            store.sync_tab_size(tab_size);
            // Store inits with main screen buffer,
            // so we align the main cursor position.
            store.sync_goto(col, row);

            // Windows *mut c_void cannot be safely moved into thread. So
            // we create it within the thread.
            #[cfg(windows)]
            let screen = win32::Handle::buffer()
                .expect("Error creating alternate Console buffer");
            #[cfg(windows)]
            let vte = win32::is_ansi_enabled();

            while is_running_arc.load(Ordering::SeqCst) {
                // Include minor delay so the thread isn't blindly using CPU.
                thread::sleep(Duration::from_millis(DELAY));
                // Handle signal commands.
                match signal_rx.try_recv() {
                    Ok(cmd) => match cmd {
                        Cmd::Continue => (),
                        Cmd::Suspend(id) => {
                            let mut roster = match emitters_arc.lock() {
                                Ok(r) => r,
                                Err(_) => match emitters_arc.lock() {
                                    Ok(r) => r,
                                    Err(_) => continue
                                },
                            };

                            roster.entry(id)
                                .and_modify(|tx: &mut EventEmitter| {
                                    tx.is_suspend = true
                                });
                        },
                        Cmd::Transmit(id) => {
                            let mut roster = match emitters_arc.lock() {
                                Ok(r) => r,
                                Err(_) => match emitters_arc.lock() {
                                    Ok(r) => r,
                                    Err(_) => continue
                                },
                            };

                            roster.entry(id)
                                .and_modify(|tx: &mut EventEmitter| {
                                    tx.is_suspend = false
                                });
                        },
                        Cmd::Stop(id) => {
                            let mut roster = match emitters_arc.lock() {
                                Ok(r) => r,
                                Err(_) => match emitters_arc.lock() {
                                    Ok(r) => r,
                                    Err(_) => continue
                                },
                            };

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

                                store.sync_goto(col, row);
                            },
                            Up(n) => {
                                #[cfg(unix)]
                                posix::up(n);
                                #[cfg(windows)]
                                win32::up(n, vte);

                                store.sync_up(n);
                            },
                            Down(n) => {
                                #[cfg(unix)]
                                posix::down(n);
                                #[cfg(windows)]
                                win32::down(n, vte);

                                store.sync_down(n);
                            },
                            Left(n) => {
                                #[cfg(unix)]
                                posix::left(n);
                                #[cfg(windows)]
                                win32::left(n, vte);

                                store.sync_left(n);
                            },
                            Right(n) => {
                                #[cfg(unix)]
                                posix::right(n);
                                #[cfg(windows)]
                                win32::right(n, vte);

                                store.sync_right(n);
                            },
                            // SCREEN/OUTPUT
                            Clear(clr) => {
                                #[cfg(unix)]
                                posix::clear(clr);
                                #[cfg(windows)]
                                win32::clear(clr, vte);

                                store.sync_clear(clr);
                            },
                            Prints(s) => {
                                #[cfg(unix)]
                                posix::prints(&s);
                                #[cfg(windows)]
                                win32::prints(&s, vte);

                                store.sync_content(&s);
                            },
                            Printf(s) => {
                                #[cfg(unix)]
                                posix::printf(&s);
                                #[cfg(windows)]
                                win32::printf(&s, vte);

                                store.sync_content(&s);
                            },
                            Flush => {
                                #[cfg(unix)]
                                posix::flush();
                                #[cfg(windows)]
                                win32::flush(vte);
                            },
                            // STYLE
                            SetFx(fx) => {
                                #[cfg(unix)]
                                posix::set_fx(fx);
                                #[cfg(windows)]
                                win32::set_fx(fx, vte);

                                store.sync_style(Style::Fx(fx));
                            },
                            SetFg(c) => {
                                #[cfg(unix)]
                                posix::set_fg(c);
                                #[cfg(windows)]
                                win32::set_fg(c, default, vte);

                                store.sync_style(Style::Fg(c));
                            },
                            SetBg(c) => {
                                #[cfg(unix)]
                                posix::set_bg(c);
                                #[cfg(windows)]
                                win32::set_bg(c, default, vte);

                                store.sync_style(Style::Bg(c));
                            },
                            SetStyles(f, b, fx) => {
                                #[cfg(unix)]
                                posix::set_styles(f, b, fx);
                                #[cfg(windows)]
                                win32::set_styles(f, b, fx, default, vte);

                                store.sync_styles(f, b, fx);
                            },
                            ResetStyles => {
                                #[cfg(unix)]
                                posix::reset_styles();
                                #[cfg(windows)]
                                win32::reset_style(default, vte);

                                let (c, fx) = (Reset, Effect::Reset as u32);
                                store.sync_styles(c, c, fx);
                            },
                            // STATEFUL/MODES
                            HideCursor => {
                                #[cfg(unix)]
                                posix::hide_cursor();
                                #[cfg(windows)]
                                win32::hide_cursor(vte);

                                store.sync_cursor(false);
                            },
                            ShowCursor => {
                                #[cfg(unix)]
                                posix::show_cursor();
                                #[cfg(windows)]
                                win32::show_cursor(vte);

                                store.sync_cursor(true);
                            },
                            EnableMouse => {
                                #[cfg(unix)]
                                posix::enable_mouse();
                                #[cfg(windows)]
                                win32::enable_mouse(vte);

                                store.sync_mouse(true);
                            },
                            DisableMouse => {
                                #[cfg(unix)]
                                posix::disable_mouse();
                                #[cfg(windows)]
                                win32::disable_mouse(vte);

                                store.sync_mouse(false);
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

                                store.sync_raw(true);
                            },
                            Cook => {
                                #[cfg(unix)]
                                posix::cook(&initial);
                                #[cfg(windows)]
                                win32::cook();

                                store.sync_raw(false);
                            },
                            // STORE OPS
                            Refresh => store.refresh(),
                            #[cfg(unix)]
                            Switch => {
                                if store.id() == 0 {
                                    posix::enable_alt();
                                } else {
                                    posix::clear(Clear::All);
                                }
                                store.new_screen();
                                posix::cook(&initial);
                                posix::disable_mouse();
                                posix::show_cursor();
                                posix::reset_styles();
                                posix::goto(0, 0);
                                posix::flush();
                            },
                            #[cfg(unix)]
                            SwitchTo(id) => {
                                let current = store.id();
                                // Bounds checking:
                                if current == id { continue }
                                if store.exists(id) { store.set(id) }
                                else { continue }
                                // Handle screen switch:
                                if id == 0 {
                                    // Disable if you are reverting back to main.
                                    posix::disable_alt();
                                } else {
                                    if current == 0 {
                                        // Enable as you are on the main screen switching to alternate.
                                        posix::enable_alt();
                                    }
                                    posix::clear(Clear::All);
                                    // Restore screen contents. Restore flushes.
                                    let s = store.contents();
                                    posix::goto(0, 0);
                                    posix::prints(&s);
                                    // Restore previous cursor position.
                                    let (col, row) = store.coord();
                                    posix::goto(col, row);
                                    posix::flush();
                                }
                                let (raw, mouse, show) = (
                                    store.is_raw(),
                                    store.is_mouse(),
                                    store.is_cursor(),
                                );
                                // Restore settings based on metadata.
                                if raw { posix::raw() } else { posix::cook(&initial) }
                                if mouse { posix::enable_mouse() } else { posix::disable_mouse() }
                                if show { posix::show_cursor() } else { posix::hide_cursor() }
                                posix::flush();
                            },
                            Resize => {
                                #[cfg(unix)]
                                let (w, h) = posix::size();
                                #[cfg(windows)]
                                let (w, h) = win32::size();

                                store.sync_size(w, h);
                            },
                            SyncSize(w, h) => {
                                #[cfg(unix)]
                                posix::resize(w, h);
                                #[cfg(windows)]
                                win32::resize(w, h, vte);

                                store.sync_size(w, h);
                            },
                            SyncTabSize(n) => store.sync_tab_size(n),
                            SyncMarker(c,r) => store.sync_marker(c,r),
                            Jump => store.jump(),
                        }
                        Cmd::Request(s) => match s {
                            State::Size(id) => {
                                let roster = match emitters_arc.lock() {
                                    Ok(r) => r,
                                    Err(_) => match emitters_arc.lock() {
                                        Ok(r) => r,
                                        Err(_) => continue
                                    },
                                };
                                if let Some(tx) = roster.get(&id) {
                                    let (w, h) = store.size();
                                    let _ = tx.event_tx.send(Dispatch(
                                        Size(w, h)));
                                }
                            },
                            State::Coord(id) => {
                                let roster = match emitters_arc.lock() {
                                    Ok(r) => r,
                                    Err(_) => match emitters_arc.lock() {
                                        Ok(r) => r,
                                        Err(_) => continue
                                    },
                                };
                                if let Some(tx) = roster.get(&id) {
                                    let (col, row) = store.coord();
                                    let _ = tx.event_tx.send(Dispatch(
                                        Coord(col, row)));
                                }
                            },
                            #[cfg(unix)]
                            State::SysPos(id) => {
                                // Lock the receiver that requested syspos:
                                match lock_owner_arc.load(Ordering::SeqCst) {
                                    0 => lock_owner_arc
                                        .store(id, Ordering::SeqCst),
                                    _ => continue,
                                }
                                posix::pos();
                                // Now unlock the receiver after syspos call:
                                match lock_owner_arc.load(Ordering::SeqCst) {
                                    0 => continue,
                                    _ => lock_owner_arc
                                        .store(0, Ordering::SeqCst),
                                }
                            },
                            #[cfg(windows)]
                            State::SysPos(id) => {
                                let roster = match emitters_arc.lock() {
                                    Ok(r) => r,
                                    Err(_) => match emitters_arc.lock() {
                                        Ok(r) => r,
                                        Err(_) => continue
                                    },
                                };
                                if let Some(tx) = roster.get(&id) {
                                    let (col, row) = win32::pos(vte);
                                    let _ = tx.event_tx.send(Dispatch(
                                        Pos(col, row)));
                                }
                            },
                            State::GetCh(id) => {
                                let roster = match emitters_arc.lock() {
                                    Ok(r) => r,
                                    Err(_) => match emitters_arc.lock() {
                                        Ok(r) => r,
                                        Err(_) => continue
                                    },
                                };
                                if let Some(tx) = roster.get(&id) {
                                    // TODO: Windows?
                                    let s = store.getch();
                                    let _ = tx.event_tx.send(Dispatch(
                                        GetCh(s)));
                                }
                            }
                        },
                    },
                    Err(e) => match e {
                        TryRecvError::Empty => (),
                        TryRecvError::Disconnected => is_running_arc
                            .store(false, Ordering::SeqCst),
                    }
                }
            }
            // Shutdown sequence:
            // Reset to terminal defaults.
            // On Unix
            #[cfg(unix)]
            posix::disable_alt();
            #[cfg(unix)]
            posix::show_cursor();
            #[cfg(unix)]
            posix::cook(&initial);
            #[cfg(unix)]
            posix::disable_mouse();
            // On Windows
            #[cfg(windows)]
            win32::disable_alt(vte);
            #[cfg(windows)]
            win32::show_cursor();
            #[cfg(windows)]
            win32::cook();
            #[cfg(windows)]
            win32::disable_mouse();
            // Close the alternate screen on Windows.
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

        // Begin reading user input.
        #[cfg(unix)] {
        self.input_handle = Some(thread::spawn(move || {
            while is_running.load(Ordering::SeqCst) {
                let tty = match fs::OpenOptions::new()
                    .read(true).write(true).open("/dev/tty")
                {
                    Ok(f) => BufReader::new(f),
                    Err(_) => continue
                };

                let (mut input, mut taken) = ([0; 12], tty.take(12));
                let _ = taken.read(&mut input);

                // Emitters clean up.
                let mut roster = match emitters_arc.lock() {
                    Ok(r) => r,
                    Err(_) => match emitters_arc.lock() {
                        Ok(r) => r,
                        Err(_) => continue
                    },
                };
                if !roster.is_empty() {
                    roster.retain( |_, tx: &mut EventEmitter| {
                        tx.is_running
                    })
                }
                // Parse the user input from /dev/tty.
                let item = input[0];
                let mut rest = input[1..].to_vec().into_iter();
                let evt = unix::parser::parse_event(item, &mut rest);
                // Push user input event.
                match lock_owner.load(Ordering::SeqCst) {
                    0 => {
                        for (_, tx) in roster.iter() {
                            if tx.is_suspend { continue }
                            let _ = tx.event_tx.send(evt.clone());
                        }
                    },
                    id => match roster.get(&id) {
                        Some(tx) => { let _ = tx.event_tx.send(evt.clone()); },
                        None => lock_owner.store(0, Ordering::SeqCst),
                    }
                }
                thread::sleep(Duration::from_millis(DELAY));
            }
        }))}

        #[cfg(windows)] {
        self.input_handle = Some(thread::spawn(move || {
            while is_running.load(Ordering::SeqCst) {
                let (_, evts) = windows::parser::read_input_events();
                for evt in evts {
                    // Emitters clean up.
                    let mut roster = match emitters_arc.lock() {
                        Ok(r) => r,
                        Err(_) => match emitters_arc.lock() {
                            Ok(r) => r,
                            Err(_) => continue
                        },
                    };
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
                                let _ = tx.event_tx.send(evt.clone());
                            }
                        },
                        id => match roster.get(&id) {
                            Some(tx) => { let _ = tx.event_tx.send(evt.clone()); },
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
            id: id,
            event_rx: event_rx,
            signal_tx: self.signal_tx.clone(),
        }
    }

    pub fn signal(&self, action: Action) -> Result<(), SendError<Cmd>> {
        self.signal_tx.send(Cmd::Signal(action))
    }

    fn shutdown(&mut self) -> std::thread::Result<()> {
        self.is_running.store(false, Ordering::SeqCst);
        // (imdaveho) TODO: Since reading /dev/tty is blocking
        // we ignore this for now as it will clean up when the
        // program ends (and Dispatcher is dropped). HOWEVER!
        // This should updated when Async/.Await gets released.
        // if let Some(t) = self.input_handle.take() { t.join()? }

        // Clear the emitters registery.
        let lock_err = "Error obtaining emitters lock";
        let mut roster = self.emitters.lock().expect(lock_err);
        roster.clear();
        if let Some(t) = self.signal_handle.take() { t.join()? }

        #[cfg(unix)]
        posix::printf("\r\n");
        #[cfg(windows)]
        let vte = is_ansi_enabled();
        #[cfg(windows)]
        win32::printf("\r\n", vte);

        Ok(())
    }
}

impl Drop for Dispatcher {
    fn drop(&mut self) {
        self.shutdown().expect("Error on shutdown")
    }
}


#[cfg(unix)]
use std::io::{ Write, BufRead };
#[cfg(unix)]
use libc::termios as Termios;

#[cfg(unix)]
fn fetch_defaults() -> (Termios, i16, i16, usize) {
    // Raw mode is needed to fetch cursor report.
    let initial = posix::get_mode();
    posix::raw();
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();

    // Get original position.
    stdout.write_all(b"\x1B[6n").expect("Error writing cursor report");
    stdout.flush().expect("Error flushing cursor report");
    stdin.lock().read_until(b'[', &mut vec![]).expect("Error reading");
    let mut row = vec![];
    stdin.lock().read_until(b';', &mut row).expect("Error reading row");
    let mut col = vec![];
    stdin.lock().read_until(b'R', &mut col).expect("Error reading col");

    row.pop(); col.pop();

    let origin_row: i16 = row.into_iter()
        .map(|b| (b as char))
        .fold(String::new(), |mut acc, n| {
            acc.push(n);
            acc
        }).parse().expect("Error parsing origin row");
    let origin_col: i16 = col.into_iter()
        .map(|b| (b as char))
        .fold(String::new(), |mut acc, n| {
            acc.push(n);
            acc
        }).parse().expect("Error parsing origin col");

    // Get tabbed column position.
    stdout.write_all(b"\t\x1B[6n").expect("Error writing tab cursor report");
    stdout.flush().expect("Error flushing tab cursor report");
    stdin.lock().read_until(b'[', &mut vec![]).expect("Error reading tab");
    let mut row = vec![];
    stdin.lock().read_until(b';', &mut row).expect("Error reading tab row");
    let mut col = vec![];
    stdin.lock().read_until(b'R', &mut col).expect("Error reading tab col");

    col.pop();

    let tabbed_col: i16 = col.into_iter()
        .map(|b| (b as char))
        .fold(String::new(), |mut acc, n| {
            acc.push(n);
            acc
        }).parse().expect("Error parsing tabbed col");
    let tab_size = (tabbed_col - origin_col) as usize;

    // Revert back to original mode.
    posix::cook(&initial);
    posix::printf("\r");

    (initial, origin_col - 1, origin_row - 1, tab_size)
}
