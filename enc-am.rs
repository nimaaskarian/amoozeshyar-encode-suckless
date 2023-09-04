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
    line = line.replace("ک","%");
    line = line.replace("ی","%");
    println!("%{}%", line);
}
