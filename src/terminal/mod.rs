mod ansi;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod wincon;

#[cfg(windows)]
mod windows;