extern crate csv;

use std::io::stdin;
use std::process;

fn main() {
    let mut rdr = csv::Reader::from_reader(stdin());
    for result in rdr.records() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => {
                println!("error reading CSV from <stdin>: {}", err);
                process::exit(1);
            }
        }
    }
}
