use std::io;
use std::io::prelude::*;

use std::collections::HashMap;


fn read() -> Vec<Vec<char>> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .map(|l| {
            let line = l.unwrap();
            line.chars().collect()
        })
        .collect()
}


fn calculate_legal(grid: &Vec<Vec<char>>) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut legal = HashMap::new();

    for (r1, r2) in (0..grid.len() - 1).zip(1..grid.len()) {
        for c in 0..grid[0].len() {
            let h1 = match grid[r1][c] {
                'S' => 'a' as u8,
                'E' => 'z' as u8,
                x @ _ => x as u8,
            };
            let h2 = match grid[r2][c] {
                'S' => 'a' as u8,
                'E' => 'z' as u8,
                x @ _ => x as u8,
            };

            if h1 <= h2 + 1 {
                legal.entry((r2, c)).or_insert_with(Vec::new).push((r1, c));
            }
            if h2 <= h1 + 1 {
                legal.entry((r1, c)).or_insert_with(Vec::new).push((r2, c));
            }
        }
    }
    for (c1, c2) in (0..grid[0].len() - 1).zip(1..grid[0].len()) {
        for r in 0..grid.len() {
            let h1 = match grid[r][c1] {
                'S' => 'a' as u8,
                'E' => 'z' as u8,
                x @ _ => x as u8,
            };
            let h2 = match grid[r][c2] {
                'S' => 'a' as u8,
                'E' => 'z' as u8,
                x @ _ => x as u8,
            };

            if h1 <= h2 + 1 {
                legal.entry((r, c2)).or_insert_with(Vec::new).push((r, c1));
            }
            if h2 <= h1 + 1 {
                legal.entry((r, c1)).or_insert_with(Vec::new).push((r, c2));
            }
        }
    }

    legal
}


fn main() {
    let grid = read();

    println!("grid: {:?}", grid);

    let legal = calculate_legal(&grid);

    println!("legal: {:?}", legal);
}
