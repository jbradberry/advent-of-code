use std::io;
use std::io::prelude::*;


fn read() -> Vec<Vec<char>> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .map(|l| {
            let line = l.unwrap();
            line.chars().collect()
        })
        .collect()
}


fn main() {
    let grid = read();

    println!("grid: {:?}", grid);
}
