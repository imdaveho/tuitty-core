import os
import sys
import platform
from enum import Enum, auto
from ctypes import c_uint, c_uint8, c_int16, c_uint32, \
    cdll, c_bool, c_char_p, c_void_p, \
    Structure, POINTER, byref, cast


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
    
    # Cursor Actions
    def goto(self, col: int, row: int):
        lib.dispatcher_goto(self.ptr, col, row)
    
    def up(self, n: int):
        lib.dispatcher_up(self.ptr, n)
    
    def down(self, n: int):
        lib.dispatcher_down(self.ptr, n)
    
    def left(self, n: int):
        lib.dispatcher_left(self.ptr, n)
    
    def right(self, n: int):
        lib.dispatcher_right(self.ptr, n)
    
    # Screen Actions
    def clear(self, clr: Clear):
        lib.dispatcher_clear(self.ptr, clr.value)
    
    def resize(self, w: int, h: int):
        lib.dispatcher_resize(self.ptr, w, h)
    
    def prints(self, content: str):
        lib.dispatcher_prints(self.ptr, content.encode('utf-8'))
    
    def printf(self, content: str):
        lib.dispatcher_printf(self.ptr, content.encode('utf-8'))
    
    def flush(self):
        lib.dispatcher_flush(self.ptr)

    # Style Actions
    def set_fg(self, color = None, ansi = None, rgb = None):
        if color is not None and isinstance(color, Color):
            lib.dispatcher_set_basic_fg(self.ptr, color.value)
        if ansi is not None and isinstance(ansi, int) and ansi < 256:
            lib.dispatcher_set_ansi_fg(self.ptr, ansi)
        if rgb is not None and isinstance(rgb, tuple) and len(rgb) == 3:
            lib.dispatcher_set_rgb_fg(self.ptr, rgb[0], rgb[1], rgb[2])
    
    def set_bg(self, color = None, ansi = None, rgb = None):
        if color is not None and isinstance(color, Color):
            lib.dispatcher_set_basic_bg(self.ptr, color.value)
        if ansi is not None and isinstance(ansi, int) and ansi < 256:
            lib.dispatcher_set_ansi_bg(self.ptr, ansi)
        if rgb is not None and isinstance(rgb, tuple) and len(rgb) == 3:
            lib.dispatcher_set_rgb_bg(self.ptr, rgb[0], rgb[1], rgb[2])
    
    def set_fx(self, fx: int):
        lib.dispatcher_set_fx(self.ptr, fx)
    
    def set_styles(self, fg: int, bg: int, fx: int):
        lib.dispatcher_set_styles(self.ptr, fg, bg, fx)
    
    def reset_styles(self):
        lib.dispatcher_reset_styles(self.ptr)
    
    # Toggle Mode Actions
    def show_cursor(self):
        lib.dispatcher_show_cursor(self.ptr)
    
    def hide_cursor(self):
        lib.dispatcher_hide_cursor(self.ptr)
    
    def enable_mouse(self):
        lib.dispatcher_enable_mouse(self.ptr)
    
    def disable_mouse(self):
        lib.dispatcher_disable_mouse(self.ptr)

    def enable_alt(self):
        lib.dispatcher_enable_alt(self.ptr)
    
    def disable_alt(self):
        lib.dispatcher_disable_alt(self.ptr)
    
    def raw(self):
        lib.dispatcher_raw(self.ptr)
    
    def cook(self):
        lib.dispatcher_cook(self.ptr)

    # Store Operation Actions
    def switch(self):
        lib.dispatcher_switch(self.ptr)
    
    def switch_to(self, id: int):
        lib.dispatcher_switch_to(self.ptr, id)
    
    def resized(self):
        lib.dispatcher_resized(self.ptr)
    
    def mark(self, col: int, row: int):
        lib.dispatcher_mark(self.ptr, col, row)
    
    def jump(self):
        lib.dispatcher_jump(self.ptr)
    
    def sync_tab_size(self):
        lib.dispatcher_sync_tab_size(self.ptr)
    
    # Manual cleanup (if not using with statement)
    def close(self):
        lib.dispatcher_free(self.ptr)


class EventHandle:
    def __init__(self, event_handle_ptr):
        self.ptr = event_handle_ptr
        self.evt = Eventmeta()
    
    def __enter__(self):
        return self
    
    def __exit__(self, exception_type, exception_value, traceback):
        lib.event_handle_free(self.ptr)
    
    # Cursor Actions
    def goto(self, col: int, row: int):
        lib.event_handle_goto(self.ptr, col, row)
    
    def up(self, n: int):
        lib.event_handle_up(self.ptr, n)
    
    def down(self, n: int):
        lib.event_handle_down(self.ptr, n)
    
    def left(self, n: int):
        lib.event_handle_left(self.ptr, n)
    
    def right(self, n: int):
        lib.event_handle_right(self.ptr, n)
    
    # Screen Actions
    def clear(self, clr: Clear):
        lib.event_handle_clear(self.ptr, clr.value)
    
    def resize(self, w: int, h: int):
        lib.event_handle_resize(self.ptr, w, h)
    
    def prints(self, content: str):
        lib.event_handle_prints(self.ptr, content.encode('utf-8'))
    
    def printf(self, content: str):
        lib.event_handle_printf(self.ptr, content.encode('utf-8'))
    
    def flush(self):
        lib.event_handle_flush(self.ptr)

    # Style Actions
    def set_fg(self, color = None, ansi = None, rgb = None):
        if color is not None and isinstance(color, Color):
            lib.event_handle_set_basic_fg(self.ptr, color.value)
        if ansi is not None and isinstance(ansi, int) and ansi < 256:
            lib.event_handle_set_ansi_fg(self.ptr, ansi)
        if rgb is not None and isinstance(rgb, tuple) and len(rgb) == 3:
            lib.event_handle_set_rgb_fg(self.ptr, rgb[0], rgb[1], rgb[2])
    
    def set_bg(self, color = None, ansi = None, rgb = None):
        if color is not None and isinstance(color, Color):
            lib.event_handle_set_basic_bg(self.ptr, color.value)
        if ansi is not None and isinstance(ansi, int) and ansi < 256:
            lib.event_handle_set_ansi_bg(self.ptr, ansi)
        if rgb is not None and isinstance(rgb, tuple) and len(rgb) == 3:
            lib.event_handle_set_rgb_bg(self.ptr, rgb[0], rgb[1], rgb[2])
    
    def set_fx(self, fx: int):
        lib.event_handle_set_fx(self.ptr, fx)
    
    def set_styles(self, fg: int, bg: int, fx: int):
        lib.event_handle_set_styles(self.ptr, fg, bg, fx)
    
    def reset_styles(self):
        lib.event_handle_reset_styles(self.ptr)
    
    # Toggle Mode Actions
    def show_cursor(self):
        lib.event_handle_show_cursor(self.ptr)
    
    def hide_cursor(self):
        lib.event_handle_hide_cursor(self.ptr)
    
    def enable_mouse(self):
        lib.event_handle_enable_mouse(self.ptr)
    
    def disable_mouse(self):
        lib.event_handle_disable_mouse(self.ptr)

    def enable_alt(self):
        lib.event_handle_enable_alt(self.ptr)
    
    def disable_alt(self):
        lib.event_handle_disable_alt(self.ptr)
    
    def raw(self):
        lib.event_handle_raw(self.ptr)
    
    def cook(self):
        lib.event_handle_cook(self.ptr)

    # Store Operation Actions
    def switch(self):
        lib.event_handle_switch(self.ptr)
    
    def switch_to(self, id: int):
        lib.event_handle_switch_to(self.ptr, id)
    
    def resized(self):
        lib.event_handle_resized(self.ptr)
    
    def mark(self, col: int, row: int):
        lib.event_handle_mark(self.ptr, col, row)
    
    def jump(self):
        lib.event_handle_jump(self.ptr)
    
    def sync_tab_size(self):
        lib.event_handle_sync_tab_size(self.ptr)
    
    # Store Requests (EventHandle Only)
    def size(self):
        size = lib.event_handle_size(self.ptr)
        return ((size >> 16), (size & 0xffff))

    def coord(self):
        coord = lib.event_handle_coord(self.ptr)
        return ((coord >> 16), (coord & 0xffff))

    def syspos(self):
        syspos = lib.event_handle_syspos(self.ptr)
        return ((syspos >> 16), (syspos &0xffff))

    def getch(self):
        ptr = lib.event_handle_getch(self.ptr)
        try:
            return cast(ptr, c_char_p).value.decode('utf-8')
        finally:
            lib.gotch_free(ptr)
        
    def poll_async(self):
        finished_polling = lib.event_handle_poll_async(
            self.ptr, byref(self.evt))
        if finished_polling:
            return self.evt
        else:
            return None
    
    def poll_latest_async(self):
        finished_polling = lib.event_handle_poll_latest_async(
            self.ptr, byref(self.evt))
        if finished_polling:
            return self.evt
        else:
            return None
    
    def poll_sync(self):
        lib.event_handle_poll_sync(self.ptr, byref(self.evt))
        return self.evt

    # Event Handle Commands
    def suspend(self):
        lib.event_handle_suspend(self.ptr)
    
    def transmit(self):
        lib.event_handle_transmit(self.ptr)
    
    def stop(self):
        # NOTE: This removes the Sender. This would cause the Receiver to
        # close; thus dropping the Channel (in idiomatic Rust). Since we
        # converted the object into a raw pointer, does the above still
        # hold true? If so, we have a problem that now the held pointer
        # might be pointing to garbage and present unsafe memory problems.
        lib.event_handle_stop(self.ptr)
    
    def lock(self):
        lib.event_handle_lock(self.ptr)
    
    def unlock(self):
        lib.event_handle_unlock(self.ptr)

    # Manual cleanup (if not using with statement)
    def close(self):
        lib.event_handle_free(self.ptr)


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

lib.dispatcher_reset_styles.argtypes = (POINTER(_Dispatcher),)
lib.event_handle_reset_styles.argtypes = (POINTER(_EventHandle),)


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
lib.event_handle_size.argtypes = (POINTER(_EventHandle),)
lib.event_handle_size.restype = c_uint32

lib.event_handle_coord.argtypes = (POINTER(_EventHandle),)
lib.event_handle_coord.restype = c_uint32

lib.event_handle_syspos.argtypes = (POINTER(_EventHandle),)
lib.event_handle_syspos.restype = c_uint32

lib.event_handle_getch.argtypes = (POINTER(_EventHandle),)
lib.event_handle_getch.restype = c_void_p
lib.gotch_free.argtypes = (c_void_p,)

lib.event_handle_poll_async.argtypes = (POINTER(_EventHandle), POINTER(Eventmeta))
lib.event_handle_poll_async.restype = c_bool
lib.event_handle_poll_latest_async.argtypes = (POINTER(_EventHandle), POINTER(Eventmeta))
lib.event_handle_poll_latest_async.restype = c_bool
lib.event_handle_poll_sync.argtypes = (POINTER(_EventHandle), POINTER(Eventmeta))


# Event Handle Commands
lib.event_handle_suspend.argtypes = (POINTER(_EventHandle),)
lib.event_handle_transmit.argtypes = (POINTER(_EventHandle),)
lib.event_handle_stop.argtypes = (POINTER(_EventHandle),)
lib.event_handle_lock.argtypes = (POINTER(_EventHandle),)
lib.event_handle_unlock.argtypes = (POINTER(_EventHandle),)

