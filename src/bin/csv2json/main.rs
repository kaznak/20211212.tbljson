//tutorial-read-01.rs
extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

/// このプロセスに送られた最初の固定引数を返す
/// 固定引数がなかった場合エラーを返す。
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
