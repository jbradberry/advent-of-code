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
            let c = match r[1] {
                'X' => match r[0] {  // loss
                    'A' => 'C',
                    'B' => 'A',
                    'C' => 'B',
                    _ => unreachable!()
                },
                'Y' => r[0],  // draw
                'Z' => match r[0] {  // win
                    'A' => 'B',
                    'B' => 'C',
                    'C' => 'A',
                    _ => unreachable!()
                },
                _ => ' '
            };

            let a = match c {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                _ => unreachable!()
            };

            let b = match (r[0], c) {
                ('A', 'B') => 6,
                ('B', 'C') => 6,
                ('C', 'A') => 6,
                ('A', 'A') => 3,
                ('B', 'B') => 3,
                ('C', 'C') => 3,
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
