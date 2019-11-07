import unittest
import time
from tuitty.ffi import Dispatcher, InputEvent


def test_poll_async(self):
    with Dispatcher() as dispatch:
        # TODO: optional -- remove once confirmed
        dispatch.goto(5, 5)
        dispatch.prints("Hello, tuitty!")
        time.sleep(2)
        dispatch.enable_alt()
        dispatch.raw()
        is_running = True
        with dispatch.listen() as listener:
            while is_running:
                time.sleep(1./25)
                evt = listener.poll_async
                if evt is None: continue
                if evt.kind() == InputEvent.Left:
                    dispatch.left(1)
                elif evt.kind() == InputEvent.Right:
                    dispatch.right(1)
                elif evt.kind() == InputEvent.Up:
                    dispatch.up(1)
                elif evt.kind() == InputEvent.Down:
                    dispatch.down(1)
                elif evt.kind() == InputEvent.Char:
                    if evt.data() == 'q': is_running = False
                else:
                    pass
        dispatch.cook()
        dispatch.disable_alt()
    # Dispatcher drops
    print("waiting 2 secs...")
    time.sleep(2)

if __name__ == '__main__':
    test_poll_async()




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
