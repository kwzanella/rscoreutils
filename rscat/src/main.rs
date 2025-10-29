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
    file: Vec<String>,

    #[arg(short, long, help = "number all output lines")]
    number: bool,

    #[arg(short = 'b', long, help = "number nonempty output lines, overrides -n")]
    number_nonblank: bool
}

fn print_file(f_path: File, line_counter: &mut usize) -> std::io::Result<()> {
    // Decreases system calls by buffering file
    let mut reader = BufReader::new(f_path);
    let mut line = String::new();
    let mut num_bytes: usize;
    loop {
        // Clear buffer before reading next line
        line.clear();

        num_bytes = reader.read_line(&mut line)?;

        // EOF has been found
        if num_bytes == 0 {
            return Ok(());
        }

        // TODO Lock stdout before loop
        print!("\t{}  {}", *line_counter, line);
        *line_counter += 1;
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // TODO Check if args.file is empty
    // If it is, read from stdin

    // TODO Check if all files exist before starting

    // TODO What if file is not ascii?

    // Iterate over all files
    // Concatenation will happen because files will be printed out in order

    let mut line_counter = 0;
    for file_name in &args.file {
        // TODO Treat for -

        let file = match File::open(file_name) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Failed to open file");
                std::process::exit(1);
            }
        };

        match print_file(file, &mut line_counter) {
            Ok(()) => (),
            Err(_) => {
                println!("Error printing file");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
