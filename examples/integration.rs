extern crate tuitty;

use std::thread;
use std::time::Duration;

use tuitty::common::{
    traits::*,
    enums::{ Color, Clear, Style },
};

use tuitty::{
    interface,
    terminal::{ Terminal, CommonTerminal },
};

use std::sync::{
    Arc, atomic::{ AtomicBool, Ordering },
};

use std::io::{ stdin, stdout, Result, BufRead, Write };

#[cfg(unix)]
use tuitty::terminal::unix::size;

#[cfg(windows)]
use tuitty::terminal::wincon::{ Handle, screen::size };


fn test_cursor_goto(confirm: &Arc<AtomicBool>) {
    let tty = CommonTerminal::new();
    tty.goto(1, 1);
    tty.printf("Test case:");
    tty.goto(1, 2);
    tty.printf("..........");
    tty.goto(1, 3);
    tty.printf("..........");

    let course: [(i16, i16); 4] = [(12, 3), (14, 3), (14, 1), (12, 1)];
    let mut i = 0;
    loop {
        if confirm.load(Ordering::SeqCst) {
            break
        }
        let (x, y) = course[i];
        tty.goto(x, y);
        tty.printf("#");
        thread::sleep(Duration::from_millis(400));
        tty.goto(x, y);
        tty.printf(" ");
        i += 1;
        if i > 3 { i = 0; }
    }
}

fn test_cursor_move(confirm: &Arc<AtomicBool>) {
    let tty = CommonTerminal::new();
    tty.goto(1, 1);
    tty.printf("Test case:");
    tty.goto(1, 2);
    tty.printf("..........");
    tty.goto(12, 1);
    loop {
        if confirm.load(Ordering::SeqCst) {
            break
        }
        tty.right(1);
        tty.printf("%");
        thread::sleep(Duration::from_millis(400));
        tty.left(1);
        tty.printf(" ");
        tty.left(1);

        tty.down(1);
        tty.printf("%");
        thread::sleep(Duration::from_millis(400));
        tty.left(1);
        tty.printf(" ");
        tty.left(1);

        tty.left(1);
        tty.printf("%");
        thread::sleep(Duration::from_millis(400));
        tty.left(1);
        tty.printf(" ");
        tty.left(1);

        tty.up(1);
        tty.printf("%");
        thread::sleep(Duration::from_millis(400));
        tty.left(1);
        tty.printf(" ");
        tty.left(1);
    }
}

fn test_screen_clear(confirm: &Arc<AtomicBool>) {
    let tty = CommonTerminal::new();
    tty.show_cursor();
    let content = "Contrary to popular belief, Lorem Ipsum is not simply \
                   random text. It has roots in a piece of classical Latin \
                   literature from 45 BC, making it over 2000 years old.";

    loop {
        // Clear Current Line.
        if confirm.load(Ordering::SeqCst) {
            tty.hide_cursor();
            break
        }
        tty.goto(1, 1);
        tty.clear(Clear::CurrentLn);
        tty.printf("Test case (CurrLn):");
        tty.goto(0, 2);
        tty.printf(content);
        tty.goto(18, 2);
        tty.flush();
        thread::sleep(Duration::from_millis(800));
        tty.clear(Clear::CurrentLn);
        tty.flush();
        thread::sleep(Duration::from_millis(800));

        // Clear Until New Line.
        if confirm.load(Ordering::SeqCst) {
            tty.hide_cursor();
            break
        }
        tty.goto(1, 1);
        tty.clear(Clear::CurrentLn);
        tty.printf("Test case (NewLn):");
        tty.goto(0, 2);
        tty.printf(content);
        tty.goto(8, 2);
        tty.flush();
        thread::sleep(Duration::from_millis(800));
        tty.clear(Clear::NewLn);
        tty.flush();
        thread::sleep(Duration::from_millis(800));

        // Clear Up from Cursor Location.
        // Then clear the top section to prep
        // for the next Clear variant.
        if confirm.load(Ordering::SeqCst) {
            tty.hide_cursor();
            break
        }
        tty.goto(1, 1);
        tty.clear(Clear::CurrentLn);
        tty.printf("Test case (CursUp):");
        tty.goto(0, 2);
        tty.printf(content);
        tty.goto(21, 3);
        tty.flush();
        thread::sleep(Duration::from_millis(800));
        tty.clear(Clear::CursorUp);
        tty.flush();
        thread::sleep(Duration::from_millis(800));
        tty.goto(0, 4);
        tty.clear(Clear::CursorUp);
        tty.flush();

        // Clear Down from Cursor Location.
        // Then clear the bottom section to reset
        // the view for the loop to restart.
        if confirm.load(Ordering::SeqCst) {
            tty.hide_cursor();
            break
        }
        let (_, h) = size();
        let row = h / 2 + 8 + 2;
        tty.goto(1, row);
        tty.clear(Clear::CurrentLn);
        tty.printf("Test case (CurDn):");
        tty.goto(0, row + 1);
        tty.printf(content);
        tty.goto(21, row + 1);
        tty.flush();
        thread::sleep(Duration::from_millis(800));
        tty.clear(Clear::CursorDn);
        tty.flush();
        thread::sleep(Duration::from_millis(800));
        tty.goto(1, row);
        tty.clear(Clear::CursorDn);
        tty.flush();
    }
}

fn test_cursor_markload(tty: &mut Terminal, confirm: &Arc<AtomicBool>) {
    // let tty = CommonTerminal::new();
    let content = "But mutable references have one big restriction: you \
                   can have only one mutable reference to a particular \
                   piece of data in a particular scope. This restriction \
                   allows for mutation but in a very controlled fashion.\
                   It’s something that new Rustaceans struggle with, \
                   because most languages let you mutate whenever you’d like.";
    tty.show_cursor();
    tty.goto(1, 1);
    tty.printf("Test case:");

    tty.goto(0, 2);
    tty.printf(content);

    tty.goto(8, 3);
    tty.flush();

    'outer: loop {
        if confirm.load(Ordering::SeqCst) {
            tty.hide_cursor();
            break 'outer
        }

        for i in 9..15 {
            if confirm.load(Ordering::SeqCst) {
                tty.hide_cursor();
                break 'outer
            }
            tty.goto(i, 3);
            tty.flush();
            thread::sleep(Duration::from_millis(1200));
            if i == 12 {
                tty.mark_pos(); // 'i'
            }
        }

        tty.goto(48, 5);
        tty.flush();
        thread::sleep(Duration::from_millis(1200));

        if confirm.load(Ordering::SeqCst) {
            tty.hide_cursor();
            break 'outer
        }
        tty.load_pos();
        tty.flush();
        thread::sleep(Duration::from_millis(2400));
    }
}


fn main() {
    // Setup shared elements:
    let mut tty = Terminal::init();
    let mut input = tty.read_async();
    let mut alertbox = interface::AlertBox::new();
    let confirm = Arc::new(AtomicBool::new(false));

    // Do the test in the alternate screen.
    // (imdaveho) NOTE: This also confirms that the following work:
    // * enable_alt
    // * enable_raw
    // * mouse_mode
    // * hide_cursor
    tty.switch();
    tty.raw();
    tty.hide_cursor();
    tty.enable_mouse();
    tty.flush();

    // Test #1: Check Cursor (goto):
    // alertbox.set_content("Is the cursor going to the correct locations?");
    // alertbox.render();
    // let refc = confirm.clone();
    // let goto_test = thread::spawn(move || {
    //     test_cursor_goto(&refc);
    // });
    // let goto_res = alertbox.handle(&mut input);
    // confirm.store(true, Ordering::SeqCst);
    // goto_test.join().unwrap();
    // // Failure message on main screen.
    // if !goto_res {
    //     tty.to_main();
    //     panic!("Cursor (goto) test failed.");
    // }

    // // Reset for next test.
    // tty.clear(Clear::All);
    // confirm.store(false, Ordering::SeqCst);

    // Test # 2: Check Cursor (moveX):
    // alertbox.set_content("Is the cursor moving by 1 in all directions?");
    // alertbox.render();
    // let refc = confirm.clone();
    // let move_test = thread::spawn(move || {
    //     test_cursor_move(&refc);
    // });
    // let move_res = alertbox.handle(&mut input);
    // confirm.store(true, Ordering::SeqCst);
    // move_test.join().unwrap();
    // // Failure message on main screen.
    // if !move_res {
    //     tty.to_main();
    //     panic!("Cursor (moveX) test failed.");
    // }

    // // Reset for next test.
    // tty.clear(Clear::All);
    // confirm.store(false, Ordering::SeqCst);

    // Test #3: Check Screen (clear):
    // alertbox.set_content("Is the screen clearing correctly?");
    // alertbox.render();
    // let refc = confirm.clone();
    // let clear_test = thread::spawn(move || {
    //     test_screen_clear(&refc)
    // });
    // let clear_res = alertbox.handle(&mut input);
    // confirm.store(true, Ordering::SeqCst);
    // clear_test.join().unwrap();
    // // Failure message on main screen.
    // if !clear_res {
    //     tty.to_main();
    //     panic!("Screen (clear) test failed.");
    // }

    // // Reset for next test.
    // tty.clear(Clear::All);
    // confirm.store(false, Ordering::SeqCst);

    // Test #4: Check Cursor (mark/load):
    alertbox.set_content("Did the cursor reload at `i`?");
    alertbox.render();
    let refc = confirm.clone();
    let markload_test = thread::spawn(move || {
        test_cursor_markload(&mut tty, &refc);
        return tty;
    });
    let markload_res = alertbox.handle(&mut input);
    confirm.store(true, Ordering::SeqCst);
    let mut tty = markload_test.join().unwrap();
    // Failure message on main screen.
    if !markload_res {
        tty.to_main();
        panic!("Cursor (mark/load) test failed.");
    }

    // Reset for next test.
    tty.clear(Clear::All);
    confirm.store(false, Ordering::SeqCst);

    // Test #5: ...


    // Reset for next test.
    tty.clear(Clear::All);
    confirm.store(false, Ordering::SeqCst);


    // Finish!
    tty.to_main();
    tty.printf("\nIntegration test complete!\n");
}
