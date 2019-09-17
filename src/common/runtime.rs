#[cfg(windows)]
use crate::wincon::Handle

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
    }
    #[cfg(windows)]
    else {
        let enable_vt = 0x0004;
        let handle = match Handle::stdout() {
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
        }
    }
    #[cfg(!windows)]
    else { return false }
}

pub fn is_wincon_enabled() -> bool {
    // MinTTY (and alledgedly ConPTY) do not have common support for the native
    // Console API. The MinTTY instance used by `git-bash` emulates over MSYS2,
    // which supports ANSI sequences, but throws an error when tryiing to fetch
    // the default terminal mode from `Termios` (does not exist on Windows) or
    // from the `Handle` (Console API not supported).
    //
    // MSYSTEM environment variable: (stackoverflow)
    // questions/37460073/msys-vs-mingw-internal-environment-variables
    //
    // MinTTY github issue: https://github.com/mintty/mintty/issues/56
    match std::env::var("MSYSTEM") {
        Ok(_) => false, // MSYS, MINGW64, MINGW32
        Err(_) => true, //
    }
}