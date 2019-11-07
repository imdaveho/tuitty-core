# import unittest
# import time
# from tuitty.ffi import Tty


# class TestExample(unittest.TestCase):
#     def test_example(self):
#         with Tty() as tty:
#             tty.prints("w: {}, h: {}\n".format(*tty.size()))

#             tty.set_fg("green")
#             tty.prints("Hello (g), ")
#             tty.reset()
#             tty.prints("Hello (d), ")
#             tty.flush()
#             tty.switch()
#             tty.clear("all")

#             tty.raw()
#             tty.enable_mouse()

#             words = """A good choice of font for your coding can make a huge difference and improve your productivity, so take a look at the fonts in this post that can make your text editor or terminal emulator look little bit nicer. Andale® Mono — is a monospaced sans-serif typeface designed by Steve Matteson for terminal emulation and software development environments, originally for the Taligent project by Apple Inc. and IBM. The Andalé design was originally created by Monotype, as part of Andalé font families. Aperçu — Aperçu was started in December 2009, and has been trialled and tested through a number of design comissions taken on by The Entente through 2010. The conceit behind Aperçu was to create a synopsis or amalgamation of classic realist typefaces: Johnston, Gill Sans, Neuzeit & Franklin Gothic."""

#             tty.set_fg("red")
#             tty.prints(words)
#             tty.flush()

#             time.sleep(3)

#             tty.to_main()

#             tty.prints("Hello (r), ")
#             tty.set_fg("darkblue")
#             tty.prints("Hello (db), ")
#             tty.reset()
#             tty.prints("End\n")
#             tty.flush()

#             time.sleep(2)

#         time.sleep(2)


# if __name__ == "__main__":
#     unittest.main()
