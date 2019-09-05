import unittest
import time
# from enum import Enum
from tuitty.ffi import Tty, InputEvent


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
                ev = stdin.event
                tty.goto(0, 12)
                tty.clear("currentln")
                tty.prints("kind_int: {}, data_int: {}, chr_val: {}".format(
                    ev._kind, ev._data, chr(ev._data)))
                tty.goto(0, 10)
                tty.prints(str(ev))

                if ev.kind() == InputEvent.Ctrl:
                    if ev.data() == 'q':
                        tty.clear("currentln")
                        tty.goto(0, 10)
                        tty.prints("Ctrl(q) pressed. Exiting...")
                        tty.flush()
                        time.sleep(2)
                        break
            # end loop
            tty.to_main()

            tty.prints("Done! Does the cursor show? > ")
            tty.flush()

            time.sleep(2)
        # tty is dropped here
