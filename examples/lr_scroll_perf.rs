use std::io::{stdout, BufWriter, Write};
use std::thread;
use std::time::Duration;



#[cfg(unix)]
use libc::{
    cfmakeraw, tcgetattr, tcsetattr,
    STDIN_FILENO, TCSANOW, termios as Termios
};

#[cfg(unix)]
use std::{ mem, io::{ Error, Result } };


fn prints(content: &str) {
    let output = stdout();
    let lock = output.lock();
    let mut outbuf = BufWriter::new(lock);
    outbuf.write_all(content.as_bytes()).expect("I/O error on write");
}

fn flush() {
    let output = stdout();
    let lock = output.lock();
    let mut outbuf = BufWriter::new(lock);
    outbuf.flush().expect("I/O error on flush");
}

fn printf(content: &str) {
    let output = stdout();
    let lock = output.lock();
    let mut outbuf = BufWriter::new(lock);
    outbuf.write_all(content.as_bytes()).expect("I/O error on write");
    outbuf.flush().expect("I/O error on flush");
}

fn goto(col: i16, row: i16) -> String {
    format!("\x1B[{};{}H", row + 1, col + 1)
}



fn enable_alt() -> String {
    "\x1B[?1049h".to_string()
}

fn disable_alt() -> String {
    "\x1B[?1049l".to_string()
}

#[cfg(unix)]
fn get_mode() -> Result<Termios> {
    unsafe {
        let mut termios = mem::zeroed();
        if tcgetattr(STDIN_FILENO, &mut termios) == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(termios)
        }
    }
}

/// This function enables raw mode in the current screen.
#[cfg(unix)]
fn enable_raw() -> Result<()> {
    unsafe {
        // Get the current terminal attrs.
        let mut termios = get_mode()?;
        // Apply the raw attr to the current terminal attrs.
        // There is no effect until a subsequent call to tcsetattr().
        // https://www.mkssoftware.com/docs/man3/cfmakeraw.3.asp
        cfmakeraw(&mut termios);
        // Set the current terminal with raw-enabled attrs.
        // unwrap(tcsetattr(0, 0, &termios)).and(Ok(()))
        set_mode(&termios)?;
        Ok(())
    }
}

#[cfg(unix)]
fn set_mode(termios: &Termios) -> Result<()> {
    if unsafe { tcsetattr(STDIN_FILENO, TCSANOW, termios) } == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

fn main() {
    // Null printing test (to see if it skips chars)
    // printf(&"\n".repeat(30));
    // prints(&goto(0, 4));
    // printf("Hello, world");
    // prints(&goto(0, 4));

    // thread::sleep(Duration::from_millis(2000));
    // printf("\0\0\0ttt\0\0\0a\0f");
    // thread::sleep(Duration::from_millis(2000));


    let mode = get_mode().unwrap();

    printf(&enable_alt());
    let _ = enable_raw();

    // DCH test with emoji and cjk
    printf(&goto(85, 29));
    printf("X");
    // printf("\x1B[4h");
    printf(&goto(73, 0));
    thread::sleep(Duration::from_millis(400));
    // printf("👨‍👩‍👦");
    // printf("o園o明");
    printf("Hello, world!😀你好 yay!");
    thread::sleep(Duration::from_millis(1000));
    printf(&goto(80, 0));
    thread::sleep(Duration::from_millis(1000));
    printf("\x1b[4h");
    printf("a\ta");
    // printf("whale!Go US");
    // printf("\x1B[2P");
    printf("\x1B[4l");
    thread::sleep(Duration::from_millis(2000));


    // VT100 Left/Right Scrolling
    // {
    //     let s = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGH";
    //     prints(&goto(0, 0));
    //     for i in 0..30 {
    //         prints(s);
    //     }
    //     flush();
    //     thread::sleep(Duration::from_millis(2000));

    //     // Right Scroll (DCH) 5
    //     // Part 0 - highlight delete region
    //     for i in 0..30 {
    //         prints(&goto(0, i));
    //         prints("\x1B[31mabcde\x1B[39m");
    //     }
    //     flush();
    //     thread::sleep(Duration::from_millis(2000));

    //     // Part 1 - delete
    //     prints(&goto(0, 0));
    //     for _ in 0..30 {
    //         printf("\x1B[5P\x1B[B"); // DCH
    //     }
    //     // Part 2 - fill right side
    //     for i in 0..30 {
    //         prints(&goto(81, i));
    //         prints("\x1b[32mIJKLM\x1b[39m");
    //     }
    //     flush();
    //     thread::sleep(Duration::from_millis(2000));


    //     // Left Scroll (ICH) 5
    //     // Part 0 - highlight delete region
    //     for i in 0..30 {
    //         prints(&goto(81, i));
    //         prints("\x1B[31mIJKLM\x1B[39m");
    //     }
    //     flush();
    //     thread::sleep(Duration::from_millis(2000));

    //     // Part 1 - insert
    //     // prints(&goto(0, 0));
    //     prints("\x1B[4h"); // insert mode
    //     for i in 0..30 {
    //         prints(&goto(0, i));
    //         prints("\x1B[32mabcde\x1B[39m");
    //         // printf("\x1B[5@\x1B[B"); // ICH (option B)
    //     }
    //     prints("\x1B[4l"); // overwrite mode
    //     // Part 2 - fill left side (option B)
    //     // for i in 0..30 {
    //     //     prints(&goto(81, i));
    //     //     prints("\x1b[32mIJKLM\x1b[39m");
    //     // }
    //     flush();
    //     thread::sleep(Duration::from_millis(2000));
    // }


    // Tab behavior example
    // {
    //   printf(&goto(0, 29));
    //   printf(&"-".repeat(86));
    //   printf(&goto(0, 0));
    //   thread::sleep(Duration::from_millis(1000));

    //   printf("123456789a\r\ndefghijklm");
    //   thread::sleep(Duration::from_millis(2000));

    //   printf(&goto(2, 0));
    //   thread::sleep(Duration::from_millis(2000));

    //   // printf("\x1B[4h"); // insert mode
    //   // printf("\n\t0");
    //   printf("\x1B[6X"); // n to next tapstop (ECH)
    //   printf("\t0");
    //   // printf("\x1B[4l"); // overwrite mode
    //   thread::sleep(Duration::from_millis(2000));

    //   prints(&goto(0, 2));
    //   prints("qwerqwerxy");
    //   prints(&goto(0, 3));
    //   prints("qwerqwerxy");
    //   thread::sleep(Duration::from_millis(2000));

    //   let rest = ["b", "n", "z", "z"];
    //   for i in 0..4 {
    //       prints(&goto(0, i));
    //       prints("\x1B[P"); // Delete Char (shifts left)
    //       prints(&goto(8, i));
    //       prints(rest[i as usize]);
    //   }
    //   flush();

    //   // printf("\x1B[5X"); // Erase Char

    //   // printf(&goto(3, 3));
    //   // thread::sleep(Duration::from_millis(2000));

    //   // printf("\t");
    //   // thread::sleep(Duration::from_millis(2000));
    //   // Does not work except on XTerm (vt420 emulation)
    //   // printf("\x1B[?69h");
    //   // printf("\x1B[2'~"); // DECIC
    //   // printf("\x1B[2'}"); // DECDC
    //   // printf("\x1B[2'~");
    //   // printf("\x1B[?69l");
    //   thread::sleep(Duration::from_millis(2000));
    // }

    let _ = set_mode(&mode);
    printf(&disable_alt());
}
