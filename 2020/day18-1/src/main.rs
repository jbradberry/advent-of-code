use std::io;
use std::io::prelude::*;


fn read() -> Vec<Vec<char>> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|s| s.unwrap().chars().filter(|c| *c != ' ').collect())
        .collect()
}


fn main() {
    let problems = read();

    println!("{:?}", problems);
}
