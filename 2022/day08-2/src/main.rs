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


fn visible(trees: &Vec<Vec<i8>>) -> Vec<Vec<isize>> {
    let mut results: Vec<Vec<isize>> = trees.iter().map(|row| row.iter().map(|_| 1).collect()).collect();
    let mut index;

    for r in 0..trees.len() {
        for c1 in 0..trees[0].len() {
            index = 0;
            for c2 in (c1 + 1)..trees[0].len() {
                index += 1;
                if trees[r][c1] <= trees[r][c2] { break; }
            }
            results[r][c1] *= index;
        }
    }

    for r in 0..trees.len() {
        for c1 in (0..trees[0].len()).rev() {
            index = 0;
            for c2 in (0..c1).rev() {
                index += 1;
                if trees[r][c1] <= trees[r][c2] { break; }
            }
            results[r][c1] *= index;
        }
    }

    for c in 0..trees[0].len() {
        for r1 in 0..trees.len() {
            index = 0;
            for r2 in (r1 + 1)..trees.len() {
                index += 1;
                if trees[r1][c] <= trees[r2][c] { break; }
            }
            results[r1][c] *= index;
        }
    }

    for c in 0..trees[0].len() {
        for r1 in (0..trees.len()).rev() {
            index = 0;
            for r2 in (0..r1).rev() {
                index += 1;
                if trees[r1][c] <= trees[r2][c] { break; }
            }
            results[r1][c] *= index;
        }
    }

    results
}


fn main() {
    let trees = read();

    println!("trees: {:?}", trees);

    let vis = visible(&trees);

    let number: isize = *vis.iter().map(|row| row.iter().max().unwrap()).max().unwrap();

    println!("highest scenic score: {}", number);
}
