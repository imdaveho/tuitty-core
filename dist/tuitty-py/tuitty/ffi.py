import os
import sys
import platform
from ctypes import c_bool, c_char_p, c_int, c_uint8, c_int16, c_uint32, cdll, \
    Structure, POINTER


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


class CSize(Structure):
    _fields_ = [("w", c_int16), ("h", c_int16)]

    def __str__(self):
        return "({}, {})".format(self.w, self.h)


class CCoord(Structure):
    _fields_ = [("col", c_int16), ("row", c_int16)]

    def __str__(self):
        return "({}, {})".format(self.col, self.row)


class CSyncInput(Structure):
    pass


class CAsyncInput(Structure):
    pass


# Tty.init()
lib.init.restype = POINTER(CTty)

# Tty.terminate()
lib.terminate.argtypes = (POINTER(CTty), )

# Tty.size()
lib.size.argtypes = (POINTER(CTty), )
lib.size.restype = CSize

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
lib.read_sync.restype = POINTER(CSyncInput)

# SyncInput.SyncReader.next()
lib.sync_next.argtypes = (POINTER(CSyncInput), )

# SyncInput.Event.kind
lib.get_sync_kind.argtypes = (POINTER(CSyncInput), )
lib.get_sync_kind.restype = c_uint8

# SyncInput.Event.label
lib.get_sync_label.argtypes = (POINTER(CSyncInput), )
lib.get_sync_label.restype = c_uint8

# SyncInput.Event.btn
lib.get_sync_btn.argtypes = (POINTER(CSyncInput), )
lib.get_sync_btn.restype = c_uint8

# SyncInput.Event.coord
lib.get_sync_coord.argtypes = (POINTER(CSyncInput), )
lib.get_sync_coord.restype = CCoord

# SyncInput.Event.ch
lib.get_sync_ch.argtypes = (POINTER(CSyncInput), )
lib.get_sync_ch.restype = c_uint32

# drop(SyncInput)
lib.sync_free.argtypes = (POINTER(CSyncInput), )

# Tty.read_async()
lib.read_async.argtypes = (POINTER(CTty), )
lib.read_async.restype = POINTER(CAsyncInput)

# Tty.read_until_async()
lib.read_until_async.argtypes = (POINTER(CTty), c_int)
lib.read_until_async.restype = POINTER(CAsyncInput)

# AsyncInput.AsyncReader.next()
lib.async_next.argtypes = (POINTER(CAsyncInput), )
lib.async_next.restype = c_bool

# AsyncInput.Event.kind
lib.get_async_kind.argtypes = (POINTER(CAsyncInput), )
lib.get_async_kind.restype = c_uint8

# AsyncInput.Event.label
lib.get_async_label.argtypes = (POINTER(CAsyncInput), )
lib.get_async_label.restype = c_uint8

# AsyncInput.Event.btn
lib.get_async_btn.argtypes = (POINTER(CAsyncInput), )
lib.get_async_btn.restype = c_uint8

# AsyncInput.Event.coord
lib.get_async_coord.argtypes = (POINTER(CAsyncInput), )
lib.get_async_coord.restype = CCoord

# AsyncInput.Event.ch
lib.get_async_ch.argtypes = (POINTER(CAsyncInput), )
lib.get_async_ch.restype = c_uint32

# drop(AsyncInput)
lib.async_free.argtypes = (POINTER(CAsyncInput), )

# Tty.clear()
lib.clear.argtypes = (POINTER(CTty), c_uint8)

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
lib.dpad.argtypes = (POINTER(CTty), c_uint8, c_int16)

# Tty.pos()
lib.pos.argtypes = (POINTER(CTty), )
lib.pos.restype = CCoord

# Tty.mark()
lib.mark.argtypes = (POINTER(CTty), )

# Tty.load()
lib.load.argtypes = (POINTER(CTty), )

# Tty.hide_cursor()
lib.hide_cursor.argtypes = (POINTER(CTty), )

# Tty.show_cursor()
lib.show_cursor.argtypes = (POINTER(CTty), )

# Tty.set_fg()
lib.set_fg.argtypes = (POINTER(CTty), c_uint8)

# Tty.set_bg()
lib.set_bg.argtypes = (POINTER(CTty), c_uint8)

# Tty.aset_tx()
lib.set_tx.argtypes = (POINTER(CTty), c_uint8)

# Tty.set_fg_rgb()
lib.set_fg_rgb.argtypes = (POINTER(CTty), c_uint8, c_uint8, c_uint8)

# Tty.set_bg_rgb()
lib.set_bg_rgb.argtypes = (POINTER(CTty), c_uint8, c_uint8, c_uint8)

# Tty.set_fg_ansi()
lib.set_fg_ansi.argtypes = (POINTER(CTty), c_uint8)

# Tty.set_bg_ansi()
lib.set_bg_ansi.argtypes = (POINTER(CTty), c_uint8)

# Tty.set_style()
lib.set_style.argtypes = (POINTER(CTty), c_uint8, c_uint8, c_uint8)

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
        return (size.w, size.h)

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
        m = {
            "all": 0,
            "newln": 1,
            "currentln": 2,
            "cursorup": 3,
            "cursordn": 4,
        }.get(method, 5)
        lib.clear(self.tty, m)

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

    def dpad(self, direct: str, n: int):
        d = {
            "up": 0,
            "dn": 1,
            "left": 2,
            "right": 3,
        }.get(direct, 4)
        lib.dpad(self.tty, d, n)

    def pos(self) -> (int, int):
        coord = lib.pos(self.tty)
        return (coord.col, coord.row)

    def mark(self):
        lib.mark(self.tty)

    def load(self):
        lib.load(self.tty)

    def hide_cursor(self):
        lib.hide_cursor(self.tty)

    def show_cursor(self):
        lib.show_cursor(self.tty)

    def _color_match(self, color: str) -> c_uint8:
        return {
            "reset": 0,
            "black": 1,
            "red": 2,
            "green": 3,
            "yellow": 4,
            "blue": 5,
            "magenta": 6,
            "cyan": 7,
            "white": 8,
            "dark_grey": 9,
            "darkgrey": 9,
            "dark_red": 10,
            "darkred": 10,
            "dark_green": 11,
            "darkgreen": 11,
            "dark_yellow": 12,
            "darkyellow": 12,
            "dark_blue": 13,
            "darkblue": 13,
            "dark_magenta": 14,
            "darkmagenta": 14,
            "dark_cyan": 15,
            "darkcyan": 15,
            "grey": 16
        }.get(color, 17)

    def set_fg(self, color: str):
        lib.set_fg(self.tty, self._color_match(color))

    def set_bg(self, color: str):
        lib.set_fg(self.tty, self._color_match(color))

    def _style_match(self, style: str) -> c_uint8:
        lookup = ','.join(map(lambda x: x.strip(), style.split(',')))
        return {
            "reset": 0,
            "bold": 1,
            "dim": 2,
            "underline": 4,
            "reverse": 7,
            "hide": 8,

            "bold,underline": 14,
            "bold,reverse": 17,
            "bold,hide": 18,

            "dim,underline": 24,
            "dim,reverse": 27,
            "dim,hide": 28,

            "underline,reverse": 47,
            "underline,hide": 48,

            "reverse,hide": 49,

            "bold,underline,reverse": 147,
            "dim,underline,reverse": 247,

            "bold,reverse,hide": 148,
            "dim,reverse,hide": 248,

            "bold,underline,reverse,hide": 254,
            "dim,underline,reverse,hide": 255
        }.get(lookup, 9)

    def set_tx(self, style: str):
        lib.set_tx(self.tty, self._style_match(style))

    def set_fg_rgb(self, r: int, g: int, b: int):
        lib.set_fg_rgb(self.tty, r, g, b)

    def set_bg_rgb(self, r: int, g: int, b: int):
        lib.set_bg_rgb(self.tty, r, g, b)

    def set_fg_ansi(self, v: int):
        lib.set_fg_ansi(self.tty, v)

    def set_bg_ansi(self, v: int):
        lib.set_bg_ansi(self.tty, v)

    def set_style(self, fg: str, bg: str, style: str):
        lib.set_style(
            self.tty,
            self._color_match(fg),
            self._color_match(bg),
            self._style_match(style))

    def reset(self):
        lib.reset(self.tty)

    def prints(self, s: str):
        lib.prints(self.tty, s.encode('utf-8'))

    def flush(self):
        lib.flush(self.tty)


class SyncInput:
    def __init__(self, tty: 'Tty'):
        self.obj: POINTER('CSyncInput') = lib.read_sync(tty)
        self.evt: 'Event' = Event()

    def __enter__(self):
        return self

    def __iter__(self):
        return self

    def __next__(self):
        lib.sync_next(self.obj)  # blocking call
        self.evt.kind = lib.get_sync_kind(self.obj)
        self.evt.label = lib.get_sync_label(self.obj)
        self.evt.btn = lib.get_sync_btn(self.obj)
        self.evt.coord = lib.get_sync_coord(self.obj)
        self.evt.ch = lib.get_sync_ch(self.obj)

    def __exit__(self, exception_type, exception_value, traceback):
        lib.sync_free(self.obj)

    def close(self):
        lib.sync_free(self.obj)


class AsyncInput:
    def __init__(self, tty: 'Tty', delim=None):
        if delim is not None:
            self.obj: POINTER('CAsyncInput') = lib.read_until_async(tty, delim)
        else:
            self.obj: POINTER('CAsyncInput') = lib.read_async(tty)
        self.evt: 'Event' = Event()

    def __enter__(self):
        return self

    def __iter__(self):
        return self

    def __next__(self) -> bool:
        if lib.async_next(self.obj):  # non-blocking call
            self.evt.kind = lib.get_async_kind(self.obj)
            self.evt.label = lib.get_async_label(self.obj)
            self.evt.btn = lib.get_async_btn(self.obj)
            self.evt.coord = lib.get_async_coord(self.obj)
            self.evt.ch = lib.get_async_ch(self.obj)
            return True
        else:
            return False  # skip if there was no update to evt

    def __exit__(self, exception_type, exception_value, traceback):
        lib.async_free(self.obj)

    def close(self):
        lib.async_free(self.obj)


class Event:
    kind: int = 2
    label: int = None
    btn: int = None
    coord: CCoord = None
    ch: int = None
