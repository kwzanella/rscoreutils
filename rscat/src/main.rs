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

fn print_file(f_path: File) -> std::io::Result<()> {
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
        print!("{line}");
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    dbg!(&args);

    // TODO Check if args.file is empty
    // If it is, read from stdin

    // let Some(first_file) = args.file.get(0) else {
    //     eprintln!("No file provided!");
    //     std::process::exit(1);
    // };

    // Iterate over all files
    // Concatenation will happen because files will be printed out in order
    for file_name in &args.file {
        // TODO Treat for -

        // FIXME: This does not end program. Should it?
        let f = File::open(file_name)?;
        print_file(f)?;
    }
    Ok(())
}
