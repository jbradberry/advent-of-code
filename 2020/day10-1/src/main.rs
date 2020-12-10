use std::io;
use std::io::prelude::*;

use itertools::Itertools;


fn read() -> Vec<u16> {
    let stdin = io::stdin();

    let mut data = vec![0];
    let input = stdin.lock().lines()
        .map(|s| s.unwrap().parse().unwrap())
        .sorted()
        .collect::<Vec<u16>>();
    let device = input.iter().max().unwrap() + 3;

    data.extend(input);
    data.push(device);

    data
}


fn main() {
    let joltages = read();

    let diffs = joltages.iter()
        .zip(joltages[1..].iter())
        .map(|(a, b)| b - a)
        .collect::<Vec<u16>>();

    let ones = diffs.iter().filter(|&x| *x == 1).count();
    let threes = diffs.iter().filter(|&x| *x == 3).count();

    println!("{} * {} = {}", ones, threes, ones * threes);
}
