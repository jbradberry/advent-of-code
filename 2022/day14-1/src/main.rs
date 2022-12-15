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
    let mut grid: HashMap<(usize, usize), Tile> = paths.iter()
        .flat_map(|p| p.iter().map(|&c| (c, Tile::Rock)))
        .collect();

    grid.insert((500, 0), Tile::Source);

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


fn main() {
    let paths = read();
    let grid = calculate_grid(&paths);

    display(&grid);
}
