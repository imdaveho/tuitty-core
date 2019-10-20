// This module exposes terminal functions 

pub mod ansi;
#[cfg(windows)]
pub mod wincon;


pub fn is_ansi_enabled() -> bool {
    const TERMS: [&'static str; 15] = [
        "xterm",  // xterm, PuTTY, Mintty
        "rxvt",   // RXVT
        "eterm",  // Eterm
        "screen", // GNU screen, tmux
        "tmux",   // tmux
        "vt100", "vt102", "vt220", "vt320",   // DEC VT series
        "ansi",    // ANSI
        "scoansi", // SCO ANSI
        "cygwin",  // Cygwin, MinGW
        "linux",   // Linux console
        "konsole", // Konsole
        "bvterm",  // Bitvise SSH Client
    ];

    let matched_terms = match std::env::var("TERM") {
        Ok(val) => val != "dumb" || TERMS.contains(&val.as_str()),
        Err(_) => false,
    };

    if matched_terms {
        return true
    } else {
        #[cfg(windows)] {
        let enable_vt = 0x0004;
        let handle = match wincon::windows::Handle::stdout() {
            Ok(h) => h,
            Err(_) => return false,
        };
        let mode = match handle.get_mode() {
            Ok(m) => m,
            Err(_) => return false,
        };
        match handle.set_mode(&(mode | enable_vt)) {
            Ok(_) => return true,
            Err(_) => return false,
        }}
        #[cfg(not(windows))]
        return false
    }
}