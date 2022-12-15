use std::io;
use std::io::prelude::*;

use std::collections::HashMap;


#[derive(Debug, Copy, Clone)]
enum Tile {
    Void,
    Rock,
    Sand,
    Source,
}


impl From<Tile> for char {
    fn from(t: Tile) -> Self {
        match t {
            Tile::Void => '.',
            Tile::Rock => '#',
            Tile::Sand => 'o',
            Tile::Source => '+',
        }
    }
}


fn read() -> Vec<Vec<(usize, usize)>> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .map(|l| {
            let line = l.unwrap();
            line.split(" -> ")
                .map(|c| {
                    let mut coord = c.split(',');
                    (coord.next().unwrap().parse().unwrap(), coord.next().unwrap().parse().unwrap())
                })
                .collect()
        })
        .collect()
}


fn calculate_grid(paths: &Vec<Vec<(usize, usize)>>) -> HashMap<(usize, usize), Tile> {
    let mut grid = HashMap::new();
    grid.insert((500, 0), Tile::Source);

    for path in paths {
        for w in path.windows(2) {
            if w[0].0 == w[1].0 {
                let (a, b) = if w[0].1 < w[1].1 { (w[0].1, w[1].1) } else { (w[1].1, w[0].1) };
                for r in a..=b {
                    grid.insert((w[0].0, r), Tile::Rock);
                }
            } else {
                let (a, b) = if w[0].0 < w[1].0 { (w[0].0, w[1].0) } else { (w[1].0, w[0].0) };
                for c in a..=b {
                    grid.insert((c, w[0].1), Tile::Rock);
                }
            }
        }
        println!("");
    }

    grid
}


fn display(grid: &HashMap<(usize, usize), Tile>) {
    let min_col = grid.keys().map(|&(c, _)| c).min().unwrap();
    let max_col = grid.keys().map(|&(c, _)| c).max().unwrap();
    let min_row = grid.keys().map(|&(_, r)| r).min().unwrap();
    let max_row = grid.keys().map(|&(_, r)| r).max().unwrap();

    let mut s = vec![
        (min_col / 100).to_string(),
        (min_col / 10 % 10).to_string(),
        (min_col % 10).to_string()
    ];
    for col in min_col+1..max_col {
        match col {
            500 => {
                s[0].push('5'); s[1].push('0'); s[2].push('0');
            },
            _ => {
                s[0].push(' '); s[1].push(' '); s[2].push(' ');
            },
        };
    }
    s[0].push_str(&(max_col / 100).to_string());
    s[1].push_str(&(max_col / 10 % 10).to_string());
    s[2].push_str(&(max_col % 10).to_string());

    println!("     {}", s[0]);
    println!("     {}", s[1]);
    println!("     {}", s[2]);

    for row in min_row..=max_row {
        let line = (min_col..=max_col)
            .map(|col| {
                match grid.get(&(col, row)) {
                    Some(&x) => x.into(),
                    None => '.',
                }
            })
            .collect::<String>();
        println!("{:>3}  {}", row, line);
    }
}


fn simulate(grid: &mut HashMap<(usize, usize), Tile>) -> bool {
    let min_col = grid.keys().map(|&(c, _)| c).min().unwrap();
    let max_col = grid.keys().map(|&(c, _)| c).max().unwrap();
    let min_row = grid.keys().map(|&(_, r)| r).min().unwrap();
    let max_row = grid.keys().map(|&(_, r)| r).max().unwrap();

    let mut c = 500;
    let mut r = 0;

    loop {
        if r > max_row || c < min_col || c > max_col { return false; }
        match grid.get(&(c, r + 1)) {
            Some(Tile::Void) | None => { r += 1; continue; },
            Some(_) => {},
        };
        match grid.get(&(c - 1, r + 1)) {
            Some(Tile::Void) | None => { r += 1; c -= 1; continue; },
            Some(_) => {},
        };
        match grid.get(&(c + 1, r + 1)) {
            Some(Tile::Void) | None => { r += 1; c += 1; continue; },
            Some(_) => { grid.insert((c, r), Tile::Sand); break; },
        };
    }

    display(grid);
    true
}


fn main() {
    let paths = read();
    let mut grid = calculate_grid(&paths);

    // display(&grid);

    while simulate(&mut grid) {}
}
