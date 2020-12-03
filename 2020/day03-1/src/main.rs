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

    let trees = (0..map.len())
        .filter(|r| map[*r][(3 * r) % map[0].len()] == '#')
        .count();

    println!("number of trees hit: {}", trees);
}
