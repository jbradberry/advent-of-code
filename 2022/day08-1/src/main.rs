use std::io;
use std::io::prelude::*;


fn read() -> Vec<Vec<u8>> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|x| {
            let line = x.unwrap();
            line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
        })
        .collect()
}


fn main() {
    let trees = read();

    println!("trees: {:?}", trees);
}
