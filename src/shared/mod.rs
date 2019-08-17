//! Shared code and abstractions.
//! Contains:
//!  * errors
//!  * functions
//!  * macros
//!  * etc.

use std:: {
    fmt::{self, Display, Formatter},
    io,
};

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::{Handle, ConsoleInfo};

/// Tuitty Error Handling
/// ****************************************************************************

pub type TtyResult<T> = std::result::Result<T, TtyErrorKind>;

/// Enumerated errors for `tuitty`.
#[derive(Debug)]
pub enum TtyErrorKind {
    IoError(io::Error),
    FormatError(fmt::Error),
    ResizingError(String),

    #[doc(hidden)]
    __Nonexhaustive,
}

impl std::error::Error for TtyErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            TtyErrorKind::IoError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl Display for TtyErrorKind {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            TtyErrorKind::IoError(_) => write!(fmt, "IO-error occurred"),
            TtyErrorKind::ResizingError(_) => {
                write!(fmt, "Cannot resize the screen")
            },
            _ => write!(fmt, "Some error has occurred"),
        }
    }
}

impl From<io::Error> for TtyErrorKind {
    fn from(e: io::Error) -> TtyErrorKind {
        TtyErrorKind::IoError(e)
    }
}

impl From<fmt::Error> for TtyErrorKind {
    fn from(e: fmt::Error) -> TtyErrorKind {
        TtyErrorKind::FormatError(e)
    }
}

impl From<TtyErrorKind> for io::Error {
    fn from(e: TtyErrorKind) -> io::Error {
        match e {
            TtyErrorKind::IoError(io) => return io,
            _ => io::Error::new(io::ErrorKind::Other,
                                "cannot convert error to IO error"),
        }
    }
}


/// Tuitty Macros
/// ****************************************************************************

/// Append a the first few characters of an ANSI escape code to the given string.
#[macro_export]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

/// Write a string to standard output whereafter the screen will be flushed.
#[macro_export]
macro_rules! write_cout {
    ($string:expr) => {{
        let stdout = ::std::io::stdout();
        let mut stdout = stdout.lock();
        let mut size = 0;

        let result = stdout.write($string.as_bytes());

        size += match result {
            Ok(size) => size,
            Err(e) => return Err(crate::shared::TtyErrorKind::IoError(e)),
        };

        match stdout.flush() {
            Ok(_) => Ok(size),
            Err(e) => Err(crate::shared::TtyErrorKind::IoError(e)),
        }
    }};
}
