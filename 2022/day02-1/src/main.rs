use std::io;
use std::io::prelude::*;


fn read() -> Vec<Vec<char>> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(
            |x| x.unwrap()
                .split(" ")
                .map(|c| c.parse().unwrap()).collect()
        )
        .collect()
}


fn score(strategy: &Vec<Vec<char>>) -> u32 {
    strategy.iter()
        .map(|r| {
            let a = match r[1] {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => 0
            };

            let b = match (r[0], r[1]) {
                ('A', 'Y') => 6,
                ('B', 'Z') => 6,
                ('C', 'X') => 6,
                ('A', 'X') => 3,
                ('B', 'Y') => 3,
                ('C', 'Z') => 3,
                _ => 0
            };

            a + b
        })
        .sum::<u32>()
}


fn main() {
    let strategy = read();
    let score = score(&strategy);

    println!("score: {}", score);
}
