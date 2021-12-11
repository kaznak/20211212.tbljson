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

fn setup(mut args: env::Args) -> Config {
    let reader: Box<dyn io::Read> = match args.nth(1) {
        None => Box::new(io::stdin()),
        Some(file_path) => Box::new(File::open(file_path).unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        })),
    };
    Config {
        reader,
        has_headers: true,
    }
}

type Record = (String, String, Option<u64>, f64, f64);

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut reader_builder = csv::ReaderBuilder::new();
    let mut rdr = reader_builder
        .has_headers(config.has_headers)
        .from_reader(config.reader);
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let config = setup(env::args());
    if let Err(err) = run(config) {
        println!("{}", err);
        process::exit(1);
    }
}
