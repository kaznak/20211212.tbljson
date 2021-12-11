extern crate csv;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;

struct Config {
    reader: Box<dyn io::Read>,
    writer:  Box<dyn io::Write>,
    has_headers: bool,
}

fn setup(mut args: env::Args) -> Config {
    let reader: Box<dyn io::Read> = match args.nth(1) {
        None => Box::new(io::stdin()),
        Some(file_path) => Box::new(File::open(file_path).unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        })),
    };
    let writer = Box::new(io::stdout());
    Config {
        reader,
        writer,
        has_headers: true,
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut reader_builder = csv::ReaderBuilder::new();
    let mut rdr = reader_builder
        .has_headers(config.has_headers)
        .from_reader(config.reader);
    let mut wtr = csv::Writer::from_writer(config.writer);
    for result in rdr.records() {
        let record = result?;
        wtr.write_record(&record)?;
    }
    wtr.flush()?;
    Ok(())
}

fn main() {
    let config = setup(env::args());
    if let Err(err) = run(config) {
        println!("{}", err);
        process::exit(1);
    }
}
