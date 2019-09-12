use super::Tty;
use std::thread;
use std::time::Duration;


// fn _prompt(tty: &mut Tty, text: &str, err: &str) {
//     // TODO: make the below more ergonomic.
//     let (w, h) = tty.screen_size();
//     let linear = "-".repeat(w as usize);
//     tty.goto(0, h - 3);
//     tty.printf(&linear);
//     tty.goto(0, h - 2);
//     tty.flush();
//     tty.clear("cursordn");
//     tty.flush();
//     tty.goto(0, h - 2);
//     tty.flush();
//     thread::sleep(Duration::from_millis(400));
//     tty.prints("[");
//     tty.set_fg("green");
//     tty.prints("?");
//     tty.reset();
//     tty.printf(&format!("] {} (Y/N)? ", text));

//     'prompt: loop {
//         let c = tty.read_char();
//         match c {
//             'N' | 'n' => {
//                 tty.clear("all");
//                 panic!("{}", err)
//             }
//             'Y' | 'y' => {
//                 break 'prompt
//             }

//             _ => ()
//         }
//     }
// }


#[test]
fn test_screen() {
    let delay = 800;
    let mut tty = Tty::init();


    tty.switch();
    tty.flush();
    tty.goto(5, 2);
    thread::sleep(Duration::from_millis(2000));
    // tty.set_fg("yellow");
    tty.printf("hello, world");

    thread::sleep(Duration::from_millis(2000));

    tty.switch();
    tty.flush();
    tty.goto(8, 3);
    // tty.set_fg("cyan");
    tty.printf("goodbye, world");

    thread::sleep(Duration::from_millis(2000));

    tty.switch_to(1);
    tty.flush();

    thread::sleep(Duration::from_millis(2000 + delay));

    tty.to_main();






    // tty.automatic();

    // NOTE: Methods to be tested:
    // - size
    // - resize
    // - clear(all)
    // - clear(currentln)
    // - clear(newln)
    // - clear(cursordn)
    // - clear(cursorup)
    // - enable_alt (switch and switch_to)
    // - disable_alt (to_main)

    // // NOTE:  Test #1: size()
    // let (w, h) = tty.screen_size();

    // tty.resize(80, 30);
    // // thread::sleep(Duration::from_millis(200));
    // let redims = tty.screen_size();
    // assert_eq!(redims, (80, 30));

    // tty.goto(0, 0);
    // thread::sleep(Duration::from_millis(delay));


    // // NOTE: Test #2 enable_alt (switch)
    // tty.switch();
    // let mut text = "Did the screen switch to an alternate screen";
    // let mut err = "The screen did not switch to an alternate screen.";
    // _prompt(&mut tty, text, err);

    // let fatline = "=".repeat(80);
    // tty.goto(0, 15);
    // tty.prints(&fatline);
    // tty.goto(0, 17);
    // tty.prints(&fatline);
    // tty.goto(0, 16);
    // tty.prints(&fatline);
    // tty.goto((text.len() - 1) as i16, 16);
    // tty.flush();

    // thread::sleep(Duration::from_millis(delay));

    // // NOTE:  Test #3: clear(currentln)
    // tty.clear("currentln");
    // text = "Did the screen clear the current line";
    // err = "The screen did not clear the current line.";
    // _prompt(&mut tty, text, err);

    // tty.goto(0, 0);
    // thread::sleep(Duration::from_millis(delay));

    // tty.clear("all");

    // let words = "A good choice of font for your coding can make a huge \
    //              difference and improve your productivity, so take a look at \
    //              the fonts in this post that can make your text editor or \
    //              terminal emulator look little bit nicer. Andale® Mono — is a \
    //              monospaced sans-serif typeface designed by Steve Matteson for \
    //              terminal emulation and software development environments, \
    //              originally for the Taligent project by Apple Inc. and IBM. \
    //              The Andalé design was originally created by Monotype, as part \
    //              of Andalé font families. Aperçu — Aperçu was started in \
    //              December 2009, and has been trialled and tested through a \
    //              number of design comissions taken on by The Entente through \
    //              2010. The conceit behind Aperçu was to create a synopsis or \
    //              amalgamation of classic realist typefaces: Johnston, Gill \
    //              Sans, Neuzeit & Franklin Gothic.";

    // tty.printf(words);

    // // NOTE: ensure that there are enough lines to visibly clear
    // let len = words.len() as i16;
    // let mut lines = len / 80;
    // if lines < 5 {
    //     tty.resize((words.len() * 5) as i16, 30);
    //     lines = len / (len * 5);
    // }

    // tty.goto(14, 3);
    // tty.flush();
    // thread::sleep(Duration::from_millis(delay));

    // // NOTE:  Test #4: clear(newln)
    // tty.clear("newln");
    // tty.goto(0, lines + 1);
    // text = "Did the screen clear until a new line";
    // err = "The screen did not clear until a new line.";
    // _prompt(&mut tty, text, err);

    // tty.goto(14, 3);
    // tty.flush();
    // thread::sleep(Duration::from_millis(delay));

    // // NOTE:  Test #5: clear(cursordn)
    // tty.clear("cursordn");
    // tty.goto(0, lines + 1);
    // text = "Did the screen clear from the cursor down";
    // err = "The screen did not clear from the cursor down.";
    // _prompt(&mut tty, text, err);

    // tty.goto(14, 2);
    // tty.flush();
    // thread::sleep(Duration::from_millis(delay));

    // // NOTE:  Test #6: clear(cursorup)
    // tty.clear("cursorup");
    // tty.goto(0, lines + 1);
    // text = "Did the screen clear from the cursor up";
    // err = "The screen did not clear from the cursor up.";
    // _prompt(&mut tty, text, err);

    // tty.goto(18, 3);
    // tty.flush();
    // thread::sleep(Duration::from_millis(delay));

    // // (imdaveho) TODO: "" needs to skip not reset:
    // tty.set_style("red", "", "underline");
    // tty.flush();
    // thread::sleep(Duration::from_millis(delay));
    // tty.printf("alternate screen <index: 1>");
    // tty.reset();

    // thread::sleep(Duration::from_millis(delay));

    // // NOTE:  Test #7: disable_alt (to_main)
    // tty.to_main();

    // tty.goto(0, 3);
    // tty.printf(words);
    // tty.goto(0, 0);

    // text = "Did the screen switch back to the main screen";
    // err = "The screen did not switch back to the main screen.";
    // _prompt(&mut tty, text, err);

    // // NOTE: Test #8 clear(all)
    // tty.clear("all");
    // text = "Did the screen clear everything";
    // err = "The screen did not clear the entire screen.";
    // _prompt(&mut tty, text, err);

    // thread::sleep(Duration::from_millis(delay));

    // tty.goto(0, 0);
    // tty.flush();
    // tty.printf(words);

    // thread::sleep(Duration::from_millis(delay));

    // // NOTE: Test #9 enable_alt (switch_to)
    // tty.switch();
    // tty.flush();

    // text = "Did the screen switch to a new (blank) alternate screen";
    // err = "The screen failed to switch to a new alternate screen.";
    // _prompt(&mut tty, text, err);

    // tty.switch_to(1);
    // tty.flush();

    // tty.goto(0, 0);
    // thread::sleep(Duration::from_millis(delay));

    // // (imdaveho) TODO: this will not be possible until `content` is stored in
    // // the `Metadata` struct.
    // text = "Did the screen switch to the previous alternate screen <index: 1>";
    // err = "The screen failed to switch to the previous alternate screen.";
    // _prompt(&mut tty, text, err);

    // tty.switch_to(0);
    // tty.flush();

    // // NOTE: Test #10 resize()
    // tty.resize(w, h);
    // thread::sleep(Duration::from_millis(1000));
    // assert_eq!(tty.screen_size(), (w, h));

    // tty.goto(0, h - 3);
    // tty.clear("currentln");
    // tty.goto(0, h - 2);
    // tty.clear("currentln");
    // tty.flush();

    // // tty should terminate at this point when it is dropped
    // // tty.terminate();
}
