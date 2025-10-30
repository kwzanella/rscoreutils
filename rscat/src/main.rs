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

use std::io::{BufRead, BufReader, stdin};
use std::fs::File;
use clap::{Parser, command};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, long_about = "Concatenate FILE(s) to standard output.")]
struct Args {
    file: Vec<String>,

    #[arg(short, long, help = "number all output lines")]
    number: bool,

    #[arg(short = 'b', long, help = "number nonempty output lines, overrides -n")]
    number_nonblank: bool
}

fn print_file<R: BufRead>(reader: &mut R, line_counter: &mut usize) -> std::io::Result<()> {
    let mut line = String::new();
    let mut num_bytes: usize;

    loop {
        // Clear buffer before reading next line
        line.clear();

        // This may fail if file is not UTF-8
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

fn check_files(file_paths: &[String]) -> bool {
    for p in file_paths {

        // No need to check if stdin exists
        if p == "-" {
            continue;
        }

        if !Path::new(p).exists() {
            eprintln!("File {p} does not exist");
            return false;
        }
    }
    return true;
}

fn main() -> std::io::Result<()> {
    let mut args = Args::parse();

    // TODO Check if args.file is empty
    // If it is, read from stdin

    // TODO What if file is not ascii?

    // TODO Change exit codes? Maybe copy from cat

    // TODO Treat other args

    // TODO Maybe unnecessary?
    if args.file.is_empty() {
        args.file.push("-".to_string());
    }
    else {  // Only check files if any have been input by user
        if !check_files(&args.file) {
            std::process::exit(1);
        }
    }

    let mut line_counter = 0;
    for file_name in &args.file {

        if file_name == "-" {  // read from stdin
            match print_file(&mut BufReader::new(stdin().lock()), &mut line_counter) {
                Ok(()) => (),
                Err(_) => {
                    println!("Error printing stdin");
                    std::process::exit(1);
                }
            }
        }
        else {  // normal file
            let file = match File::open(file_name) {
                Ok(f) => f,
                Err(_) => {
                    eprintln!("Error reading file");
                    std::process::exit(1);
                }
            };

            // TODO Fix this for raw data files
            // Currently it fails if we try reading a file that is not UTF-8
            match print_file(&mut BufReader::new(file), &mut line_counter) {
                Ok(()) => (),
                Err(_) => {
                    println!("Error printing file");
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
