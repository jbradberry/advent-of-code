use std::io;
use std::io::prelude::*;


fn read() -> Vec<Vec<i8>> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|x| {
            let line = x.unwrap();
            line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect()
        })
        .collect()
}


fn visible(trees: &Vec<Vec<i8>>) -> Vec<Vec<bool>> {
    let mut results: Vec<Vec<bool>> = trees.iter().map(|row| row.iter().map(|_| false).collect()).collect();
    let mut current;

    for r in 0..trees.len() {
        current = -1;
        for c in 0..trees[0].len() {
            let x = trees[r][c] > current;
            results[r][c] = x || results[r][c];
            if x { current = trees[r][c]; }
        }
    }

    for r in 0..trees.len() {
        current = -1;
        for c in (0..trees[0].len()).rev() {
            let x = trees[r][c] > current;
            results[r][c] = x || results[r][c];
            if x { current = trees[r][c]; }
        }
    }

    for c in 0..trees[0].len() {
        current = -1;
        for r in 0..trees.len() {
            let x = trees[r][c] > current;
            results[r][c] = x || results[r][c];
            if x { current = trees[r][c]; }
        }
    }

    for c in 0..trees[0].len() {
        current = -1;
        for r in (0..trees.len()).rev() {
            let x = trees[r][c] > current;
            results[r][c] = x || results[r][c];
            if x { current = trees[r][c]; }
        }
    }

    results
}


fn main() {
    let trees = read();

    println!("trees: {:?}", trees);

    let vis = visible(&trees);
    let number: usize = vis.iter().map(|row| row.iter().filter(|x| **x).count()).sum();

    println!("visible trees: {}", number);
}
