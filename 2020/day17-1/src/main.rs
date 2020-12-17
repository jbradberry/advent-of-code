use std::io;
use std::io::prelude::*;

use std::collections::HashMap;


fn read() -> HashMap<(i64, i64, i64), bool> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .enumerate()
        .flat_map(|(i, s)| {
            let line = s.unwrap();
            line.chars()
                .enumerate()
                .map(move |(j, c)| {
                    ((i as i64, j as i64, 0i64), c == '#')
                })
                .collect::<Vec<_>>()
        })
        .collect()
}


fn main() {
    let mut grid = read();

    println!("{:?}", grid);
}
