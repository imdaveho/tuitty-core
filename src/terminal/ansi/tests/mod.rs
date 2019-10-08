#[test]
fn test_ansi_cursor() {
    let goto_string = super::cursor::goto(16, 75);
    assert_eq!(goto_string, "\x1B[76;17H");

    let move_up_string = super::cursor::move_up(8);
    assert_eq!(move_up_string, "\x1B[8A");

    let move_down_string = super::cursor::move_down(8);
    assert_eq!(move_down_string, "\x1B[8B");

    let move_left_string = super::cursor::move_left(8);
    assert_eq!(move_left_string, "\x1B[8D");
    
    let move_right_string = super::cursor::move_right(8);
    assert_eq!(move_right_string, "\x1B[8C");

    // (imdaveho) NOTE: Mark and load positions will be
    // tested visually.
}

#[test]
fn test_ansi_screen() {
    let resize_string = super::screen::resize(40, 90);
    assert_eq!(resize_string, "\x1B[8;90;40t");

    let enable_alt_string = super::screen::enable_alt();
    assert_eq!(enable_alt_string, "\x1B[?1049h");

    let disable_alt_string = super::screen::disable_alt();
    assert_eq!(disable_alt_string, "\x1B[?1049l");

    // (imdaveho) NOTE: Each of these including clear will
    // be tested visually.
}

#[test]
fn test_ansi_style() {
    let reset_string = super::style::reset();
    assert_eq!(reset_string, "\x1B[0m");

    use crate::common::enums::{ Style::*, Color::*, Effect };
    let colors = [
        Black, DarkGrey, 
        Red, DarkRed, 
        Green, DarkGreen,
        Yellow, DarkYellow,
        Blue, DarkBlue,
        Magenta, DarkMagenta,
        Cyan, DarkCyan,
        White, Grey, 
        Reset
    ];
    let fg_color_strings = [
        "\x1B[38;5;0m", "\x1B[38;5;8m", 
        "\x1B[38;5;9m", "\x1B[38;5;1m", 
        "\x1B[38;5;10m", "\x1B[38;5;2m", 
        "\x1B[38;5;11m", "\x1B[38;5;3m", 
        "\x1B[38;5;12m", "\x1B[38;5;4m",
        "\x1B[38;5;13m", "\x1B[38;5;5m",
        "\x1B[38;5;14m", "\x1B[38;5;6m",
        "\x1B[38;5;15m", "\x1B[38;5;7m", 
        ""
    ];
    let bg_color_strings = [
        "\x1B[48;5;0m", "\x1B[48;5;8m", 
        "\x1B[48;5;9m", "\x1B[48;5;1m", 
        "\x1B[48;5;10m", "\x1B[48;5;2m", 
        "\x1B[48;5;11m", "\x1B[48;5;3m", 
        "\x1B[48;5;12m", "\x1B[48;5;4m",
        "\x1B[48;5;13m", "\x1B[48;5;5m",
        "\x1B[48;5;14m", "\x1B[48;5;6m",
        "\x1B[48;5;15m", "\x1B[48;5;7m", 
        ""
    ];
    // Test Foreground Strings
    for (i, c) in colors.iter().enumerate() {
        let fg_string = super::style::set_style(Fg(*c));
        if *c != Reset {
            assert_eq!(fg_string, fg_color_strings[i]);
        } else {
            assert_eq!(fg_string, "\x1B[39m")
        }
    }
    // Test Background Strings
    for (i, c) in colors.iter().enumerate() {
        let bg_string = super::style::set_style(Bg(*c));
        if *c != Reset {
            assert_eq!(bg_string, bg_color_strings[i]);
        } else {
            assert_eq!(bg_string, "\x1B[49m")
        }
    }

    let effect = Effect::Dim as u32;
    let fx_string = super::style::set_style(Fx(effect));
    assert_eq!(fx_string, "\x1B[2m");

    let effects = Effect::Bold | Effect::Underline | Effect::Hide;
    let fxs_string = super::style::set_style(Fx(effects));
    assert_eq!(fxs_string, "\x1B[1m\x1B[4m\x1B[8m");

    let style_string = super::style::set_styles(Yellow, DarkMagenta, effects);
    assert_eq!(style_string, "\x1B[38;5;11m\x1B[48;5;5m\x1B[1m\x1B[4m\x1B[8m");
}

// (imdaveho) NOTE: Mouse will be tested visually.
// (imdaveho) NOTE: AnsiTerminal will be tested visually.

#[test]
fn test_ansi_cache() {
    use crate::common::{
        cache::CacheUpdater,
        unicode::wcwidth::UnicodeWidthStr,
    };
    
    let mut cache = super::cell::CellInfoCache::new();
    let content = "the\x00 \x1B[38;5;9m빨리\x1B[39m 褐色 🦊 jumps over the 大懒 🐕.";
    let content_width = UnicodeWidthStr::width(content);

    assert_eq!(content_width, 52);
    
    let tuitty_width = "the".len() + "".len()
        + " ^[38;5;9m".len() + 4
        + "^[39m ".len() + 4
        + " ".len() + 2
        + " jumps over the ".len() + 4
        + " ".len() + 2 + ".".len();

    assert_eq!(tuitty_width, 54);
    
    let (test_w, test_h) = (40, 40);
    cache._sync_size(test_w, test_h);
    assert_eq!(cache._screen_size(), (test_w, test_h));

    let test_len = (test_w * test_h) as usize;
    assert_eq!(cache.buffer.len(), test_len);

    let (test_col, test_row) = (20, 15);
    cache._sync_pos(test_col, test_row);
    assert_eq!(cache._screen_pos(), (test_col, test_row));

    let start = ((test_row * test_w) + test_col) as usize;
    cache._cache_content(content);

    let mut count_none_until = 0;
    for cell in &cache.buffer {
        match cell {
            Some(_) => break,
            None => count_none_until += 1,
        }
    }
    assert_eq!(count_none_until, start);
    assert!(cache.buffer[tuitty_width].is_none());
    assert_eq!(
        ( (start + tuitty_width) as i16 % test_w, (start + tuitty_width) as i16 / test_w),
        cache._screen_pos() );
    
    cache._sync_tab(8);
    assert_eq!(cache._tab_width(), 8);

    // (imdaveho) NOTE: Movement and Styles will be tested visually.
}