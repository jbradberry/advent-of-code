use std::io;
use std::io::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}


impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => Self::Floor,
        }
    }
}


fn read() -> Vec<Vec<Seat>> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|s| {
            s.unwrap().chars()
                .map(|c| Seat::from(c))
                .collect()
        })
        .collect()
}


fn occupied_direction(grid: &Vec<Vec<Seat>>, center: (usize, usize), direction: (i64, i64)) -> bool {
    let (mut r, mut c) = (center.0 as i64, center.1 as i64);
    loop {
        r += direction.0;
        c += direction.1;

        if r < 0 || c < 0 { return false; }
        if r >= (grid.len() as i64) || c >= (grid[0].len() as i64) { return false; }

        match grid[r as usize][c as usize] {
            Seat::Occupied => { return true; },
            Seat::Empty => { return false; },
            Seat::Floor => {},
        }
    }
}


fn calculate_next(grid: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    grid.iter().enumerate()
        .map(|(i, r)| {
            r.iter().enumerate()
                .map(|(j, s)| {
                    let occupied = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].iter()
                        .filter(|&dir| occupied_direction(&grid, (i, j), *dir))
                        .count();
                    match s {
                        Seat::Floor => Seat::Floor,
                        Seat::Empty => {
                            if occupied == 0 { Seat::Occupied }
                            else { Seat::Empty }
                        },
                        Seat::Occupied => {
                            if occupied >= 5 { Seat::Empty }
                            else { Seat::Occupied }
                        },
                    }
                })
                .collect()
        })
        .collect()
}


fn main() {
    let mut prev = read();
    let mut grid = calculate_next(&prev);

    let mut count = 1;
    while prev != grid {
        count += 1;
        prev = grid;
        grid = calculate_next(&prev);
    }

    println!("{} steps", count);
    println!("{} occupied seats",
             grid.iter().map(|r| r.iter().filter(|&s| *s == Seat::Occupied).count()).sum::<usize>());
}
