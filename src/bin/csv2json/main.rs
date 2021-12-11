//tutorial-read-01.rs
extern crate csv;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;


fn run<R: io::Read>(reader_raw: R) -> Result<(), Box<dyn Error>> {
    let mut reader_builder = csv::ReaderBuilder::new();
    let mut rdr = reader_builder.has_headers(false).from_reader(reader_raw);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let reader_raw: Box<dyn io::Read> = match env::args().nth(1) {
        None => Box::new(io::stdin()),
        Some(file_path) => Box::new(File::open(file_path).unwrap()),
    };
    if let Err(err) = run(reader_raw) {
        println!("{}", err);
        process::exit(1);
    }
}
