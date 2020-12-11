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


fn calculate_next(grid: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    grid.iter().enumerate()
        .map(|(i, r)| {
            r.iter().enumerate()
                .map(|(j, s)| {
                    let occupied = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].iter()
                        .filter(|(a, b)| {
                            if (i as i64) + a < 0 || (i as i64) + a >= (grid.len() as i64) { return false; }
                            if (j as i64) + b < 0 || (j as i64) + b >= (grid[0].len() as i64) { return false; }
                            grid[((i as i64) + a) as usize][((j as i64) + b) as usize] == Seat::Occupied
                        })
                        .count();
                    match s {
                        Seat::Floor => Seat::Floor,
                        Seat::Empty => {
                            if occupied == 0 { Seat::Occupied }
                            else { Seat::Empty }
                        },
                        Seat::Occupied => {
                            if occupied >= 4 { Seat::Empty }
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
