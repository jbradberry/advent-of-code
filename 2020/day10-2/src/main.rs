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

    let mut combinations = (0..joltages.len()).map(|_| 1).collect::<Vec<u64>>();
    let mut j_iter = joltages.iter().enumerate().rev();
    j_iter.next();

    for (i, j) in j_iter {
        let mut count = 0;
        for a in 1..=3 {
            if i + a >= joltages.len() { continue; }
            if joltages[i + a] > j + 3 { continue; }
            count += combinations[i + a];
        }
        combinations[i] = count;
    }

    println!("combinations: {}", combinations[0]);
}
