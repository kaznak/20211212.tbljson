use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use std::process;

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    const NL: &str = "\n";

    for line in reader.lines() {
        let line_str = line.unwrap_or_else(|err| {
            eprintln!("Error while reading a line: {}", err);
            process::exit(1);
        });
        let _size = writer.write(line_str.as_bytes()).unwrap_or_else(|err| {
            eprintln!("Error while writing a line: {}", err);
            process::exit(1);
        });
        let _size_nl = writer.write(NL.as_bytes()).unwrap_or_else(|err| {
            eprintln!("Error while writing a line: {}", err);
            process::exit(1);
        });
    }
}
