use std::io;
use std::io::prelude::*;


fn read() -> Vec<Vec<String>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.split("\n\n")
        .map(|g| g.lines().map(String::from).collect())
        .collect()
}


fn main() {
    let answers = read();

    let sum: usize = answers.iter()
        .map(|g| {
            let first = &g[0];
            first.chars()
                .filter(|&c| g.iter().all(|s| s.contains(c)))
                .count()
        })
        .sum();

    println!("sum: {}", sum);
}
