import asyncio
from tuitty.ffi import Dispatcher, InputEvent, Color, Effect


def render_banner(handle):
    (w, _) = handle.size()
    handle.goto(0, 0)
    # handle.set_fx(Effect.Dim)
    handle.set_fg(Color.Green)
    handle.prints("≡ ")
    handle.set_fg(Color.Reset)
    # handle.set_fx(Effect.Reset)
    handle.prints("Menu")
    handle.goto(w - 4, 0)
    handle.set_fg(Color.Red)
    handle.prints("[✖]")
    handle.set_fg(Color.Reset)
    # Print Bottom Row
    handle.goto(0, 1)
    handle.set_fx(Effect.Dim)
    handle.prints("─" * w)
    handle.set_fx(Effect.Reset)
    handle.flush()

def render_opener(handle):
    (w, h) = handle.size()
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
    handle.printf(subtitle[20:])

async def render_splash_section(handle, is_running, section_queue):
    (w, h) = handle.size()
    # (_, row) = handle.coord()
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
    sections = ["ABOUT", "EXPERIENCE", "SKILLS", "RECENT POSTS", "OPEN SOURCE"]
    handle.set_fx(Effect.Dim)
    from_col = midpoint - 10
    from_top = from_top + 2
    for i, section in enumerate(sections):
        handle.goto(from_col, from_top + i)
        if i == 0:
            handle.set_fx(Effect.Reset)
            handle.prints(section)
            handle.set_fx(Effect.Dim)
        else:
            handle.prints(section)
    handle.set_fx(Effect.Reset)
    # Tab instructions
    instructions = "Press <TAB> any time to open the navigation menu"
    from_col = w // 2 - len(instructions) // 2 - 4
    from_top = from_top + 7
    handle.goto(from_col, from_top)
    handle.prints(instructions[:6])
    handle.set_fg(Color.Cyan)
    handle.prints(instructions[6:11])
    handle.set_fg(Color.Reset)
    handle.printf(instructions[11:])
    # Handle events
    from_col = w // 2 - 10
    from_top = from_top - 7
    selected = 0
    while is_running[0]:
        if not section_queue.empty():
            if await section_queue.get() != 0:
                break
        await asyncio.sleep(0.05)
        evt = handle.poll_latest_async()
        if evt is None: continue
        if evt.kind() == InputEvent.Up:
            if selected == 0:
                # At top, so wrap around
                handle.goto(from_col, from_top)
                handle.set_fx(Effect.Dim)
                handle.prints(sections[0])
                handle.goto(from_col, from_top + 4)
                handle.set_fx(Effect.Reset)
                handle.printf(sections[4])
                selected = 4
            else:
                # Move up one
                handle.goto(from_col, from_top + selected)
                handle.set_fx(Effect.Dim)
                handle.prints(sections[selected])
                selected = selected - 1
                handle.goto(from_col, from_top + selected)
                handle.set_fx(Effect.Reset)
                handle.printf(sections[selected])
        elif evt.kind() == InputEvent.Down:
            if selected == 4:
                # At bottom, so wrap around
                handle.goto(from_col, from_top + 4)
                handle.set_fx(Effect.Dim)
                handle.prints(sections[4])
                handle.goto(from_col, from_top)
                handle.set_fx(Effect.Reset)
                handle.printf(sections[0])
                selected = 0
            else:
                # Move down one
                handle.goto(from_col, from_top + selected)
                handle.set_fx(Effect.Dim)
                handle.prints(sections[selected])
                selected = selected + 1
                handle.goto(from_col, from_top + selected)
                handle.set_fx(Effect.Reset)
                handle.printf(sections[selected])
        elif evt.kind() == InputEvent.Enter:
            return selected
        else:
            pass

def render_menu(handle):
    sections = ["ABOUT", "EXPERIENCE", "SKILLS", "RECENT POSTS", "OPEN SOURCE"]
    selected = 0
    handle.set_bg(Color.DarkGreen)
    for i, section in enumerate(sections):
        handle.goto(1, 1 + i)
        if selected == 0:
            handle.set_fx(Effect.Reverse)
            handle.prints(" " + section.ljust(13))
            handle.set_fx(Effect.Reset)
        else:
            handle.prints(" " + section.ljust(13))
    handle.set_bg(Color.Reset)
    handle.flush()

async def handle_menu_navigation(handle, is_running, section_queue):
    is_open = False
    sections = ["ABOUT", "EXPERIENCE", "SKILLS", "RECENT POSTS", "OPEN SOURCE"]
    selected = 0
    while is_running[0]:
        await asyncio.sleep(0.05)
        evt = handle.poll_latest_async();
        if evt is None: continue
        if evt.kind() == InputEvent.Tab and not is_open:
            # TODO: maybe the below should return contents
            # of what was overwritten so that it can restore
            render_menu(handle)
            is_open = True
            handle.lock()
        elif evt.kind() == InputEvent.Right and is_open:
            # TODO: Seems like ESC is not being detected
            # TODO: needs to not only clear itself, but
            # restore the previous contents below it
            handle.unlock()
            is_open = False
        elif evt.kind() == InputEvent.Up and is_open:
            if selected == 0:
                handle.goto(1, 1)
                handle.set_fx(Effect.Reset)
                handle.prints(" " + sections[selected].ljust(13))
                selected = 4
                handle.goto(1, 5)
                handle.set_fx(Effect.Reverse)
                handle.prints(" " + sections[selected].ljust(13))
                handle.set_fx(Effect.Reset)
                handle.flush()
            else:
                handle.goto(1, 1 + selected)
                handle.set_fx(Effect.Reset)
                handle.prints(" " + sections[selected].ljust(13))
                selected = selected - 1
                handle.goto(1, 1 + selected)
                handle.set_fx(Effect.Reverse)
                handle.prints(" " + sections[selected].ljust(13))
                handle.set_fx(Effect.Reset)
                handle.flush()
        elif evt.kind() == InputEvent.Down and is_open:
            if selected == 4:
                handle.goto(1, 5)
                handle.set_fx(Effect.Reset)
                handle.prints(" " + sections[selected].ljust(13))
                selected = 0
                handle.goto(1, 1)
                handle.set_fx(Effect.Reverse)
                handle.prints(" " + sections[selected].ljust(13))
                handle.set_fx(Effect.Reset)
                handle.flush()
            else:
                handle.goto(1, 1 + selected)
                handle.set_fx(Effect.Reset)
                handle.prints(" " + sections[selected].ljust(13))
                selected = selected + 1
                handle.goto(1, 1 + selected)
                handle.set_fx(Effect.Reverse)
                handle.prints(" " + sections[selected].ljust(13))
                handle.set_fx(Effect.Reset)
                handle.flush()
        elif evt.kind() == InputEvent.Enter and is_open:
            handle.unlock()
            is_open = False
            await section_queue.put(selected)
        else:
            pass

async def handle_quit(handle, is_running):
    (w, _) = handle.size()
    while is_running[0]:
        await asyncio.sleep(0.05)
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

        with dispatch.listen() as h, dispatch.listen() as q, dispatch.listen() as m:
            render_banner(h)
            render_opener(h)
            # Async stuff begins:
            section_queue = asyncio.Queue(1)
            splash_task = asyncio.create_task(
                render_splash_section(h, is_running, section_queue))
            shutdown_task = asyncio.create_task(
                handle_quit(q, is_running))
            menu_task = asyncio.create_task(
                handle_menu_navigation(m, is_running, section_queue))
            await splash_task
            # Render sections based on queue
            while is_running[0]:
                try:
                    section = await section_queue.get_nowait()
                except asyncio.QueueEmpty:
                    await asyncio.sleep(0.05)
                    continue
                except:
                    break

                if section == 0:
                    await render_splash_section(h, is_running, section_queue)
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
        dispatch.cook()
        await asyncio.sleep(0.5)
        dispatch.switch_to(0)

    # <-- Dispatcher drops and ends


if __name__ == "__main__":
    asyncio.run(main())
