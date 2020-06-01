#[cfg(unix)]
mod posix;
#[cfg(windows)]
mod win32;

#[cfg(unix)]
pub use posix::Term;
#[cfg(windows)]
pub use win32::Term;