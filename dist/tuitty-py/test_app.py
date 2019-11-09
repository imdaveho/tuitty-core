import time
import asyncio
from tuitty.ffi import Dispatcher, InputEvent, Color, Effect


DELAY = 0.01
SECTIONS = ["ABOUT", "EXPERIENCE", "SKILLS", "RECENT POSTS", "OPEN SOURCE"]


def render_banner(handle, w):
    # Top Bar
    handle.goto(0, 0)
    handle.set_fg(Color.Green)
    handle.prints("≡ ")
    handle.set_fg(Color.Reset)
    handle.prints("MENU")
    handle.goto(w - 4, 0)
    handle.set_fg(Color.Red)
    handle.prints("[✖]")
    handle.set_fg(Color.Reset)
    # Horiz Rule
    handle.goto(0, 1)
    handle.set_fx(Effect.Dim)
    handle.prints("─" * w)
    handle.set_fx(Effect.Reset)

def render_greeting(handle, w, h):
    # Greeting
    from_top = h // 3
    # Print Title
    title = "Dave Ho CLI v0.beta"
    from_col = w // 2 - (len(title) // 2) - 4
    handle.goto(from_col, from_top)
    handle.prints(title)
    # Print Subtitle
    subtitle = "\\\'dæv • \'hoh\\  賀毅超  (he, him, his)"
    from_col = w // 2 - (len(subtitle) + 3) // 2 - 4
    handle.goto(from_col, from_top + 1)
    handle.prints(subtitle[:13])
    handle.set_fx(Effect.Dim)
    handle.prints(subtitle[13:20])
    handle.set_fx(Effect.Reset)
    handle.prints(subtitle[20:])

def render_menu(handle):
    selected = 0
    handle.goto(0, 1)
    handle.set_fx(Effect.Dim)
    handle.prints("┌" + "─" * 14 + "┬")
    for i, section in enumerate(SECTIONS):
        handle.goto(0, 2 + i)
        if i == 0:
            handle.prints("│")
            handle.set_fx(Effect.Reset)
            handle.set_fg(Color.Green)
            handle.prints("▎")
            handle.set_bg(Color.White)
            handle.set_fg(Color.Black)
            handle.prints(section.ljust(13))
            handle.set_styles(Color.Reset, Color.Reset, Effect.Dim)
            handle.prints("│")
        else:
            handle.prints("│")
            handle.set_fx(Effect.Reset)
            handle.set_fg(Color.Green)
            handle.prints("▎")
            handle.set_fg(Color.Reset)
            handle.prints(section.ljust(13))
            handle.set_fx(Effect.Dim)
            handle.prints("│")
    handle.goto(0, 7)
    handle.printf("└" + "─" * 14 + "┘")

def render_splash_section(handle, w, h):
    from_top = h // 3 + 3
    midpoint = w // 2
    instructions = "Navigate with ↑↓ Press <ENTER> to view"
    from_col = midpoint - len(instructions) // 2 - 4
    handle.goto(from_col, from_top)
    handle.prints(instructions[:23])
    handle.set_fg(Color.Cyan)
    handle.prints(instructions[23:30])
    handle.set_fg(Color.Reset)
    handle.prints(instructions[30:])
    # Initial output of Sections
    handle.set_fx(Effect.Dim)
    from_col = midpoint - 10
    from_top = from_top + 2
    for i, section in enumerate(SECTIONS):
        handle.goto(from_col, from_top + i)
        if i == 0:
            handle.set_fx(Effect.Reset)
            handle.set_fg(Color.Green)
            handle.prints(section)
            handle.set_fg(Color.Reset)
            handle.set_fx(Effect.Dim)
        else:
            handle.prints(section)
    handle.set_fx(Effect.Reset)
    # Tab instructions
    instructions = "Press <TAB> any time to toggle the navigation menu"
    from_col = w // 2 - len(instructions) // 2 - 4
    from_top = from_top + 7
    handle.goto(from_col, from_top)
    handle.prints(instructions[:6])
    handle.set_fg(Color.Cyan)
    handle.prints(instructions[6:11])
    handle.set_fg(Color.Reset)
    handle.printf(instructions[11:])

async def handle_splash_section(handle, w, h, is_running, section_queue):
    from_col = w // 2 - 10
    # NOTE: from previous render
    from_top = h // 3 + 3 + 2
    selected = 0
    while is_running[0]:
        if not section_queue.empty():
            if await section_queue.get() != 0:
                break
        await asyncio.sleep(DELAY)
        evt = handle.poll_latest_async()
        if evt is None: continue
        handle.set_fg(Color.Reset)
        if evt.kind() == InputEvent.Up:
            if selected == 0:
                # At top, so wrap around
                handle.goto(from_col, from_top)
                handle.set_fx(Effect.Dim)
                handle.prints(SECTIONS[0])
                handle.goto(from_col, from_top + 4)
                handle.set_fx(Effect.Reset)
                handle.set_fg(Color.Green)
                handle.printf(SECTIONS[4])
                selected = 4
            else:
                # Move up one
                handle.goto(from_col, from_top + selected)
                handle.set_fx(Effect.Dim)
                handle.prints(SECTIONS[selected])
                selected = selected - 1
                handle.goto(from_col, from_top + selected)
                handle.set_fx(Effect.Reset)
                handle.set_fg(Color.Green)
                handle.printf(SECTIONS[selected])
        elif evt.kind() == InputEvent.Down:
            if selected == 4:
                # At bottom, so wrap around
                handle.goto(from_col, from_top + 4)
                handle.set_fx(Effect.Dim)
                handle.prints(SECTIONS[4])
                handle.goto(from_col, from_top)
                handle.set_fx(Effect.Reset)
                handle.set_fg(Color.Green)
                handle.printf(SECTIONS[0])
                selected = 0
            else:
                # Move down one
                handle.goto(from_col, from_top + selected)
                handle.set_fx(Effect.Dim)
                handle.prints(SECTIONS[selected])
                selected = selected + 1
                handle.goto(from_col, from_top + selected)
                handle.set_fx(Effect.Reset)
                handle.set_fg(Color.Green)
                handle.printf(SECTIONS[selected])
        elif evt.kind() == InputEvent.Enter:
            return selected
        else:
            pass

async def handle_menu(handle, is_running, section_queue):
    is_open = False
    selected = 0
    while is_running[0]:
        await asyncio.sleep(DELAY)
        evt = handle.poll_latest_async();
        if evt is None: continue
        if evt.kind() == InputEvent.Tab:
            if not is_open:
                render_menu(handle)
                is_open = True
                handle.lock()
            else:
                # close menu
                handle.unlock()
                is_open = False
                handle.reset_styles()
                handle.goto(0, 1)
                handle.set_fx(Effect.Dim)
                handle.prints("─" * 16)
                handle.set_fx(Effect.Reset)
                for i, section in enumerate(SECTIONS):
                    handle.goto(0, 2 + i)
                    handle.prints(" " * 16)
                handle.goto(0, 7)
                handle.prints(" " * 16)
                handle.flush()

        elif evt.kind() == InputEvent.MousePressLeft:
            (col, row) = evt.data()
            if row == 0 and 0 <= col <= 5 and not is_open:
                render_menu(handle)
                is_open = True
                handle.lock()
            elif row == 0 and 0 <= col <= 5 and is_open:
                # close menu
                handle.unlock()
                is_open = False
                handle.reset_styles()
                handle.goto(0, 1)
                handle.set_fx(Effect.Dim)
                handle.prints("─" * 16)
                handle.set_fx(Effect.Reset)
                for i, section in enumerate(SECTIONS):
                    handle.goto(0, 2 + i)
                    handle.prints(" " * 16)
                handle.goto(0, 7)
                handle.prints(" " * 16)
                handle.flush()

        elif evt.kind() == InputEvent.Esc and is_open:
            handle.unlock()
            is_open = False
            # TODO: needs to not only clear itself, but
            # restore the previous contents below it
            # NOTE: This restores splash screen only:
            handle.reset_styles()
            handle.goto(0, 1)
            handle.set_fx(Effect.Dim)
            handle.prints("─" * 16)
            handle.set_fx(Effect.Reset)
            for i, section in enumerate(SECTIONS):
                handle.goto(0, 2 + i)
                handle.prints(" " * 16)
            handle.goto(0, 7)
            handle.prints(" " * 16)
            handle.flush()

        elif evt.kind() == InputEvent.Up and is_open:
            if selected == 0:
                handle.goto(0, 2)
                handle.prints("│")
                handle.set_fx(Effect.Reset)
                handle.set_fg(Color.Green)
                handle.prints("▎")
                handle.set_fg(Color.Reset)
                handle.prints(SECTIONS[selected].ljust(13))
                handle.set_fx(Effect.Dim)
                handle.prints("│")
                selected = 4
                handle.goto(0, 2 + 4)
                handle.prints("│")
                handle.set_fx(Effect.Reset)
                handle.set_fg(Color.Green)
                handle.prints("▎")
                handle.set_bg(Color.White)
                handle.set_fg(Color.Black)
                handle.prints(SECTIONS[selected].ljust(13))
                handle.set_styles(Color.Reset, Color.Reset, Effect.Dim)
                handle.prints("│")
                handle.flush()
            else:
                handle.goto(0, 2 + selected)
                handle.prints("│")
                handle.set_fx(Effect.Reset)
                handle.set_fg(Color.Green)
                handle.prints("▎")
                handle.set_fg(Color.Reset)
                handle.prints(SECTIONS[selected].ljust(13))
                handle.set_fx(Effect.Dim)
                handle.prints("│")
                selected = selected - 1
                handle.goto(0, 2 + selected)
                handle.prints("│")
                handle.set_fx(Effect.Reset)
                handle.set_fg(Color.Green)
                handle.prints("▎")
                handle.set_bg(Color.White)
                handle.set_fg(Color.Black)
                handle.prints(SECTIONS[selected].ljust(13))
                handle.set_styles(Color.Reset, Color.Reset, Effect.Dim)
                handle.prints("│")
                handle.flush()

        elif evt.kind() == InputEvent.Down and is_open:
            if selected == 4:
                handle.goto(0, 2 + 4)
                handle.prints("│")
                handle.set_fx(Effect.Reset)
                handle.set_fg(Color.Green)
                handle.prints("▎")
                handle.set_fg(Color.Reset)
                handle.prints(SECTIONS[selected].ljust(13))
                handle.set_fx(Effect.Dim)
                handle.prints("│")
                selected = 0
                handle.goto(0, 2)
                handle.prints("│")
                handle.set_fx(Effect.Reset)
                handle.set_fg(Color.Green)
                handle.prints("▎")
                handle.set_bg(Color.White)
                handle.set_fg(Color.Black)
                handle.prints(SECTIONS[selected].ljust(13))
                handle.set_styles(Color.Reset, Color.Reset, Effect.Dim)
                handle.prints("│")
                handle.flush()
            else:
                handle.goto(0, 2 + selected)
                handle.prints("│")
                handle.set_fx(Effect.Reset)
                handle.set_fg(Color.Green)
                handle.prints("▎")
                handle.set_fg(Color.Reset)
                handle.prints(SECTIONS[selected].ljust(13))
                handle.set_fx(Effect.Dim)
                handle.prints("│")
                selected = selected + 1
                handle.goto(0, 2 + selected)
                handle.prints("│")
                handle.set_fx(Effect.Reset)
                handle.set_fg(Color.Green)
                handle.prints("▎")
                handle.set_bg(Color.White)
                handle.set_fg(Color.Black)
                handle.prints(SECTIONS[selected].ljust(13))
                handle.set_styles(Color.Reset, Color.Reset, Effect.Dim)
                handle.prints("│")
                handle.flush()

        elif evt.kind() == InputEvent.Enter and is_open:
            handle.unlock()
            is_open = False
            await section_queue.put(selected)
        else:
            pass

async def handle_quit(handle, w, is_running):
    (w, _) = handle.size()
    while is_running[0]:
        await asyncio.sleep(DELAY)
        evt = handle.poll_latest_async()
        if evt is None: continue
        if evt.kind() == InputEvent.Ctrl:
            if evt.data() == 'q':
                is_running[0] = False
                break
        elif evt.kind() == InputEvent.MousePressLeft:
            (col, row) = evt.data()
            if row == 0 and (w - 4) <= col <= (w - 2):
                is_running[0] = False
                break

async def main():
    is_running = [True, ]
    with Dispatcher() as dispatch:
        dispatch.switch()
        dispatch.raw()
        dispatch.enable_mouse()
        dispatch.hide_cursor()

        with dispatch.listen() as l, dispatch.listen() as q, dispatch.listen() as m:
            (w, h) = l.size()
            # Render initial screen
            render_banner(l, w)
            render_greeting(l, w, h)
            render_splash_section(l, w, h)
            l.flush()
            # Async stuff begins:
            section_queue = asyncio.Queue(1)
            # Initial splash screen handle:
            splash_task = asyncio.create_task(
                handle_splash_section(l, w, h, is_running, section_queue))
            # The below live for the lifetime of the application
            menu_task = asyncio.create_task(
                handle_menu(m, is_running, section_queue))
            shutdown_task = asyncio.create_task(
                handle_quit(q, w, is_running))
            await splash_task

            # NOTE: HOW TO RUN THE PART THAT HANDLES PAGE TOGGLING??
            # Render sections based on queue
            while is_running[0]:
                try:
                    section = await section_queue.get_nowait()
                except asyncio.QueueEmpty:
                    await asyncio.sleep(DELAY)
                    continue
                except:
                    break

                if section == 0:
                    await render_splash_section(l, w, h, is_running, section_queue)
                elif section == 1:
                    # ABOUT
                    pass
                elif section == 2:
                    # EXPERIENCE
                    pass
                elif section == 3:
                    # SKILLS
                    pass
                elif section == 4:
                    # RECENT POSTS
                    pass
                elif section == 5:
                    # OPEN SOURCE
                    pass
                else:
                    pass

            await menu_task
            await shutdown_task

        dispatch.disable_mouse()
        dispatch.show_cursor()
        time.sleep(0.5)
        dispatch.cook()
        dispatch.switch_to(0)

    # <-- Dispatcher drops and ends


if __name__ == "__main__":
    asyncio.run(main())
