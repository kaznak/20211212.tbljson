//tutorial-read-01.rs
extern crate csv;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;

fn do_work<R: io::Read>(mut rdr: csv::Reader<R>) -> Result<(), Box<dyn Error>> {
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let reader_raw: Box<dyn io::Read> = match env::args().nth(1) {
        None => Box::new(io::stdin()),
        Some(file_path) => Box::new(File::open(file_path)?),
    };
    let mut reader_builder = csv::ReaderBuilder::new();
    let rdr = reader_builder.has_headers(false).from_reader(reader_raw);
    do_work(rdr)
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
