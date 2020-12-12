use std::io;
use std::io::prelude::*;
use std::str::FromStr;


#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}


#[derive(Debug, Clone, Copy, PartialEq)]
struct Action {
    direction: Direction,
    value: u16,
}


impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s[1..].parse().unwrap();
        match s.chars().next().unwrap() {
            'N' => Ok(Action { direction: Direction::North, value }),
            'S' => Ok(Action { direction: Direction::South, value }),
            'E' => Ok(Action { direction: Direction::East, value }),
            'W' => Ok(Action { direction: Direction::West, value }),
            'L' => Ok(Action { direction: Direction::Left, value }),
            'R' => Ok(Action { direction: Direction::Right, value }),
            'F' => Ok(Action { direction: Direction::Forward, value }),
            _ => Err(()),
        }
    }
}


fn read() -> Vec<Action> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|s| Action::from_str(&s.unwrap()).unwrap())
        .collect()
}


fn main() {
    let actions = read();

    println!("{:?}", actions);
}
