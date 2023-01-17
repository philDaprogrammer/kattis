use std::io::{self, Stdin, BufRead};


fn main() {
    let stdin: Stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();

    for line in stdin.lock().lines().map(|r| r.unwrap()) {
        println!("{:?}", line);
        lines.push(line)
    }

    if lines[0].len() >= lines[1].len() {
        println!("go");
    } else {
        println!("no");
    }
}