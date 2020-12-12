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


impl Direction {
    fn left(&self, value: i32) -> Self {
        let mut n = value;
        let mut dir = self;

        while n > 0 {
            dir = match dir {
                Direction::North => &Direction::West,
                Direction::West => &Direction::South,
                Direction::South => &Direction::East,
                Direction::East => &Direction::North,
                _ => unreachable!(),
            };
            n -= 90;
        }

        *dir
    }

    fn right(&self, value: i32) -> Self {
        let mut n = value;
        let mut dir = self;

        while n > 0 {
            dir = match dir {
                Direction::North => &Direction::East,
                Direction::East => &Direction::South,
                Direction::South => &Direction::West,
                Direction::West => &Direction::North,
                _ => unreachable!(),
            };
            n -= 90;
        }

        *dir
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
struct Action {
    direction: Direction,
    value: i32,
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
    let (mut east, mut north) = (0i32, 0i32);
    let (mut e_way, mut n_way) = (10i32, 1i32);

    for Action { direction, value } in actions {
        match direction {
            Direction::North => { n_way += value; },
            Direction::South => { n_way -= value; },
            Direction::East => { e_way += value; },
            Direction::West => { e_way -= value; },
            Direction::Left => {
                let mut v = value;
                while v > 0 {
                    let (n, e) = (e_way, -n_way);
                    n_way = n;
                    e_way = e;
                    v -= 90;
                }
            },
            Direction::Right => {
                let mut v = value;
                while v > 0 {
                    let (n, e) = (-e_way, n_way);
                    n_way = n;
                    e_way = e;
                    v -= 90;
                }
            },
            Direction::Forward => {
                north += n_way * value;
                east += e_way * value;
            },
        }
    }

    println!("north: {}, east: {}", north, east);
    println!("Manhattan distance: {}", north.abs() + east.abs());
}
