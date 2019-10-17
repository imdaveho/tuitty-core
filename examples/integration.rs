// extern crate tuitty;

// use std::thread;
// use std::time::Duration;

// use tuitty::{
//     common::{
//         traits::*,
//         enums::{ Clear, }
//     },
//     interface::AlertBox,
//     terminal::Terminal,
// };

// use std::sync::{
//     Arc, atomic::{ AtomicBool, Ordering },
// };


// fn main() {
//     let mut tty = Terminal::init();
//     // Do the test in the alternate screen.
//     // (imdaveho) NOTE: This also confirms that the following work:
//     // * enable_alt
//     // * enable_raw
//     // * mouse_mode
//     // * hide_cursor
//     tty.switch();
//     tty.raw();
//     tty.hide_cursor();
//     tty.enable_mouse();
//     tty.flush();

//     // Test Cursor:
//     // test_goto(&mut tty);
//     // test_move(&mut tty);
//     test_marker(&mut tty);


//     // Finish!
//     tty.terminate();
//     tty.printf("\nIntegration test complete!\n");
// }


// fn test_goto(tty: &mut Terminal) {
//     // Initialize AlertBox and AsyncReader.
//     let mut alert = AlertBox::new();
//     alert.set_content("Is the cursor going to the correct locations?");
//     // Setup Thread-safe.
//     let alert = Arc::new(alert);
//     let refct = Arc::clone(&alert);
//     let end = Arc::new(AtomicBool::new(false));
//     let end_sig = end.clone();
//     // **************************
//     // Spawn thread for AlertBox.
//     // **************************
//     let result = thread::spawn(move || {
//         let res = refct.handle();
//         end_sig.store(true, Ordering::SeqCst);
//         return res;
//     });

//     // ***************************
//     // Run main test case in loop.
//     // ***************************
//     let coords: [(i16, i16); 4] = [
//         (20, 3), (24, 3), (24, 1), (20, 1)
//     ];

//     'test: loop {
//         for (c, r) in coords.iter() {
//             if end.load(Ordering::SeqCst) { break 'test }
//             let (col, row) = (*c, *r);
//             // Print test case.
//             tty.goto(1, 1);
//             tty.printf(&format!("Test case ({}, {}):", col, row));
//             tty.goto(1, 2);
//             tty.printf("..................");
//             tty.goto(1, 3);
//             tty.printf("..................");
//             // Print cursor from coords.
//             tty.goto(col, row);
//             tty.printf("#");
//             thread::sleep(Duration::from_millis(200));
//             tty.goto(col, row);
//             tty.printf(" ");
//         }
//         alert.render();
//     }

//     if !result.join().unwrap() {
//         tty.terminate();
//         panic!("Cursor test: [goto] failed.");
//     }

//     tty.clear(Clear::All);
//     tty.flush();
// }

// fn test_move(tty: &mut Terminal) {
//     // Initialize AlertBox and AsyncReader.
//     let mut alert = AlertBox::new();
//     alert.set_content(
//         "Is the cursor moving by one (1) cell in a clockwise fashion?");
//     // Setup Thread-safe.
//     let alert = Arc::new(alert);
//     let refct = Arc::clone(&alert);
//     let end = Arc::new(AtomicBool::new(false));
//     let end_sig = end.clone();
//     // **************************
//     // Spawn thread for AlertBox.
//     // **************************
//     let result = thread::spawn(move || {
//         let res = refct.handle();
//         end_sig.store(true, Ordering::SeqCst);
//         return res;
//     });

//     // ***************************
//     // Run main test case in loop.
//     // ***************************
//     'test: loop {
//         if end.load(Ordering::SeqCst) { break 'test }
//         // Print test case.
//         tty.goto(1, 1);
//         tty.printf("Test case (right/down/left/up):");
//         tty.goto(1, 2);
//         tty.printf("...............................");
//         tty.goto(33, 1);
//         // Move cursor clockwise.
//         tty.right();
//         tty.printf("%");
//         thread::sleep(Duration::from_millis(200));
//         tty.left();
//         tty.printf(" ");
//         tty.left();
//         if end.load(Ordering::SeqCst) { break 'test }

//         tty.down();
//         tty.printf("%");
//         thread::sleep(Duration::from_millis(200));
//         tty.left();
//         tty.printf(" ");
//         tty.left();
//         if end.load(Ordering::SeqCst) { break 'test }

//         tty.left();
//         tty.printf("%");
//         thread::sleep(Duration::from_millis(200));
//         tty.left();
//         tty.printf(" ");
//         tty.left();
//         if end.load(Ordering::SeqCst) { break 'test }

//         tty.up();
//         tty.printf("%");
//         thread::sleep(Duration::from_millis(200));
//         tty.left();
//         tty.printf(" ");
//         tty.left();
//         if end.load(Ordering::SeqCst) { break 'test }

//         alert.render();
//     }

//     if !result.join().unwrap() {
//         tty.terminate();
//         panic!("Cursor test: [move] failed.");
//     }

//     tty.clear(Clear::All);
//     tty.flush();
// }

// fn test_marker(tty: &mut Terminal) {
//     // Initialize AlertBox and AsyncReader.
//     let mut alert = AlertBox::new();
//     alert.set_content("Did the cursor re-position at `i` in part[i]cular?");
//     // Setup Thread-safe.
//     let alert = Arc::new(alert);
//     let refct = Arc::clone(&alert);
//     let end = Arc::new(AtomicBool::new(false));
//     let end_sig = end.clone();
//     // **************************
//     // Spawn thread for AlertBox.
//     // **************************
//     let result = thread::spawn(move || {
//         let res = refct.handle();
//         end_sig.store(true, Ordering::SeqCst);
//         return res;
//     });

//     tty.goto(1, 1);
//     tty.printf("Test case (mark/load):");
//     tty.goto(1, 2);
//     tty.printf("......................");
//     let content = "But mutable references have one big restriction: you \
//                    can have only one mutable reference to a particular \
//                    piece of data in a particular scope. This restriction \
//                    allows for mutation but in a very controlled fashion.\
//                    It’s something that new Rustaceans struggle with, \
//                    because most languages let you mutate whenever \
//                    you’d like.";
//     tty.goto(0, 3);
//     tty.show_cursor();
//     tty.printf(content);
//     // ***************************
//     // Run main test case in loop.
//     // ***************************
//     'test: loop {
//         if end.load(Ordering::SeqCst) { break 'test }
//         tty.goto(8, 4);
//         tty.flush();
//         for i in 9..15 {
//             if end.load(Ordering::SeqCst) { break 'test }
//             tty.goto(i, 4);
//             tty.flush();
//             thread::sleep(Duration::from_millis(1200));
//             if i == 12 {
//                 tty.mark_pos(); // mark at the `i` in part[i]cular.
//             }
//         }
//         tty.goto(48, 5);
//         tty.flush();
//         thread::sleep(Duration::from_millis(1200));
//         if end.load(Ordering::SeqCst) { break 'test }
//         tty.load_pos();
//         tty.flush();
//         thread::sleep(Duration::from_millis(1200));

//         alert.render();
//     }

//     tty.hide_cursor();
//     tty.flush();

//     if !result.join().unwrap() {
//         tty.terminate();
//         panic!("Cursor test: [mark/load] failed.");
//     }

//     tty.clear(Clear::All);
//     tty.flush();
// }

// // fn test_clear(tty: &mut Terminal) {
// //     ()
// // }
