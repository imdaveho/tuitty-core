import unittest
import time
from tuitty.ffi import Dispatcher, InputEvent, Clear

# def test_goto():
#     with Dispatcher() as dispatch:
#         dispatch.enable_alt()
#         dispatch.raw()
#         with dispatch.listen() as listener:
#             (col, row) = listener.coord()
#             dispatch.goto(col, row)
#             dispatch.prints("H")
#             dispatch.flush()
#             time.sleep(2)
#             dispatch.goto(0, 0)
#             dispatch.flush()
#             time.sleep(2)
#             (w, h) = listener.size()
#             dispatch.goto(w - 3, h - 3)
#             dispatch.prints("I")
#             dispatch.flush()
#             time.sleep(2)
#         dispatch.cook()
#         time.sleep(0.5)
#         dispatch.disable_alt()

def test_poll_async():
    with Dispatcher() as dispatch:
        dispatch.switch()
        dispatch.raw()
        is_running = True
        dispatch.printf("This is a sample string.")
        with dispatch.listen() as listener:
            while is_running:
                time.sleep(0.1)
                evt = listener.poll_latest_async()
                if evt is None: continue
                if evt.kind() == InputEvent.Left:
                    dispatch.left(1)
                    dispatch.flush()
                elif evt.kind() == InputEvent.Right:
                    dispatch.right(1)
                    dispatch.flush()
                elif evt.kind() == InputEvent.Up:
                    dispatch.up(1)
                    dispatch.flush()
                elif evt.kind() == InputEvent.Down:
                    dispatch.down(1)
                    dispatch.flush()
                elif evt.kind() == InputEvent.Enter:
                    ch = listener.getch()
                    (col, row) = listener.coord()
                    listener.goto(30, 5)
                    listener.printf(f"char: ({ch})")
                    listener.goto(col, row)
                    listener.flush()                    
                elif evt.kind() == InputEvent.Char:
                    if evt.data() == 'q':
                        is_running = False
                    elif evt.data() == '1':
                        dispatch.switch_to(1)
                    elif evt.data() == ' ':
                        dispatch.switch_to(0)
                    elif evt.data() == '2':
                        dispatch.switch_to(2)
                    elif evt.data() == '!':
                        dispatch.switch()
                        dispatch.raw()
                        dispatch.goto(5, 5)
                        dispatch.printf("This is aalt screeen 2")
                else: pass
        dispatch.cook()
        time.sleep(0.5)
        dispatch.disable_alt()
    # dispatch drops
    print("waiting 2 secs...")
    time.sleep(2)


# import os
# def print_root():
#     # Uncomment when `share` is in the tuitty repo directory
#     root = os.path.dirname(       # tuitty (repo)
#         os.path.dirname(          # dist
#             os.path.dirname(      # tuitty-py (dist)
#                 os.path.dirname(  # tuitty (dist)
#                     os.path.abspath(__file__)))))
#     # root = os.path.dirname(os.path.abspath(__file__))
#     path = os.path.join(
#         os.path.abspath(root), "share", "windows", "intel64",
#         "" + "tuitty" + ".dll")
#     print(path)


# import os
# import sys
# import platform

# def print_settings():
#     prefix = {'win32': ''}.get(sys.platform, 'lib')
#     extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')

#     # Uncomment when `share` is in dist/tuitty-py directory
#     # root = os.path.dirname(os.path.abspath(__file__))

#     # Uncomment when `share` is in the tuitty repo directory
#     root = os.path.dirname(       # tuitty (repo)
#         os.path.dirname(          # dist
#             os.path.dirname(      # tuitty-py (dist)
#                 os.path.dirname(  # tuitty (dist)
#                     os.path.abspath(__file__)))))

#     system = {"darwin": "macos", "win32": "windows"}.get(sys.platform, "linux")
#     bits = "64" if platform.architecture()[0] == "64bit" else "32"
#     cpu = {
#         "aarch64": "arm",
#         "x86_64": "intel",
#         "i686": "intel",
#         "AMD64": "amd"
#     }.get(platform.machine(), "arm")

#     path = os.path.join(
#         os.path.abspath(root), "share", system, cpu + bits,
#         prefix + "tuitty" + extension)
    
#     print(path)

if __name__ == '__main__':
    # test_goto()
    test_poll_async()
    # print_root()
    # print_settings()



# class TestInputs(unittest.TestCase):
#     def test_poll_async(self):
#         with Dispatcher() as dispatch:
#             # TODO: optional -- remove once confirmed
#             dispatch.goto(5, 5)
#             dispatch.prints("Hello, tuitty!")
#             time.sleep(2)
#             dispatch.enable_alt()
#             dispatch.raw()
#             is_running = True
#             with dispatch.listen() as listener:
#                 while is_running:
#                     time.sleep(1./25)
#                     evt = listener.poll_async
#                     if evt is None: continue
#                     if evt.kind() == InputEvent.Left:
#                         dispatch.left(1)
#                     elif evt.kind() == InputEvent.Right:
#                         dispatch.right(1)
#                     elif evt.kind() == InputEvent.Up:
#                         dispatch.up(1)
#                     elif evt.kind() == InputEvent.Down:
#                         dispatch.down(1)
#                     elif evt.kind() == InputEvent.Char:
#                         if evt.data() == 'q': is_running = False
#                     else:
#                         pass
#             dispatch.cook()
#             dispatch.disable_alt()
#         # Dispatcher drops
#         print("waiting 2 secs...")
#         time.sleep(2)

# if __name__ == "__main__":
#     unittest.main()









# import unittest
# import time
# from tuitty.ffi import Tty, InputEvent


# class TestInputs(unittest.TestCase):
#     def test_inputs(self):
#         tty = Tty()
#         time.sleep(2)

#         tty.switch()
#         tty.raw()
#         tty.hide_cursor()

#         tty.enable_mouse()
#         stdin = tty.read_async()

#         while True:
#             time.sleep(1./60)
#             tty.goto(0, 10)
#             tty.flush()
#             contains = next(stdin)
#             if not contains:
#                 continue
#             tty.clear("currentln")
#             ev = stdin.event
#             # tty.goto(0, 12)
#             # tty.clear("currentln")
#             # tty.prints("kind_int: {}, data_int: {}, chr_val: {}".format(
#             #     ev._kind, ev._data, chr(ev._data)))
#             tty.goto(0, 10)
#             tty.prints(str(ev))

#             if ev.kind() == InputEvent.Ctrl:
#                 if ev.data() == 'q':
#                     tty.clear("currentln")
#                     tty.goto(0, 10)
#                     tty.prints("Ctrl(q) pressed. Exiting...")
#                     tty.flush()
#                     time.sleep(2)
#                     break
#         # end loop
#         tty.to_main()

#         tty.prints("Done! Does the cursor show? > ")
#         tty.flush()

#         time.sleep(2)
#         tty.terminate()  # tty is dropped here
