use std::io;
use std::io::prelude::*;

use itertools::{Itertools, multizip};


fn read() -> Vec<u32> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.trim().chars().map(|c| c.to_digit(10).unwrap()).collect()
}


fn main() {
    let data = read();
    let layers = &data.into_iter().chunks(25 * 6)
        .into_iter()
        .map(|c| c.collect::<Vec<u32>>())
        .collect::<Vec<_>>();

    let pixels = multizip((&layers[0], &layers[1], &layers[2], &layers[3]))
        .map(|t| vec![t.0, t.1, t.2, t.3])
        .collect::<Vec<_>>();

    println!("{:#?}", pixels);
}
