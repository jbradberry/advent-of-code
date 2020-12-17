use std::io;
use std::io::prelude::*;

use std::collections::HashMap;


fn read() -> HashMap<(i64, i64, i64), bool> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .enumerate()
        .flat_map(|(i, s)| {
            let line = s.unwrap();
            line.chars()
                .enumerate()
                .map(move |(j, c)| {
                    ((i as i64, j as i64, 0i64), c == '#')
                })
                .collect::<Vec<_>>()
        })
        .collect()
}


fn render_grid(grid: &HashMap<(i64, i64, i64), bool>) {
    let min_x = grid.keys().map(|(x, _, _)| *x).min().unwrap();
    let max_x = grid.keys().map(|(x, _, _)| *x).max().unwrap();
    let min_y = grid.keys().map(|(_, y, _)| *y).min().unwrap();
    let max_y = grid.keys().map(|(_, y, _)| *y).max().unwrap();
    let min_z = grid.keys().map(|(_, _, z)| *z).min().unwrap();
    let max_z = grid.keys().map(|(_, _, z)| *z).max().unwrap();

    for z in min_z..=max_z {
        println!("z = {}", z);
        for x in min_x..=max_x {
            let row: String = (min_y..=max_y)
                .map(|y| if *grid.get(&(x, y, z)).unwrap_or(&false) { '#' } else { '.' })
                .collect();
            println!("{}", row);
        }
    }
}


fn main() {
    let mut grid = read();

    render_grid(&grid);
}
