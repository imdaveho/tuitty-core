import os
import sys
import platform
from enum import Enum, auto
from ctypes import c_uint, c_uint8, c_int16, c_uint32, \
    cdll, c_bool, c_char_p, c_void_p, \
    Structure, POINTER, byref
from typing import Callable


prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')

# Uncomment when `share` is in dist/tuitty-py directory
# root = os.path.dirname(os.path.abspath(__file__))

# Uncomment when `share` is in the tuitty repo directory
root = os.path.dirname(       # tuitty (repo)
    os.path.dirname(          # dist
        os.path.dirname(      # tuitty-py (dist)
            os.path.dirname(  # tuitty (dist)
                os.path.abspath(__file__)))))

system = {"darwin": "macos", "win32": "windows"}.get(sys.platform, "linux")
bits = "64" if platform.architecture()[0] == "64bit" else "32"
cpu = {
    "aarch64": "arm",
    "x86_64": "intel",
    "i686": "intel",
    "AMD64": "amd"
}.get(platform.machine(), "arm")

path = os.path.join(
    os.path.abspath(root), "share", system, cpu + bits,
    prefix + "tuitty" + extension)
lib = cdll.LoadLibrary(path)


## Parallel Structs (Classes) and Enums ########################################

class _Dispatcher(Structure):
    pass


class _EventHandle(Structure):
    pass


class Eventmeta(Structure):
    _fields_ = [("_kind", c_int16), ("_data", c_uint32)]

    def kind(self) -> 'InputEvent':
        return InputEvent(self._kind)

    def data(self):
        variant = InputEvent(self._kind)
        char_variants = (InputEvent.Char, InputEvent.Alt, InputEvent.Ctrl,)
        mouse_variants = (
            InputEvent.MousePressLeft,
            InputEvent.MousePressRight,
            InputEvent.MousePressMiddle,
            InputEvent.MousePressWheelUp,
            InputEvent.MousePressWheelDown,
            InputEvent.MouseRelease,
            InputEvent.MouseHold,)

        if variant in char_variants:
            return chr(self._data)
        elif variant in mouse_variants:
            return ((self._data >> 16), (self._data & 0xffff))

        else:
            return self._data

    def __str__(self):
        variant = InputEvent(self._kind)
        char_variants = (InputEvent.Char, InputEvent.Alt, InputEvent.Ctrl,)
        mouse_variants = (
            InputEvent.MousePressLeft,
            InputEvent.MousePressRight,
            InputEvent.MousePressMiddle,
            InputEvent.MousePressWheelUp,
            InputEvent.MousePressWheelDown,
            InputEvent.MouseRelease,
            InputEvent.MouseHold,)
        if variant not in char_variants + mouse_variants + (InputEvent.F,):
            return "Key: {}".format(InputEvent(self._kind).name)

        elif variant in char_variants + (InputEvent.F,):
            return "Key: {}({})".format(
                InputEvent(self._kind).name, self.data())

        elif variant in mouse_variants:
            s = {
                InputEvent.MousePressLeft: "Button-Left",
                InputEvent.MousePressRight: "Button-Right",
                InputEvent.MousePressMiddle: "Button-Middle",
                InputEvent.MousePressWheelUp: "Button-WheelUp",
                InputEvent.MousePressWheelDown: "Button-WheelDown",
                InputEvent.MouseRelease: "Release",
                InputEvent.MouseHold: "Hold",
            }.get(variant)
            return "Mouse: {} @ ({}, {})".format(s, *self.data())


class InputEvent(Enum):
    Null = 0
    Backspace = auto()
    Enter = auto()
    Left = auto()
    Right = auto()
    Up = auto()
    Down = auto()
    Home = auto()
    End = auto()
    PageUp = auto()
    PageDown = auto()
    Tab = auto()
    BackTab = auto()
    Delete = auto()
    Insert = auto()
    F = auto()
    Char = auto()
    Alt = auto()
    Ctrl = auto()
    Esc = auto()
    CtrlLeft = auto()
    CtrlRight = auto()
    CtrlUp = auto()
    CtrlDown = auto()
    ShiftLeft = auto()
    ShiftRight = auto()
    ShiftUp = auto()
    ShiftDown = auto()
    MousePressLeft = auto()
    MousePressRight = auto()
    MousePressMiddle = auto()
    MousePressWheelUp = auto()
    MousePressWheelDown = auto()
    MouseRelease = auto()
    MouseHold = auto()


class Clear(Enum):
    All = 0
    CursorDown = auto()
    CursorUp = auto()
    CurrentLine = auto()
    NewLine = auto()


class Color(Enum):
    Reset = 0
    Black = auto()
    DarkGrey = auto()
    Red = auto()
    DarkRed = auto()
    Green = auto()
    DarkGreen = auto()
    Yellow = auto()
    DarkYellow = auto()
    Blue = auto()
    DarkBlue = auto()
    Magenta = auto()
    DarkMagenta = auto()
    Cyan = auto()
    DarkCyan = auto()
    White = auto()
    Grey = auto()


class Dispatcher:
    def __init__(self):
        self.ptr = lib.dispatcher()

    def __enter__(self):
        return self

    def __exit__(self, exception_type, exception_value, traceback):
        lib.dispatcher_free(self.ptr)

    def listen(self) -> 'EventHandle':
        event_handle_ptr = lib.dispatcher_listen(self.ptr)
        return EventHandle(event_handle_ptr)

    def spawn(self) -> 'EventHandle':
        event_handle_ptr = lib.dispatcher_spawn(self.ptr)
        return EventHandle(event_handle_ptr)


class EventHandle:
    def __init__(self, event_handle_ptr):
        self.ptr = event_handle_ptr



# # FFI ########################################################################

# Dispatch initialization.
lib.dispatcher.restype = POINTER(_Dispatcher)

# Event Handle initialization and creation.
lib.dispatcher_listen.argtypes = (POINTER(_Dispatcher),)
lib.dispatcher_listen.restype = POINTER(_EventHandle)

lib.dispatcher_spawn.argtypes = (POINTER(_Dispatcher),)
lib.dispatcher_spawn.restype = POINTER(_EventHandle)

# Memory management of FFI Objects
lib.dispatcher_free.argtypes = (POINTER(_Dispatcher), )
lib.event_handle_free.argtypes = (POINTER(_EventHandle),)


# Cursor Signals
lib.dispatcher_goto.argtypes = (POINTER(_Dispatcher), c_int16, c_int16)
lib.event_handle_goto.argtypes = (POINTER(_EventHandle), c_int16, c_int16)
lib.dispatcher_up.argtypes = (POINTER(_Dispatcher), c_int16)
lib.event_handle_up.argtypes = (POINTER(_EventHandle), c_int16)
lib.dispatcher_down.argtypes = (POINTER(_Dispatcher), c_int16)
lib.event_handle_down.argtypes = (POINTER(_EventHandle), c_int16)
lib.dispatcher_left.argtypes = (POINTER(_Dispatcher), c_int16)
lib.event_handle_left.argtypes = (POINTER(_EventHandle), c_int16)
lib.dispatcher_right.argtypes = (POINTER(_Dispatcher), c_int16)
lib.event_handle_right.argtypes = (POINTER(_EventHandle), c_int16)


# Screen Signals
lib.dispatcher_clear.argtypes = (POINTER(_Dispatcher), c_uint8)
lib.event_handle_clear.argtypes = (POINTER(_EventHandle), c_uint8)
lib.dispatcher_resize.argtypes = (POINTER(_Dispatcher), c_int16, c_int16)
lib.event_handle_resize.argtypes = (POINTER(_EventHandle), c_int16, c_int16)

lib.dispatcher_prints.argtypes = (POINTER(_Dispatcher), c_char_p)
lib.event_handle_prints.argtypes = (POINTER(_EventHandle), c_char_p)
lib.dispatcher_printf.argtypes = (POINTER(_Dispatcher), c_char_p)
lib.event_handle_printf.argtypes = (POINTER(_EventHandle), c_char_p)
lib.dispatcher_flush.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_flush.argtypes = (POINTER(_EventHandle),)


# Style Signals
lib.dispatcher_set_basic_fg.argtypes = (POINTER(_Dispatcher), c_uint8)
lib.event_handle_set_basic_fg.argtypes = (POINTER(_EventHandle), c_uint8)
lib.dispatcher_set_ansi_fg.argtypes = (POINTER(_Dispatcher), c_uint8)
lib.event_handle_set_ansi_fg.argtypes = (POINTER(_EventHandle), c_uint8)
lib.dispatcher_set_rgb_fg.argtypes = (
    POINTER(_Dispatcher), c_uint8, c_uint8, c_uint8)
lib.event_handle_set_rgb_fg.argtypes = (
    POINTER(_Dispatcher), c_uint8, c_uint8, c_uint8)

lib.dispatcher_set_basic_bg.argtypes = (POINTER(_Dispatcher), c_uint8)
lib.event_handle_set_basic_bg.argtypes = (POINTER(_EventHandle), c_uint8)
lib.dispatcher_set_ansi_bg.argtypes = (POINTER(_Dispatcher), c_uint8)
lib.event_handle_set_ansi_bg.argtypes = (POINTER(_EventHandle), c_uint8)
lib.dispatcher_set_rgb_bg.argtypes = (
    POINTER(_Dispatcher), c_uint8, c_uint8, c_uint8)
lib.event_handle_set_rgb_bg.argtypes = (
    POINTER(_Dispatcher), c_uint8, c_uint8, c_uint8)

lib.dispatcher_set_fx.argtypes = (POINTER(_Dispatcher), c_uint32)
lib.event_handle_set_fx.argtypes = (POINTER(_EventHandle), c_uint32)

lib.dispatcher_set_styles.argtypes = (
    POINTER(_Dispatcher), c_uint8, c_uint8, c_uint32)
lib.event_handle_set_styles.argtypes = (
    POINTER(_EventHandle), c_uint8, c_uint8, c_uint32)


# Toggle Mode Signals
lib.dispatcher_show_cursor.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_show_cursor.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_hide_cursor.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_hide_cursor.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_enable_mouse.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_enable_mouse.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_disable_mouse.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_disable_mouse.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_enable_alt.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_enable_alt.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_disable_alt.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_disable_alt.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_raw.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_raw.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_cook.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_cook.argtypes = (POINTER(_EventHandle),)


# Store Operation Signals
lib.dispatcher_switch.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_switch.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_switch_to.argtypes = (POINTER(_Dispatcher), c_uint)
lib.event_handle_switch_to.argtypes = (POINTER(_EventHandle), c_uint)
lib.dispatcher_resized.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_resized.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_mark.argtypes = (POINTER(_Dispatcher), c_int16, c_int16)
lib.event_handle_mark.argtypes = (POINTER(_EventHandle), c_int16, c_int16)
lib.dispatcher_jump.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_jump.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_sync_tab_size.argtypes = (POINTER(_Dispatcher), c_uint)
lib.event_handle_sync_tab_size.argtypes = (POINTER(_EventHandle), c_uint)


# Store Requests (EventHandle Only)
lib.dispatcher_size.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_size.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_size.restype = c_uint32
lib.event_handle_size.restype = c_uint32

lib.dispatcher_coord.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_coord.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_coord.restype = c_uint32
lib.event_handle_coord.restype = c_uint32

lib.dispatcher_syspos.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_syspos.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_syspos.restype = c_uint32
lib.event_handle_syspos.restype = c_uint32

lib.dispatcher_getch.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_getch.argtypes = (POINTER(_EventHandle),)
lib.dispatcher_getch.restype = c_void_p
lib.event_handle_getch.restype = c_void_p

lib.dispatcher_gotch_free.argtypes = (c_void_p,)
lib.event_handle_gotch_free.argtypes = (c_void_p,)

lib.dispatcher_poll_async.argtypes = (POINTER(_Dispatcher), POINTER(Eventmeta))
lib.event_handle_poll_async.argtypes = (POINTER(_EventHandle), POINTER(Eventmeta))
lib.dispatcher_poll_async.restype = c_bool
lib.event_handle_poll_async.restype = c_bool

lib.dispatcher_poll_latest_async.argtypes = (POINTER(_Dispatcher), POINTER(Eventmeta))
lib.event_handle_poll_latest_async.argtypes = (POINTER(_EventHandle), POINTER(Eventmeta))
lib.dispatcher_poll_latest_async.restype = c_bool
lib.event_handle_poll_latest_async.restype = c_bool

lib.dispatcher_poll_sync.argtypes = (POINTER(_Dispatcher), POINTER(Eventmeta))
lib.event_handle_poll_sync.argtypes = (POINTER(_EventHandle), POINTER(Eventmeta))



# class Tty:
#     def __init__(self):
#         self.tty = lib.init()

#     def __enter__(self):
#         return self

#     def __exit__(self, exception_type, exception_value, traceback):
#         lib.terminate(self.tty)

#     def terminate(self):
#         lib.terminate(self.tty)

#     def size(self) -> (int, int):
#         size = lib.size(self.tty)
#         return ((size >> 16), (size & 0xffff))

#     def raw(self):
#         lib.raw(self.tty)

#     def cook(self):
#         lib.cook(self.tty)

#     def enable_mouse(self):
#         lib.enable_mouse(self.tty)

#     def disable_mouse(self):
#         lib.disable_mouse(self.tty)

#     def read_char(self) -> str:
#         return chr(lib.read_char(self.tty))

#     def read_sync(self) -> 'SyncInput':
#         return SyncInput(self.tty)

#     def read_async(self) -> 'AsyncInput':
#         return AsyncInput(self.tty)

#     def clear(self, method: str):
#         lib.clear(self.tty, method.encode('utf-8'))

#     def resize(self, w: int, h: int):
#         lib.resize(self.tty, w, h)

#     def switch(self):
#         lib.switch(self.tty)

#     def to_main(self):
#         lib.to_main(self.tty)

#     def switch_to(self, index: int):
#         lib.switch_to(self.tty, index)

#     def goto(self, col: int, row: int):
#         lib.goto(self.tty, col, row)

#     def up(self):
#         lib.up(self.tty)

#     def dn(self):
#         lib.dn(self.tty)

#     def left(self):
#         lib.left(self.tty)

#     def right(self):
#         lib.right(self.tty)

#     def dpad(self, direction: str, n: int):
#         lib.dpad(self.tty, direction.encode('utf-8'), n)

#     def pos(self) -> (int, int):
#         coord = lib.pos(self.tty)
#         return ((coord >> 16), (coord & 0xffff))

#     def mark(self):
#         lib.mark(self.tty)

#     def load(self):
#         lib.load(self.tty)

#     def hide_cursor(self):
#         lib.hide_cursor(self.tty)

#     def show_cursor(self):
#         lib.show_cursor(self.tty)

#     def set_fg(self, color: str):
#         lib.set_fg(self.tty, color.encode('utf-8'))

#     def set_bg(self, color: str):
#         lib.set_bg(self.tty, color.encode('utf-8'))

#     def set_tx(self, style: str):
#         lib.set_tx(self.tty, style.encode('utf-8'))

#     def set_fg_rgb(self, r: int, g: int, b: int):
#         lib.set_fg_rgb(self.tty, r, g, b)

#     def set_bg_rgb(self, r: int, g: int, b: int):
#         lib.set_bg_rgb(self.tty, r, g, b)

#     def set_fg_ansi(self, v: int):
#         lib.set_fg_ansi(self.tty, v)

#     def set_bg_ansi(self, v: int):
#         lib.set_bg_ansi(self.tty, v)

#     def set_style(self, fg: str, bg: str, style: str):
#         lib.set_style(self.tty,
#                       fg.encode('utf-8'),
#                       bg.encode('utf-8'),
#                       style.encode('utf-8'))

#     def reset(self):
#         lib.reset(self.tty)

#     def prints(self, s: str):
#         lib.prints(self.tty, s.encode('utf-8'))

#     def flush(self):
#         lib.flush(self.tty)


# class SyncInput:
#     def __init__(self, tty: 'Tty'):
#         self.obj: POINTER('CSyncReader') = lib.read_sync(tty)
#         self.event: 'Event' = Event()

#     def __enter__(self):
#         return self

#     def __iter__(self):
#         return self

#     def __next__(self):
#         lib.sync_next(self.obj, byref(self.event))  # blocking call

#     def __exit__(self, exception_type, exception_value, traceback):
#         lib.sync_free(self.obj)

#     def close(self):
#         lib.sync_free(self.obj)


# class AsyncInput:
#     def __init__(self, tty: 'Tty', delimiter=None):
#         if delimiter is not None:
#             self.obj: POINTER('CAsyncReader') = \
#                 lib.read_until_async(tty, delimiter)
#         else:
#             self.obj: POINTER('CAsyncReader') = lib.read_async(tty)
#         self.event: 'Event' = Event()

#     def __enter__(self):
#         return self

#     def __iter__(self):
#         return self

#     def __next__(self) -> bool:
#         return lib.async_next(self.obj, byref(self.event))  # non-blocking call

#     def __exit__(self, exception_type, exception_value, traceback):
#         lib.async_free(self.obj)

#     def close(self):
#         lib.async_free(self.obj)
