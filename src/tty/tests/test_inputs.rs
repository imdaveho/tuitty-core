#[test]
fn test_inputs() {
    let mut tty = super::Tty::init();

    use std::time::Duration;
    use std::thread;

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
                super::InputEvent::Keyboard(kb) => match kb {
                    super::KeyEvent::Char(c) => {
                        tty.prints(&format!("Key Pressed: Char({})", c));
                    }

                    super::KeyEvent::Backspace => {
                        tty.prints("Key Pressed: Backspace");
                    }

                    super::KeyEvent::Left => {
                        tty.prints("Key Pressed: Left (←)");
                    }

                    super::KeyEvent::Right => {
                        tty.prints("Key Pressed: Right (→)")
                    }

                    super::KeyEvent::Up => {
                        tty.prints("Key Pressed: Up (↑)")
                    }

                    super::KeyEvent::Dn => {
                        tty.prints("Key Pressed: Down (↓)")
                    }

                    super::KeyEvent::Home => {
                        tty.prints("Key Pressed: Home")
                    }

                    super::KeyEvent::End => {
                        tty.prints("Key Pressed: End")
                    }

                    super::KeyEvent::PageUp => {
                        tty.prints("Key Pressed: PageUp")
                    }

                    super::KeyEvent::PageDn => {
                        tty.prints("Key Pressed: PageDn")
                    }

                    super::KeyEvent::BackTab => {
                        tty.prints("Key Pressed: BackTab")
                    }

                    super::KeyEvent::Delete => {
                        tty.prints("Key Pressed: Delete")
                    }

                    super::KeyEvent::Insert => {
                        tty.prints("Key Pressed: Insert")
                    }

                    super::KeyEvent::F(n) => {
                        tty.prints(&format!("Key Pressed: F({})", n))
                    }

                    super::KeyEvent::Alt(c) => {
                        tty.prints(&format!("Key Pressed: Alt({})", c))
                    }

                    super::KeyEvent::Ctrl(c) => {
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

                    super::KeyEvent::Esc => {
                        tty.prints("Key Pressed: Esc")
                    }

                    super::KeyEvent::CtrlUp => {
                        tty.prints("Key Pressed: CtrlUp")
                    }

                    super::KeyEvent::CtrlDn => {
                        tty.prints("Key Pressed: CtrlDn")
                    }

                    super::KeyEvent::CtrlRight => {
                        tty.prints("Key Pressed: CtrlRight")
                    }

                    super::KeyEvent::CtrlLeft => {
                        tty.prints("Key Pressed: CtrlLeft")
                    }

                    super::KeyEvent::ShiftUp => {
                        tty.prints("Key Pressed: ShiftUp")
                    }

                    super::KeyEvent::ShiftDn => {
                        tty.prints("Key Pressed: ShiftDn")
                    }

                    super::KeyEvent::ShiftRight => {
                        tty.prints("Key Pressed: ShiftRight")
                    }

                    super::KeyEvent::ShiftLeft => {
                        tty.prints("Key Pressed: ShiftLeft")
                    }

                    super::KeyEvent::Null => (),
                }

                super::InputEvent::Mouse(me) => match me {
                    super::MouseEvent::Press(btn, col, row) => match btn {
                        super::MouseButton::Left => {
                            tty.prints(&format!(
                                "Mouse Press: Left @ ({}, {})", col, row))
                        }
                        super::MouseButton::Right => {
                            tty.prints(&format!(
                                "Mouse Press: Right @ ({}, {})", col, row))
                        }
                        super::MouseButton::Middle => {
                            tty.prints(&format!(
                                "Mouse Press: Middle @ ({}, {})", col, row))
                        }
                        super::MouseButton::WheelUp => {
                            tty.prints(&format!(
                                "Mouse Press: WheelUp @ ({}, {})", col, row))
                        }
                        super::MouseButton::WheelDn => {
                            tty.prints(&format!(
                                "Mouse Press: WheelDn @ ({}, {})", col, row))
                        }
                    }

                    super::MouseEvent::Hold(col, row) => {
                        tty.prints(&format!(
                            "Mouse Hold @ ({}, {})", col, row))
                    }

                    super::MouseEvent::Release(col, row) => {
                        tty.prints(&format!(
                            "Mouse Release @ ({}, {})", col, row))
                    }
                    super::MouseEvent::Unknown => (),
                }

                super::InputEvent::Unknown => (),
                super::InputEvent::Unsupported(_) => (),
            }
        }
        thread::sleep(Duration::from_millis(16));
        tty.flush();
    }

    tty.to_main();

    tty.prints("Done! Does the cursor show? > ");
    tty.flush();

    thread::sleep(Duration::from_secs(2));

    tty.terminate();
}
