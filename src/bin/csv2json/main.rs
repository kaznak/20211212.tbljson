//tutorial-read-01.rs
extern crate csv;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;


fn run() -> Result<(), Box<dyn Error>> {
    let mut reader_builder = csv::ReaderBuilder::new();
    reader_builder.has_headers(false);
    let path_result = env::args().nth(1);
    if None == path_result {
        let mut rdr = reader_builder.from_reader(io::stdin());
        for result in rdr.records() {
            let record = result?;
            println!("{:?}", record);
        }
    } else {
        let file_path = path_result.unwrap();
        let mut rdr = reader_builder.from_reader(File::open(file_path)?);
        for result in rdr.records() {
            let record = result?;
            println!("{:?}", record);
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
