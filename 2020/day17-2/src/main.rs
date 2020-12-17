use std::io;
use std::io::prelude::*;

use std::collections::HashMap;


static NEIGHBORS: [(i64, i64, i64, i64); 80] = [(-1, -1, -1, -1), (-1, -1, -1, 0), (-1, -1, -1, 1),
                                                (-1, -1,  0, -1), (-1, -1,  0, 0), (-1, -1,  0, 1),
                                                (-1, -1,  1, -1), (-1, -1,  1, 0), (-1, -1,  1, 1),
                                                (-1,  0, -1, -1), (-1,  0, -1, 0), (-1,  0, -1, 1),
                                                (-1,  0,  0, -1), (-1,  0,  0, 0), (-1,  0,  0, 1),
                                                (-1,  0,  1, -1), (-1,  0,  1, 0), (-1,  0,  1, 1),
                                                (-1,  1, -1, -1), (-1,  1, -1, 0), (-1,  1, -1, 1),
                                                (-1,  1,  0, -1), (-1,  1,  0, 0), (-1,  1,  0, 1),
                                                (-1,  1,  1, -1), (-1,  1,  1, 0), (-1,  1,  1, 1),

                                                ( 0, -1, -1, -1), ( 0, -1, -1, 0), ( 0, -1, -1, 1),
                                                ( 0, -1,  0, -1), ( 0, -1,  0, 0), ( 0, -1,  0, 1),
                                                ( 0, -1,  1, -1), ( 0, -1,  1, 0), ( 0, -1,  1, 1),
                                                ( 0,  0, -1, -1), ( 0,  0, -1, 0), ( 0,  0, -1, 1),
                                                ( 0,  0,  0, -1),                  ( 0,  0,  0, 1),
                                                ( 0,  0,  1, -1), ( 0,  0,  1, 0), ( 0,  0,  1, 1),
                                                ( 0,  1, -1, -1), ( 0,  1, -1, 0), ( 0,  1, -1, 1),
                                                ( 0,  1,  0, -1), ( 0,  1,  0, 0), ( 0,  1,  0, 1),
                                                ( 0,  1,  1, -1), ( 0,  1,  1, 0), ( 0,  1,  1, 1),

                                                ( 1, -1, -1, -1), ( 1, -1, -1, 0), ( 1, -1, -1, 1),
                                                ( 1, -1,  0, -1), ( 1, -1,  0, 0), ( 1, -1,  0, 1),
                                                ( 1, -1,  1, -1), ( 1, -1,  1, 0), ( 1, -1,  1, 1),
                                                ( 1,  0, -1, -1), ( 1,  0, -1, 0), ( 1,  0, -1, 1),
                                                ( 1,  0,  0, -1), ( 1,  0,  0, 0), ( 1,  0,  0, 1),
                                                ( 1,  0,  1, -1), ( 1,  0,  1, 0), ( 1,  0,  1, 1),
                                                ( 1,  1, -1, -1), ( 1,  1, -1, 0), ( 1,  1, -1, 1),
                                                ( 1,  1,  0, -1), ( 1,  1,  0, 0), ( 1,  1,  0, 1),
                                                ( 1,  1,  1, -1), ( 1,  1,  1, 0), ( 1,  1,  1, 1)];


fn read() -> HashMap<(i64, i64, i64, i64), bool> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .enumerate()
        .flat_map(|(x, s)| {
            let line = s.unwrap();
            line.chars()
                .enumerate()
                .map(move |(y, c)| {
                    ((x as i64, y as i64, 0i64, 0i64), c == '#')
                })
                .collect::<Vec<_>>()
        })
        .collect()
}


fn render_grid(grid: &HashMap<(i64, i64, i64, i64), bool>) {
    let min_x = grid.keys().map(|(x, _, _, _)| *x).min().unwrap();
    let max_x = grid.keys().map(|(x, _, _, _)| *x).max().unwrap();
    let min_y = grid.keys().map(|(_, y, _, _)| *y).min().unwrap();
    let max_y = grid.keys().map(|(_, y, _, _)| *y).max().unwrap();
    let min_z = grid.keys().map(|(_, _, z, _)| *z).min().unwrap();
    let max_z = grid.keys().map(|(_, _, z, _)| *z).max().unwrap();
    let min_w = grid.keys().map(|(_, _, _, w)| *w).min().unwrap();
    let max_w = grid.keys().map(|(_, _, _, w)| *w).max().unwrap();

    for z in min_z..=max_z {
        for w in min_w..=max_w {
            println!("z = {}, w = {}", z, w);
            for x in min_x..=max_x {
                let row: String = (min_y..=max_y)
                    .map(|y| if *grid.get(&(x, y, z, w)).unwrap_or(&false) { '#' } else { '.' })
                    .collect();
                println!("{}", row);
            }
        }
    }
}


fn step_grid(grid: &HashMap<(i64, i64, i64, i64), bool>) -> HashMap<(i64, i64, i64, i64), bool> {
    let min_x = grid.keys().map(|(x, _, _, _)| *x).min().unwrap() - 1;
    let max_x = grid.keys().map(|(x, _, _, _)| *x).max().unwrap() + 1;
    let min_y = grid.keys().map(|(_, y, _, _)| *y).min().unwrap() - 1;
    let max_y = grid.keys().map(|(_, y, _, _)| *y).max().unwrap() + 1;
    let min_z = grid.keys().map(|(_, _, z, _)| *z).min().unwrap() - 1;
    let max_z = grid.keys().map(|(_, _, z, _)| *z).max().unwrap() + 1;
    let min_w = grid.keys().map(|(_, _, _, w)| *w).min().unwrap() - 1;
    let max_w = grid.keys().map(|(_, _, _, w)| *w).max().unwrap() + 1;

    (min_w..=max_w)
        .flat_map(|w| {
            (min_z..=max_z)
                .flat_map(move |z| {
                    (min_x..=max_x)
                        .flat_map(move |x| {
                            (min_y..=max_y)
                                .map(move |y| {
                                    let value = NEIGHBORS.iter()
                                        .filter(|(dx, dy, dz, dw)| {
                                            *grid.get(&(x + dx, y + dy, z + dz, w + dw)).unwrap_or(&false)
                                        })
                                        .count();

                                    let result = match *grid.get(&(x, y, z, w)).unwrap_or(&false) {
                                        true => { (2..=3).contains(&value) },
                                        false => { value == 3 },
                                    };
                                    ((x, y, z, w), result)
                                })
                        })
                })
        })
        .collect()
}


fn main() {
    let mut grid = read();

    for _ in 0..6 { grid = step_grid(&grid); }

    render_grid(&grid);

    let count = grid.values().filter(|&v| *v).count();
    println!("number active: {}", count);
}
