import time
import random
from itertools import cycle
from tuitty.ffi import Dispatcher, Clear

def run():

    quotes_a = ["Find the good", "Perfect is the", "Thru discipline",
            "if knocked_down:", "Non nobis solum", "Ad astra"]
    quotes_b = ["and believe it", "enemy of great", "comes freedom",
        "  get_up += 1", "nati sumus", "per aspera"]

    errors_a = ["404 Not Found:", "ERR04 Recursion", "[error]: cannot",
        "[!] `undefined`", "RefErr: event", "#: Cannot find"]
    errors_b = ["file `good.py`", "depth exceeded", "assign to field",
        "is not an object", "is not defined", "vcvarsall.bat"]
    
    relaxes_a = ["Ok, keep calm", "Hmm...oh! Just", "www.reddit.com/",
        "It works again!", "It's not a bug,", "Time to go for"]
    relaxes_b = ["and carry on", "a typo : vs ;", "r/funnycats",
        "...no idea why", "it's a feature!", "a coffee run!"]
    indices = [0, 1, 2, 3 , 4, 5]
    random.shuffle(indices)

    with Dispatcher() as dispatch, open(
        "animation_loop.txt", encoding="utf-8", mode="r") as f:
        dispatch.clear(Clear.CursorDown)
        start_row = 5
        start_col = 0
        iterations = 0
        full_loops = 0
        msg_ax = 102
        msg_bx = 175
        scenes = f.read().split('1.')
        scene_len = len(scenes)
        if scene_len != 263:
            raise("Incorrect scene length.")
        
        dispatch.goto(start_col, start_row)
        scenes = cycle(scenes)
        bubble = indices.pop()
        while full_loops < (scene_len * 2):
            dispatch.goto(start_col, start_row)
            dispatch.clear(Clear.CursorDown)
            scene = next(scenes)
            
            if (iterations % scene_len) in range(46, 73):
                quote_av = quotes_a[bubble]
                quote_bv = quotes_b[bubble]
                end_ax = msg_ax + len(quote_av)
                end_bx = msg_bx + len(quote_bv)
                msg = scene[3:msg_ax] + quote_av + \
                    scene[end_ax:msg_bx] + quote_bv + scene[end_bx:]
            elif (iterations % scene_len) in range(74, 99):
                error_a = errors_a[bubble]
                error_b = errors_b[bubble]
                end_ax = msg_ax + len(error_a)
                end_bx = msg_bx + len(error_b)
                msg = scene[3:msg_ax] + error_a + \
                    scene[end_ax:msg_bx] + error_b + scene[end_bx:]
            elif (iterations % scene_len) in range(120, 132):
                relax_a = relaxes_a[bubble]
                relax_b = relaxes_b[bubble]
                end_ax = msg_ax + len(relax_a)
                end_bx = msg_bx + len(relax_b)
                msg = scene[3:msg_ax] + relax_a + scene[end_ax:msg_bx] + relax_b + scene[end_bx:]
            elif iterations == scene_len:
                iterations = 1
                full_loops += scene_len
                try:
                    bubble = indices.pop()
                except IndexError:
                    indices.extend([0, 1, 2, 3, 4, 5])
                    random.shuffle(indices)
                    bubble = indices.pop()
                time.sleep(0.1)
                continue
            else:
                msg = scene[3:]
            iterations += 1
            dispatch.printf(msg)
            time.sleep(0.1)
        dispatch.goto(start_col, start_row)
        dispatch.clear(Clear.CursorDown)
        dispatch.flush()


if __name__ == "__main__":
    run()
