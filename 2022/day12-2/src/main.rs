use std::io;
use std::io::prelude::*;

use std::collections::{HashMap, HashSet};


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
                legal.entry((r1, c)).or_insert_with(Vec::new).push((r2, c));
            }
            if h2 <= h1 + 1 {
                legal.entry((r2, c)).or_insert_with(Vec::new).push((r1, c));
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
                legal.entry((r, c1)).or_insert_with(Vec::new).push((r, c2));
            }
            if h2 <= h1 + 1 {
                legal.entry((r, c2)).or_insert_with(Vec::new).push((r, c1));
            }
        }
    }

    legal
}


fn calculate_steps(grid: &Vec<Vec<char>>) -> usize {
    let legal = calculate_legal(&grid);

    // println!("legal: {:?}", legal);

    let start: HashSet<(usize, usize)> = grid.iter().enumerate()
        .filter_map(|(i, row)| {
            match row.iter().position(|&x| x == 'S' || x == 'a') {
                Some(j) => Some((i, j)),
                None => None
            }
        })
        .collect();
    let end = grid.iter().enumerate()
        .filter_map(|(i, row)| {
            match row.iter().position(|&x| x == 'E') {
                Some(j) => Some((i, j)),
                None => None
            }
        })
        .next().unwrap();

    // println!("start: {:?}", start);
    // println!("end: {:?}", end);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut last = HashSet::from([end]);
    for steps in 1..(grid.len() * grid[0].len()) {
        visited = visited.union(&last).cloned().collect();
        // println!("{:?}", visited);

        last = last.iter().flat_map(|coord| legal.get(coord).unwrap().iter().cloned()).collect();

        if last.intersection(&start).next().is_some() { return steps; }
    }

    0
}


fn main() {
    let grid = read();

    // println!("grid: {:?}", grid);

    let steps = calculate_steps(&grid);

    println!("min number of steps: {}", steps);
}
