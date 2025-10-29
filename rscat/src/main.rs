// https://nnethercote.github.io/perf-book/io.html

/*
 * 1. Concatenate multiples files
 * 2. Print to STDOUT
 * 3. Get input from STDIN with - or without any arg
 * 4. --number
 * 5. --number-nonblank
 * 6. --help
 * 7. --version
 * 8. Integration tests
*/

use std::io::{BufRead, BufReader};
use std::fs::File;
use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(version, long_about = "Concatenate FILE(s) to standard output.")]
struct Args {
    // number all output lines
    #[arg(short, long, help = "teste")]
    number: bool,

    // number nonempty output lines, overrides -n
    #[arg(short = 'b', long, help = "teste2")]
    number_nonblank: bool
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    dbg!(&args);
    return Ok(());


    let f = File::open("poem.txt")?;

    // Decreases system calls by buffering file
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