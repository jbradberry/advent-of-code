use std::io;
use std::io::prelude::*;


fn read() -> Vec<Vec<char>> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|x| x.unwrap().chars().collect())
        .collect()
}


fn main() {
    let map = read();

    println!("{:?}", map);
}
