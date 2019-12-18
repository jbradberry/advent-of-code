use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use itertools::Itertools;


fn read() -> HashMap<(usize, usize), char> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.unwrap().char_indices()
                .filter(|(_, c)| *c != '#')
                .map(move |(x, c)| ((x, y), c)).collect::<Vec<_>>()
        })
        .collect()
}


fn main() {
    let maze = read();

    println!("{:?}", maze);
    println!("length: {}", maze.len());
}
