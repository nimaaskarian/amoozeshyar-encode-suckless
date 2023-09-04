use std::io::{BufRead};
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line) = lines.next(){
        print_converted(line.unwrap());
    }
}

fn print_converted(mut line: String) {
    const REPLACE_WITH_PERCENT_SIGN: &str = "کی";

    for ch in REPLACE_WITH_PERCENT_SIGN.chars() {
        line = line.replace(ch, "%")
    }
    println!("%{}%", line);
}
