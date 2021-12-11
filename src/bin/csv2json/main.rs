extern crate csv;

use std::io::stdin;

fn main() {
    let mut rdr = csv::Reader::from_reader(stdin());
    for result in rdr.records() {
        let record = result.expect("a CSV record");
        println!("{:?}", record);
    }
}
