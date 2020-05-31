// Module to handle user input and parse such events from the OS.

#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;
