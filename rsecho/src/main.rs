/* Extremely simple echo command implemented in Rust. */

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Do not care for executable path
    let concatenated = args[1..].join(" ");

    // Insteresting debug function
    // dbg!(&args);

    println!("{}", concatenated);
}
