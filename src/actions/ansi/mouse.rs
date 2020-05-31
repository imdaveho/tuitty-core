// ANSI specific functions that enable/disable mouse mode.

pub fn enable_mouse_mode() -> String {
    format!("{}h{}h{}h{}h",
        "\x1B[?1000",
        "\x1B[?1002",
        "\x1B[?1015",
        "\x1B[?1006"
    )
}

pub fn disable_mouse_mode() -> String {
    format!("{}l{}l{}l{}l",
        "\x1B[?1006",
        "\x1B[?1015",
        "\x1B[?1002",
        "\x1B[?1000"
    )
}
