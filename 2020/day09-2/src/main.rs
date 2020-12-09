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
    let invalid: u64 = 25918798;
    let data = read();

    let seq = (0..=data.len()).combinations(2)
        .filter(|v| data[v[0]..v[1]].iter().sum::<u64>() == invalid)
        .map(|v| (v[1] - v[0] + 1, v[0], v[1]))
        .sorted()
        .collect::<Vec<_>>();
    let (_, i1, i2) = seq.iter().last().unwrap();

    let subseq = &data[*i1..*i2];
    let min = subseq.iter().min().unwrap();
    let max = subseq.iter().max().unwrap();

    println!("min: {}", min);
    println!("max: {}", max);
    println!("sum: {}", min + max);
}
