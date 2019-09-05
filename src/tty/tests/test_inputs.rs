use super::{Tty, InputEvent, KeyEvent, MouseEvent, MouseButton};
use std::thread;
use std::time::Duration;


#[test]
fn test_inputs() {
    let mut tty = Tty::init();

    thread::sleep(Duration::from_secs(2));

    tty.switch();
    tty.raw();
    tty.hide_cursor();

    tty.enable_mouse();
    let mut stdin = tty.read_async();

    loop {
        tty.goto(0, 10);
        if let Some(input) = stdin.next() {
            tty.clear("currentln");
            match input {
                InputEvent::Keyboard(kb) => match kb {
                    KeyEvent::Char(c) => {
                        tty.prints(&format!("Key Pressed: Char({})", c));
                    }

                    KeyEvent::Backspace => {
                        tty.prints("Key Pressed: Backspace");
                    }

                    KeyEvent::Left => {
                        tty.prints("Key Pressed: Left (←)");
                    }

                    KeyEvent::Right => {
                        tty.prints("Key Pressed: Right (→)")
                    }

                    KeyEvent::Up => {
                        tty.prints("Key Pressed: Up (↑)")
                    }

                    KeyEvent::Dn => {
                        tty.prints("Key Pressed: Down (↓)")
                    }

                    KeyEvent::Home => {
                        tty.prints("Key Pressed: Home")
                    }

                    KeyEvent::End => {
                        tty.prints("Key Pressed: End")
                    }

                    KeyEvent::PageUp => {
                        tty.prints("Key Pressed: PageUp")
                    }

                    KeyEvent::PageDn => {
                        tty.prints("Key Pressed: PageDn")
                    }

                    KeyEvent::BackTab => {
                        tty.prints("Key Pressed: BackTab")
                    }

                    KeyEvent::Delete => {
                        tty.prints("Key Pressed: Delete")
                    }

                    KeyEvent::Insert => {
                        tty.prints("Key Pressed: Insert")
                    }

                    KeyEvent::F(n) => {
                        tty.prints(&format!("Key Pressed: F({})", n))
                    }

                    KeyEvent::Alt(c) => {
                        tty.prints(&format!("Key Pressed: Alt({})", c))
                    }

                    KeyEvent::Ctrl(c) => {
                        tty.prints(&format!("Key Pressed: Ctrl({})", c));
                        if c == 'q' {
                            tty.clear("currentln");
                            tty.goto(0, 10);
                            tty.prints("Ctrl(q) pressed. Exiting...");
                            tty.flush();
                            thread::sleep(Duration::from_secs(2));
                            break;
                        }
                    }

                    KeyEvent::Esc => {
                        tty.prints("Key Pressed: Esc")
                    }

                    KeyEvent::CtrlUp => {
                        tty.prints("Key Pressed: CtrlUp")
                    }

                    KeyEvent::CtrlDn => {
                        tty.prints("Key Pressed: CtrlDn")
                    }

                    KeyEvent::CtrlRight => {
                        tty.prints("Key Pressed: CtrlRight")
                    }

                    KeyEvent::CtrlLeft => {
                        tty.prints("Key Pressed: CtrlLeft")
                    }

                    KeyEvent::ShiftUp => {
                        tty.prints("Key Pressed: ShiftUp")
                    }

                    KeyEvent::ShiftDn => {
                        tty.prints("Key Pressed: ShiftDn")
                    }

                    KeyEvent::ShiftRight => {
                        tty.prints("Key Pressed: ShiftRight")
                    }

                    KeyEvent::ShiftLeft => {
                        tty.prints("Key Pressed: ShiftLeft")
                    }

                    KeyEvent::Null => (),
                }

                InputEvent::Mouse(me) => match me {
                    MouseEvent::Press(btn, col, row) => match btn {
                        MouseButton::Left => {
                            tty.prints(&format!(
                                "Mouse Press: Left @ ({}, {})", col, row))
                        }
                        MouseButton::Right => {
                            tty.prints(&format!(
                                "Mouse Press: Right @ ({}, {})", col, row))
                        }
                        MouseButton::Middle => {
                            tty.prints(&format!(
                                "Mouse Press: Middle @ ({}, {})", col, row))
                        }
                        MouseButton::WheelUp => {
                            tty.prints(&format!(
                                "Mouse Press: WheelUp @ ({}, {})", col, row))
                        }
                        MouseButton::WheelDn => {
                            tty.prints(&format!(
                                "Mouse Press: WheelDn @ ({}, {})", col, row))
                        }
                    }

                    MouseEvent::Hold(col, row) => {
                        tty.prints(&format!(
                            "Mouse Hold @ ({}, {})", col, row))
                    }

                    MouseEvent::Release(col, row) => {
                        tty.prints(&format!(
                            "Mouse Release @ ({}, {})", col, row))
                    }
                    MouseEvent::Unknown => (),
                }

                InputEvent::Unknown => (),
                InputEvent::Unsupported(_) => (),
            }
        }
        thread::sleep(Duration::from_millis(16));
        tty.flush();
    }

    tty.to_main();

    tty.prints("Done! Does the cursor show? > ");
    tty.flush();

    thread::sleep(Duration::from_secs(2));

    // tty should terminate at this point when it is dropped
    // tty.terminate();
}
