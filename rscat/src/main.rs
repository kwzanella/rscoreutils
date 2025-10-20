// use std::{env, fs};

// // static FILE_PATH: &str = "teste.txt";

// // file_path needs to be passed as reference (borrowing). Otherwise, the ownership of the object will be passed here
// // fn cat() {
// //     dbg!(FILE_PATH);
// // }

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     dbg!(args);
//     let contents = fs::read_to_string(FILE_PATH)
//         .expect("Should have been able to read the file");
// }

/* TODO:
 * 1. Make cat work
 * 2. Make cat or cat - take input from stdin
 * 3. Use buffered read
 * 4. Implement line number argument
 * 5. Use benchmarking tools to improve on the code and test difference between buffered read and normal read (use perf, hotspot, criterion, etc)
 *    a. https://nnethercote.github.io/perf-book/io.html
 * 6. Try and experiment with multiple approachs to optmize the code
*/

use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("heavy-test.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    loop {
        line.clear();

        let num_bytes = reader.read_line(&mut line)?;

        if num_bytes == 0 {
            break;  // EOF has been found
        }

        print!("{line}");
    }

    Ok(())
}