use std::io;
use std::io::prelude::*;

use itertools::Itertools;


fn read() -> Vec<u32> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.trim().chars().map(|c| c.to_digit(10).unwrap()).collect()
}


fn main() {
    let data = read();

    let min_zeros = &data.into_iter().chunks(25 * 6)
        .into_iter()
        .map(|c| c.collect::<Vec<u32>>())
        .min_by_key(|c| c.iter().filter(|&&x| x == 0).count())
        .unwrap();
}
