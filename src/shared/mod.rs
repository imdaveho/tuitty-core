// Shared code and abstractions that is leveraged by the other modules.
//
// For ANSI, there are macros to help with writing escape sequences to stdout.
//
// For WinCon, we needed a way to wrap the methods to attain pointers to the
// Handle object for terminal operations.


#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::{Handle, ConsoleInfo};


/// ANSI Macros
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
            Err(e) => return Err(e),
        };

        match stdout.flush() {
            Ok(_) => Ok(size),
            Err(e) => Err(e),
        }
    }};
}
