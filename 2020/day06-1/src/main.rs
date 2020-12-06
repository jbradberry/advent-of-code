use std::io;
use std::io::prelude::*;

use std::collections::HashSet;


fn read() -> Vec<HashSet<char>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.split("\n\n")
        .map(|s| s.chars().filter(|&c| c != '\n').collect())
        .collect()
}


fn main() {
    let answers = read();

    println!("{:?}", answers);
}
