// use tuitty;


// #[test]
// fn example() {
//     let mut tty = tuitty::tty::Tty::init();

//     tty.write(&format!{"w: {}, h: {}\n", tty.size().0, tty.size().1});

//     tty.set_fg("green");
//     tty.write("Hello (g), ");
//     tty.reset();
//     tty.write("Hello (d), ");
//     tty.flush();
//     tty.switch();
//     tty.clear("all");

//     tty.raw();
//     tty.enable_mouse();

//     let words = "A good choice of font for your coding can make a huge difference and improve your productivity, \
//     so take a look at the fonts in this post that can make your text editor or terminal emulator look little bit nicer. \
//     Andale® Mono — is a monospaced sans-serif typeface designed by Steve Matteson for terminal emulation and software \
//     development environments, originally for the Taligent project by Apple Inc. and IBM. The Andalé design was originally \
//     created by Monotype, as part of Andalé font families. Aperçu — Aperçu was started in December 2009, and has been \
//     trialled and tested through a number of design comissions taken on by The Entente through 2010. The conceit behind Aperçu \
//     was to create a synopsis or amalgamation of classic realist typefaces: Johnston, Gill Sans, Neuzeit & Franklin Gothic.";

//     tty.set_fg("red"); // this sets fg on altern screen
//     tty.write(words);
//     tty.flush();

//     use std::time::Duration;
//     use std::thread;
//     thread::sleep(Duration::from_secs(3));

//     tty.to_main();

//     tty.write("Hello (r), "); // since fg red was on the altern screen, the main screen is still white
//     tty.set_fg("darkblue");
//     tty.write("Hello (db), ");
//     tty.reset();
//     tty.write("End\n");
//     tty.flush();
//     thread::sleep(Duration::from_secs(2));

//     tty.exit();
//     thread::sleep(Duration::from_secs(2));
// }
