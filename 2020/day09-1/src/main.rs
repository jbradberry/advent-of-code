use std::io;
use std::io::prelude::*;

use itertools::Itertools;


fn read() -> Vec<u64> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect()
}


fn main() {
    let data = read();

    let (_, first_invalid) = data.iter()
        .enumerate()
        .filter(|(i, _)| *i >= 25)
        .filter(|(i, x)| {
            !data[(i - 25)..*i].iter()
                .combinations(2)
                .any(|v| v[0] + v[1] == **x)
        })
        .next().unwrap();

    println!("first invalid: {}", first_invalid);
}
