import sys
import ctypes

from ctypes import c_char_p, c_int, c_uint8, c_int16, c_uint32, \
    Structure, POINTER


prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
lib = ctypes.cdll.LoadLibrary(prefix + "tuitty" + extension)


class CTty(Structure):
    pass


class CSize(Structure):
    _fields_ = [("w", c_int16), ("h", c_int16)]

    def __str__(self):
        return "({}, {})".format(self.w, self.h)


class CCoord(Structure):
    _fields_ = [("row", c_int16), ("col", c_int16)]

    def __str__(self):
        return "({}, {})".format(self.row, self.col)


class CSyncInput(Structure):
    pass


class CAsyncInput(Structure):
    pass


lib.init.restype = POINTER(CTty)

lib.exit.argtypes = (POINTER(CTty), )

lib.size.argtypes = (POINTER(CTty), )
lib.size.restype = CSize

lib.raw.argtypes = (POINTER(CTty), )

lib.cook.argtypes = (POINTER(CTty), )

lib.enable_mouse.argtypes = (POINTER(CTty), )

lib.disable_mouse.argtypes = (POINTER(CTty), )

lib.read_char.argtypes = (POINTER(CTty), )
lib.read_char.restype = c_uint32

lib.read_sync.argtypes = (POINTER(CTty), )
lib.read_sync.restype = POINTER(CSyncInput)

lib.sync_next.argtypes = (POINTER(CSyncInput), )

lib.sync_free.argtypes = (POINTER(CSyncInput), )

lib.read_async.argtypes = (POINTER(CTty), )
lib.read_async.restype = POINTER(CAsyncInput)

lib.read_until_async.argtypes = (POINTER(CTty), )
lib.read_until_async.restype = POINTER(CAsyncInput)

lib.async_next.argtypes = (POINTER(CAsyncInput), )

lib.async_free.argtypes = (POINTER(CAsyncInput), )

lib.clear.argtypes = (POINTER(CTty), c_uint8)

lib.resize.argtypes = (POINTER(CTty), c_int16, c_int16)

lib.switch.argtypes = (POINTER(CTty), )

lib.to_main.argtypes = (POINTER(CTty), )

lib.switch_to.argtypes = (POINTER(CTty), c_int)

lib.goto.argtypes = (POINTER(CTty), c_int16, c_int16)

lib.up.argtypes = (POINTER(CTty), )

lib.dn.argtypes = (POINTER(CTty), )

lib.left.argtypes = (POINTER(CTty), )

lib.right.argtypes = (POINTER(CTty), )

lib.dpad.argtypes = (POINTER(CTty), c_uint8, c_int16)

lib.pos.argtypes = (POINTER(CTty), )
lib.pos.restype = CCoord

lib.mark.argtypes = (POINTER(CTty), )

lib.load.argtypes = (POINTER(CTty), )

lib.hide_cursor.argtypes = (POINTER(CTty), )

lib.show_cursor.argtypes = (POINTER(CTty), )

lib.set_fg.argtypes = (POINTER(CTty), c_uint8)

lib.set_bg.argtypes = (POINTER(CTty), c_uint8)

lib.set_tx.argtypes = (POINTER(CTty), c_uint8)

lib.set_fg_rgb.argtypes = (POINTER(CTty), c_uint8, c_uint8, c_uint8)

lib.set_bg_rgb.argtypes = (POINTER(CTty), c_uint8, c_uint8, c_uint8)

lib.set_fg_ansi.argtypes = (POINTER(CTty), c_uint8)

lib.set_bg_ansi.argtypes = (POINTER(CTty), c_uint8)

lib.set_style.argtypes = (POINTER(CTty), c_uint8, c_uint8, c_uint8)

lib.reset.argtypes = (POINTER(CTty), )

lib.print.argtypes = (POINTER(CTty), c_char_p)

lib.flush.argtypes = (POINTER(CTty), )


class Tty:
    def __init__(self):
        self.obj = lib.init()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.exit(self.obj)

    def size(self):
        lib.size(self.obj)

    def raw(self):
        lib.raw(self.obj)

    def cook(self):
        lib.cook(self.obj)

    def enable_mouse(self):
        lib.enable_mouse(self.obj)

    def disable_mouse(self):
        lib.disable_mouse(self.obj)

    def read_char(self):
        u32 = lib.read_char(self.obj)
        # TODO: turn U32 to byte array
