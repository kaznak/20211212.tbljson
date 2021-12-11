extern crate csv;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;

struct Config {
    reader: Box<dyn io::Read>,
    has_headers: bool,
}

fn parse_args() -> Config {
    let reader_raw: Box<dyn io::Read> = match env::args().nth(1) {
        None => Box::new(io::stdin()),
        Some(file_path) => Box::new(File::open(file_path).unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        })),
    };
    Config {
        reader: reader_raw,
        has_headers: false,
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut reader_builder = csv::ReaderBuilder::new();
    let mut rdr = reader_builder
        .has_headers(config.has_headers)
        .from_reader(config.reader);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let config = parse_args();
    if let Err(err) = run(config) {
        println!("{}", err);
        process::exit(1);
    }
}
