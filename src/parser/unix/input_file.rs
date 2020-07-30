// Unix specific implementation to read from stdin or /dev/tty. Implements mio
// and signal-hook to replicate the winapi Handler to react to windows resizing.

use std::{ fs, io, os::unix::io::{ IntoRawFd, RawFd }};
use std::io::Result;
use std::time::Duration;
use signal_hook::iterator::Signals;
use mio::{ unix::SourceFd, Events, Interest, Poll, Token };

use super::InputEvent;


const TTY_TOKEN: Token = Token(0);
const SIG_TOKEN: Token = Token(1);

// NOTE: Previous implementation:
// let (mut input, mut taken) = ()[0; 12], std::io::Read::take(tty, 12));
// let _ = std::io::Read::read(&mut taken, &mut input);
// NOTE: seems that 12 is not sufficient; dragging and releasing requires 23.
const READ_SIZE: usize = 32;


struct FileDesc {
    fd: RawFd,
    close_on_drop: bool,
}

impl FileDesc {
    fn new(fd: RawFd, close_on_drop: bool) -> Self {
        Self { fd, close_on_drop }
    }

    fn read(&self, buffer: &mut [u8], size: usize) -> Result<usize> {
        let result = unsafe {
            libc::read(
                self.fd,
                buffer.as_mut_ptr() as *mut libc::c_void,
                size as libc::size_t,
            ) as isize
        };

        if result < 0 {
            // Err(ErrorKind::IoError(io::Error::last_os_error()))
            Err(io::Error::last_os_error())
        } else {
            Ok(result as usize)
        }
    }

    fn raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl Drop for FileDesc {
    fn drop(&mut self) {
        if self.close_on_drop {
            let _ = unsafe { libc::close(self.fd) };
        }
    }
}

fn tty_fd() -> Result<FileDesc> {
    if unsafe { libc::isatty(libc::STDIN_FILENO) == 1 } {
        Ok(FileDesc::new(libc::STDIN_FILENO, false))
    } else {
        Ok(FileDesc::new(
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/tty")?
            .into_raw_fd(), true
        ))
    }
}



pub struct UnixHandle {
    poll: Poll,
    events: Events,
    signals: Signals,
    buffer: [u8; READ_SIZE],
    fd: FileDesc,
}

impl UnixHandle {
    pub fn new() -> Result<Self> {
        let poll = Poll::new()?;
        let registry = poll.registry();

        // Register stdin/tty
        let input_fd = tty_fd()?;
        let raw_fd = input_fd.raw_fd();
        let mut input_evts = SourceFd(&raw_fd);
        registry.register(&mut input_evts, TTY_TOKEN, Interest::READABLE)?;

        // Register sigwinch
        let mut signals = Signals::new(&[signal_hook::SIGWINCH])?;
        registry.register(&mut signals, SIG_TOKEN, Interest::READABLE)?;

        // Setup remaining fields
        let events = Events::with_capacity(3);
        let buffer = [0u8; READ_SIZE];

        Ok(Self { poll, events, signals, buffer, fd: input_fd })
    }

    pub fn read_input_events(&mut self) -> InputEvent {
        let timeout = Some(Duration::from_millis(0));
        if let Err(_) = self.poll.poll(&mut self.events, timeout) {
            return InputEvent::Error
        }

        if self.events.is_empty() { return InputEvent::Empty }

        for evt in self.events.iter() {
            match evt.token() {
                TTY_TOKEN => match self.fd.read(&mut self.buffer, READ_SIZE) {
                    Ok(read_count) => {
                        // NOTE: debugging max READ_SIZE
                        // println!("{}\r", read_count);
                        if read_count > 0 {
                            let input = &self.buffer[..read_count];
                            let item = input[0];
                            let mut rest = input[1..].to_vec().into_iter();
                            return super::parse_event(item, &mut rest);
                        }
                    },
                    Err(_) => return InputEvent::Error
                },
                SIG_TOKEN => for signal in &self.signals {
                    match signal {
                        signal_hook::SIGWINCH => {
                            // TODO: fetch size and return
                            return InputEvent::WinResize(0, 0)
                        },
                        _ => (),
                    }
                },
                _ => unreachable!(),
            }
        }
        InputEvent::Unsupported
    }
}
