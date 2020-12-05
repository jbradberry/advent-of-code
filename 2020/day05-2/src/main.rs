use std::io;
use std::io::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Front,
    Back,
    Left,
    Right,
}


fn to_id(pass: &Vec<Direction>) -> u16 {
    let row = pass.iter().cloned()
        .fold(0, |acc, x| {
            match x {
                Direction::Front => 2 * acc,
                Direction::Back => 2 * acc + 1,
                _ => acc,
            }
        });
    let col = pass.iter().cloned()
        .fold(0, |acc, x| {
            match x {
                Direction::Left => 2 * acc,
                Direction::Right => 2 * acc + 1,
                _ => acc,
            }
        });

    8 * row + col
}


fn read() -> Vec<Vec<Direction>> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|x| {
            x.unwrap().chars()
                .map(|c| match c {
                    'F' => Direction::Front,
                    'B' => Direction::Back,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => { unreachable!() },
                })
                .collect()
        })
        .collect()
}


fn main() {
    let passes = read();

    let mut seats = passes.iter().map(|p| to_id(p)).collect::<Vec<_>>();
    seats.sort();

    let (a, _) = seats.iter().zip(&seats[1..]).filter(|(&a, &b)| b == a + 2).next().unwrap();

    println!("your seat: {}", a + 1);
}
