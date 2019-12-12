use std::io;
use std::io::prelude::*;

use counter::Counter;
use itertools::{Itertools, zip};


fn part01() {
    let stdin = io::stdin();
    let mut twos = 0u32;
    let mut threes = 0u32;

    for line in stdin.lock().lines() {
        let char_counts = line.unwrap().chars().collect::<Counter<_>>();

        if char_counts.values().any(|&v| v == 2) { twos += 1; }
        if char_counts.values().any(|&v| v == 3) { threes += 1; }
    }

    println!("checksum = {}", twos * threes);
}


fn main() {
    let stdin = io::stdin();

    let ids = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    for (a, b) in ids.iter().tuple_combinations() {
        let common = zip(a.chars(), b.chars())
            .filter(|(m, n)| m == n)
            .map(|(m, _)| m.to_string())
            .collect::<String>();

        if common.len() == a.len() - 1 {
            println!("common = {}", common);
            return
        }
    }
}
