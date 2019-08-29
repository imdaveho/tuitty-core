// ANSI Macros
// ****************************************************************************

// Append a the first few characters of an ANSI escape code to the given string.
#[macro_export]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

// // Write a string to standard output.
// #[macro_export]
// macro_rules! writebuf {
//     ($string:expr) => {{
//         use std::io::{BufWriter, Write};
        
//         let stdout = ::std::io::stdout();
//         let lock = stdout.lock();
//         let mut writer = BufWriter::new(lock);
//         // let mut size = 0;
//         // let result = writer.write($string.as_bytes());
//         // size += match result {
//         //     Ok(size) => size,
//         //     Err(e) => return Err(e),
//         // };
//         // Ok(size)
//         writer.write($string.as_bytes())
//     }};
// }
