import os
import sys
import platform
from enum import Enum
from ctypes import c_bool, c_char_p, c_int, c_uint8, c_int16, c_uint32, cdll, \
    Structure, POINTER, byref


prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
root = os.path.dirname(os.path.dirname(__file__))
system = {"darwin": "macos", "win32": "windows"}.get(sys.platform, "linux")
bits = "64" if platform.architecture()[0] == "64bit" else "32"
cpu = {
    "aarch64": "arm",
    "x86_64": "intel",
    "i686": "intel",
    "AMD64": "amd"
}.get(platform.machine(), "arm")

path = os.path.join(
    os.path.abspath(root), "lib", system, cpu + bits,
    prefix + "tuitty" + extension)
lib = cdll.LoadLibrary(path)


class CTty(Structure):
    pass


class CSyncReader(Structure):
    pass


class CAsyncReader(Structure):
    pass


class Event(Structure):
    _fields_ = [("_kind", c_int16), ("_data", c_uint32)]

    def kind(self) -> 'InputEvent':
        return InputEvent(self._kind)

    def data(self):
        if self._kind in (14, 15, 16):
            return chr(self._data)

        elif self._kind in (26, 27, 28, 29, 30, 31, 32):
            return ((self._data >> 16), (self._data & 0xffff))

        else:
            return self._data

    def __str__(self):
        if self._kind not in (13, 14, 15, 16, 26, 27, 28, 29, 30, 31, 32):
            return "Key: {}".format(InputEvent(self._kind).name)

        elif self._kind in (13, 14, 15, 16):
            return "Key: {}({})".format(
                InputEvent(self._kind).name, self.data())

        elif self._kind in (26, 27, 28, 29, 30, 31, 32):
            s = {
                26: "Button-Left",
                27: "Button-Right",
                28: "Button-Middle",
                29: "Button-WheelUp",
                30: "Button-WheelDn",
                31: "Hold",
                32: "Release",
            }.get(self._kind)
            return "Mouse: {} @ ({}, {})".format(s, *self.data())


class InputEvent(Enum):
    Null = 0
    Backspace = 1
    Left = 2
    Right = 3
    Up = 4
    Dn = 5
    Home = 6
    End = 7
    PageUp = 8
    PageDn = 9
    BackTab = 10
    Delete = 11
    Insert = 12
    F = 13
    Char = 14
    Alt = 15
    Ctrl = 16
    Esc = 17
    CtrlUp = 18
    CtrlDn = 19
    CtrlRight = 20
    CtrlLeft = 21
    ShiftUp = 22
    ShiftDn = 23
    ShiftRight = 24
    ShiftLeft = 25
    MousePressLeft = 26
    MousePressRight = 27
    MousePressMiddle = 28
    MousePressWheelUp = 29
    MousePressWheelDn = 30
    MouseHold = 31
    MouseRelease = 32


# Tty.init()
lib.init.restype = POINTER(CTty)

# Tty.terminate()
lib.terminate.argtypes = (POINTER(CTty), )

# Tty.size()
lib.size.argtypes = (POINTER(CTty), )
lib.size.restype = c_uint32

# Tty.raw()
lib.raw.argtypes = (POINTER(CTty), )

# Tty.cook()
lib.cook.argtypes = (POINTER(CTty), )

# Tty.enable_mouse()
lib.enable_mouse.argtypes = (POINTER(CTty), )

# Tty.disable_mouse()
lib.disable_mouse.argtypes = (POINTER(CTty), )

# Tty.read_char()
lib.read_char.argtypes = (POINTER(CTty), )
lib.read_char.restype = c_uint32

# Tty.read_sync()
lib.read_sync.argtypes = (POINTER(CTty), )
lib.read_sync.restype = POINTER(CSyncReader)

# SyncReader.next()
lib.sync_next.argtypes = (POINTER(CSyncReader), POINTER(Event))

# drop(SyncReader)
lib.sync_free.argtypes = (POINTER(CSyncReader), )

# Tty.read_async()
lib.read_async.argtypes = (POINTER(CTty), )
lib.read_async.restype = POINTER(CAsyncReader)

# Tty.read_until_async()
lib.read_until_async.argtypes = (POINTER(CTty), c_int)
lib.read_until_async.restype = POINTER(CAsyncReader)

# AsyncReader.next()
lib.async_next.argtypes = (POINTER(CAsyncReader), POINTER(Event))
lib.async_next.restype = c_bool

# drop(AsyncReader)
lib.async_free.argtypes = (POINTER(CAsyncReader), )

# Tty.clear()
lib.clear.argtypes = (POINTER(CTty), c_char_p)

# Tty.resize()
lib.resize.argtypes = (POINTER(CTty), c_int16, c_int16)

# Tty.switch()
lib.switch.argtypes = (POINTER(CTty), )

# Tty.to_main()
lib.to_main.argtypes = (POINTER(CTty), )

# Tty.switch_to()
lib.switch_to.argtypes = (POINTER(CTty), c_int)

# Tty.goto()
lib.goto.argtypes = (POINTER(CTty), c_int16, c_int16)

# Tty.up()
lib.up.argtypes = (POINTER(CTty), )

# Tty.dn()
lib.dn.argtypes = (POINTER(CTty), )

# Tty.left()
lib.left.argtypes = (POINTER(CTty), )

# Tty.right()
lib.right.argtypes = (POINTER(CTty), )

# Tty.dpad()
lib.dpad.argtypes = (POINTER(CTty), c_char_p, c_int16)

# Tty.pos()
lib.pos.argtypes = (POINTER(CTty), )
lib.pos.restype = c_uint32

# Tty.mark()
lib.mark.argtypes = (POINTER(CTty), )

# Tty.load()
lib.load.argtypes = (POINTER(CTty), )

# Tty.hide_cursor()
lib.hide_cursor.argtypes = (POINTER(CTty), )

# Tty.show_cursor()
lib.show_cursor.argtypes = (POINTER(CTty), )

# Tty.set_fg()
lib.set_fg.argtypes = (POINTER(CTty), c_char_p)

# Tty.set_bg()
lib.set_bg.argtypes = (POINTER(CTty), c_char_p)

# Tty.aset_tx()
lib.set_tx.argtypes = (POINTER(CTty), c_char_p)

# Tty.set_fg_rgb()
lib.set_fg_rgb.argtypes = (POINTER(CTty), c_uint8, c_uint8, c_uint8)

# Tty.set_bg_rgb()
lib.set_bg_rgb.argtypes = (POINTER(CTty), c_uint8, c_uint8, c_uint8)

# Tty.set_fg_ansi()
lib.set_fg_ansi.argtypes = (POINTER(CTty), c_uint8)

# Tty.set_bg_ansi()
lib.set_bg_ansi.argtypes = (POINTER(CTty), c_uint8)

# Tty.set_style()
lib.set_style.argtypes = (POINTER(CTty), c_char_p, c_char_p, c_char_p)

# Tty.reset()
lib.reset.argtypes = (POINTER(CTty), )

# Tty.prints()
lib.prints.argtypes = (POINTER(CTty), c_char_p)

# Tty.flush()
lib.flush.argtypes = (POINTER(CTty), )


class Tty:
    def __init__(self):
        self.tty = lib.init()

    def __enter__(self):
        return self

    def __exit__(self, exception_type, exception_value, traceback):
        lib.terminate(self.tty)

    def terminate(self):
        lib.terminate(self.tty)

    def size(self) -> (int, int):
        size = lib.size(self.tty)
        return ((size >> 16), (size & 0xffff))

    def raw(self):
        lib.raw(self.tty)

    def cook(self):
        lib.cook(self.tty)

    def enable_mouse(self):
        lib.enable_mouse(self.tty)

    def disable_mouse(self):
        lib.disable_mouse(self.tty)

    def read_char(self) -> str:
        return chr(lib.read_char(self.tty))

    def read_sync(self) -> 'SyncInput':
        return SyncInput(self.tty)

    def read_async(self) -> 'AsyncInput':
        return AsyncInput(self.tty)

    def clear(self, method: str):
        lib.clear(self.tty, method.encode('utf-8'))

    def resize(self, w: int, h: int):
        lib.resize(self.tty, w, h)

    def switch(self):
        lib.switch(self.tty)

    def to_main(self):
        lib.to_main(self.tty)

    def switch_to(self, index: int):
        lib.switch_to(self.tty, index)

    def goto(self, col: int, row: int):
        lib.goto(self.tty, col, row)

    def up(self):
        lib.up(self.tty)

    def dn(self):
        lib.dn(self.tty)

    def left(self):
        lib.left(self.tty)

    def right(self):
        lib.right(self.tty)

    def dpad(self, direction: str, n: int):
        lib.dpad(self.tty, direction.encode('utf-8'), n)

    def pos(self) -> (int, int):
        coord = lib.pos(self.tty)
        return ((coord >> 16), (coord & 0xffff))

    def mark(self):
        lib.mark(self.tty)

    def load(self):
        lib.load(self.tty)

    def hide_cursor(self):
        lib.hide_cursor(self.tty)

    def show_cursor(self):
        lib.show_cursor(self.tty)

    def set_fg(self, color: str):
        lib.set_fg(self.tty, color.encode('utf-8'))

    def set_bg(self, color: str):
        lib.set_bg(self.tty, color.encode('utf-8'))

    def set_tx(self, style: str):
        lib.set_tx(self.tty, style.encode('utf-8'))

    def set_fg_rgb(self, r: int, g: int, b: int):
        lib.set_fg_rgb(self.tty, r, g, b)

    def set_bg_rgb(self, r: int, g: int, b: int):
        lib.set_bg_rgb(self.tty, r, g, b)

    def set_fg_ansi(self, v: int):
        lib.set_fg_ansi(self.tty, v)

    def set_bg_ansi(self, v: int):
        lib.set_bg_ansi(self.tty, v)

    def set_style(self, fg: str, bg: str, style: str):
        lib.set_style(self.tty,
                      fg.encode('utf-8'),
                      bg.encode('utf-8'),
                      style.encode('utf-8'))

    def reset(self):
        lib.reset(self.tty)

    def prints(self, s: str):
        lib.prints(self.tty, s.encode('utf-8'))

    def flush(self):
        lib.flush(self.tty)


class SyncInput:
    def __init__(self, tty: 'Tty'):
        self.obj: POINTER('CSyncReader') = lib.read_sync(tty)
        self.event: 'Event' = Event()

    def __enter__(self):
        return self

    def __iter__(self):
        return self

    def __next__(self):
        lib.sync_next(self.obj, byref(self.event))  # blocking call

    def __exit__(self, exception_type, exception_value, traceback):
        lib.sync_free(self.obj)

    def close(self):
        lib.sync_free(self.obj)


class AsyncInput:
    def __init__(self, tty: 'Tty', delimiter=None):
        if delimiter is not None:
            self.obj: POINTER('CAsyncReader') = \
                lib.read_until_async(tty, delimiter)
        else:
            self.obj: POINTER('CAsyncReader') = lib.read_async(tty)
        self.event: 'Event' = Event()

    def __enter__(self):
        return self

    def __iter__(self):
        return self

    def __next__(self) -> bool:
        return lib.async_next(self.obj, byref(self.event))  # non-blocking call

    def __exit__(self, exception_type, exception_value, traceback):
        lib.async_free(self.obj)

    def close(self):
        lib.async_free(self.obj)
