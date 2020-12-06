use std::io;
use std::io::prelude::*;

use std::collections::HashSet;


fn read() -> Vec<HashSet<char>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.split("\n\n")
        .map(|s| {
             s.lines()
                .map(|p| p.chars().collect::<HashSet<char>>())
                .fold(None, |acc, p| match acc {
                    None => Some(p),
                    Some(x) => Some(x.intersection(&p).copied().collect()),
                })
                .unwrap_or(HashSet::new())
        })
        .collect()
}


fn main() {
    let answers = read();

    let sum: usize = answers.iter().map(|a| a.len()).sum();

    println!("sum: {}", sum);
}
