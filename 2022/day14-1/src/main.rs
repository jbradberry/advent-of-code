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


impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Rock,
            'o' => Self::Sand,
            '+' => Self::Source,
            _ => Self::Void,
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
    let min_row = grid.keys().map(|&(r, _)| r).min().unwrap();
    let max_row = grid.keys().map(|&(r, _)| r).max().unwrap();
    let min_col = grid.keys().map(|&(_, c)| c).min().unwrap();
    let max_col = grid.keys().map(|&(_, c)| c).max().unwrap();

    for row in min_row..=max_row {
        let line = (min_col..=max_col)
            .map(|col| {
                match grid.get(&(row, col)) {
                    Some(&x) => x,
                    None => Tile::Void,
                }
            })
            .map(|t| t.into())
            .collect::<String>();
        println!("{}", line);
    }
}


fn main() {
    let paths = read();
    let grid = calculate_grid(&paths);

    display(&grid);
}
