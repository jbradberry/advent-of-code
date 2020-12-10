use std::io;
use std::io::prelude::*;

use itertools::Itertools;


fn read() -> Vec<u16> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|s| s.unwrap().parse().unwrap())
        .sorted()
        .collect()
}


fn main() {
    let joltages = read();

    println!("{:?}", joltages);
}
