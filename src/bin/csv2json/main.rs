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
    let mut reader_builder = csv::ReaderBuilder::new();
    reader_builder.has_headers(false);
    match env::args().nth(1) {
        None => {
            let rdr = reader_builder.from_reader(io::stdin());
            return do_work(rdr);
        }
        Some(file_path) => {
            let rdr = reader_builder.from_reader(File::open(file_path)?);
            return do_work(rdr);
        }
    };
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
