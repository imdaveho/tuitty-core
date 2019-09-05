import unittest
import time
from enum import Enum
from tuitty.ffi import Tty


class TestInputs(unittest.TestCase):
    def test_inputs(self):
        with Tty() as tty:
            time.sleep(2)

            tty.switch()
            tty.raw()
            tty.hide_cursor()

            tty.enable_mouse()
            stdin = tty.read_async()

            while True:
                time.sleep(1./60)
                tty.goto(0, 10)
                tty.flush()
                # TODO: make AsyncInput and SyncInput into iterators
                contains = next(stdin)
                if not contains:
                    continue
                tty.clear("currentln")
                ev = stdin.evt
                if InputEvent(ev.kind) == InputEvent.Keyboard:
                    if KeyEvent(ev.label) == KeyEvent.Char:
                        tty.prints("Key Pressed: Char({})".format(chr(ev.ch)))
                    elif KeyEvent(ev.label) == KeyEvent.Backspace:
                        tty.prints("Key Pressed: Backspace")
                    elif KeyEvent(ev.label) == KeyEvent.Left:
                        tty.prints("Key Pressed: Left (←)")
                    elif KeyEvent(ev.label) == KeyEvent.Right:
                        tty.prints("Key Pressed: Right (→)")
                    elif KeyEvent(ev.label) == KeyEvent.Up:
                        tty.prints("Key Pressed: Up (↑)")
                    elif KeyEvent(ev.label) == KeyEvent.Dn:
                        tty.prints("Key Pressed: Down (↓)")
                    elif KeyEvent(ev.label) == KeyEvent.Home:
                        tty.prints("Key Pressed: Home")
                    elif KeyEvent(ev.label) == KeyEvent.End:
                        tty.prints("Key Pressed: End")
                    elif KeyEvent(ev.label) == KeyEvent.PageUp:
                        tty.prints("Key Pressed: PageUp")
                    elif KeyEvent(ev.label) == KeyEvent.PageDn:
                        tty.prints("Key Pressed: PageDn")
                    elif KeyEvent(ev.label) == KeyEvent.BackTab:
                        tty.prints("Key Pressed: BackTab")
                    elif KeyEvent(ev.label) == KeyEvent.Delete:
                        tty.prints("Key Pressed: Delete")
                    elif KeyEvent(ev.label) == KeyEvent.Insert:
                        tty.prints("Key Pressed: Insert")
                    elif KeyEvent(ev.label) == KeyEvent.F:
                        tty.prints("Key Pressed: F({})".format(ev.ch))
                    elif KeyEvent(ev.label) == KeyEvent.Alt:
                        tty.prints("Key Pressed: Alt({})".format(chr(ev.ch)))
                    elif KeyEvent(ev.label) == KeyEvent.Ctrl:
                        tty.prints("Key Pressed: Ctrl({})".format(chr(ev.ch)))

                        if chr(ev.ch) == 'q':
                            tty.clear("currentln")
                            tty.goto(0, 10)
                            tty.prints("Ctrl(q) pressed. Exiting...")
                            tty.flush()
                            time.sleep(2)
                            break

                    elif KeyEvent(ev.label) == KeyEvent.Esc:
                        tty.prints("Key Pressed: Esc")
                    elif KeyEvent(ev.label) == KeyEvent.CtrlUp:
                        tty.prints("Key Pressed: CtrlUp")
                    elif KeyEvent(ev.label) == KeyEvent.CtrlDn:
                        tty.prints("Key Pressed: CtrlDn")
                    elif KeyEvent(ev.label) == KeyEvent.CtrlRight:
                        tty.prints("Key Pressed: CtrlRight")
                    elif KeyEvent(ev.label) == KeyEvent.CtrlLeft:
                        tty.prints("Key Pressed: CtrlLeft")
                    elif KeyEvent(ev.label) == KeyEvent.ShiftUp:
                        tty.prints("Key Pressed: ShiftUp")
                    elif KeyEvent(ev.label) == KeyEvent.ShiftDn:
                        tty.prints("Key Pressed: ShiftDn")
                    elif KeyEvent(ev.label) == KeyEvent.ShiftRight:
                        tty.prints("Key Pressed: ShiftRight")
                    elif KeyEvent(ev.label) == KeyEvent.ShiftLeft:
                        tty.prints("Key Pressed: ShiftLeft")
                    else:
                        pass  # Null
                elif InputEvent(ev.kind) == InputEvent.Mouse:
                    if MouseEvent(ev.label) == MouseEvent.Press:
                        if MouseButton(ev.btn) == MouseButton.Left:
                            tty.prints(
                                "Mouse Press: Left @ ({}, {})"
                                .format(ev.coord.col, ev.coord.row))
                        elif MouseButton(ev.btn) == MouseButton.Right:
                            tty.prints(
                                "Mouse Press: Right @ ({}, {})"
                                .format(ev.coord.col, ev.coord.row))
                        elif MouseButton(ev.btn) == MouseButton.Middle:
                            tty.prints(
                                "Mouse Press: Middle @ ({}, {})"
                                .format(ev.coord.col, ev.coord.row))
                        elif MouseButton(ev.btn) == MouseButton.WheelUp:
                            tty.prints(
                                "Mouse Press: WheelUp @ ({}, {})"
                                .format(ev.coord.col, ev.coord.row))
                        elif MouseButton(ev.btn) == MouseButton.WheelDn:
                            tty.prints(
                                "Mouse Press: WheelDn @ ({}, {})"
                                .format(ev.coord.col, ev.coord.row))
                    elif MouseEvent(ev.label) == MouseEvent.Hold:
                        tty.prints("Mouse Hold @ ({}, {})"
                                   .format(ev.coord.col, ev.coord.row))
                    elif MouseEvent(ev.label) == MouseEvent.Release:
                        tty.prints("Mouse Release @ ({}, {})"
                                   .format(ev.coord.col, ev.coord.row))
                    else:
                        pass  # Unknown
                else:
                    pass  # Unknown
            # end loop
            tty.to_main()

            tty.prints("Done! Does the cursor show? > ")
            tty.flush()

            time.sleep(2)
        # tty is dropped here


class InputEvent(Enum):
    Keyboard = 0
    Mouse = 1
    Unknown = 2


class KeyEvent(Enum):
    Backspace = 0
    Left = 1
    Right = 2
    Up = 3
    Dn = 4
    Home = 5
    End = 6
    PageUp = 7
    PageDn = 8
    BackTab = 9
    Delete = 10
    Insert = 11
    F = 12
    Char = 13
    Alt = 14
    Ctrl = 15
    # Null
    Esc = 16
    CtrlUp = 17
    CtrlDn = 18
    CtrlRight = 19
    CtrlLeft = 20
    ShiftUp = 21
    ShiftDn = 22
    ShiftRight = 23
    ShiftLeft = 24


class MouseEvent(Enum):
    Press = 0
    Hold = 1
    Release = 2


class MouseButton(Enum):
    Left = 0
    Right = 1
    Middle = 2
    WheelUp = 3
    WheelDn = 4
