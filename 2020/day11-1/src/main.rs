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


fn main() {
    let grid = read();

    println!("{:?}", grid);
}
