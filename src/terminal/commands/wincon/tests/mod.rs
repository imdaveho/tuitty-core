#[test]
fn test_wincon_cursor() {
    let o = super::cursor::pos().unwrap();
    super::cursor::goto(12, 9).unwrap();
    let p = super::cursor::pos().unwrap();
    assert_eq!((12, 9), p);
    super::cursor::goto(o.0, o.1);

    // (imdaveho) NOTE: Mark, load, and move positions will be
    // tested visually.
}

// (imdaveho) NOTE: Screen, Style, Mouse, Win32Console will be tested visually.

