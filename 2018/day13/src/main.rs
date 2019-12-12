use std::io;
use std::io::prelude::*;

use std::collections::HashMap;


#[derive(Clone, Copy)]
enum Track {
    Horizontal,
    Vertical,
    Intersection,
    UpCurve,
    DownCurve,
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Turn {
    Left,
    Straight,
    Right,
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Cart {
    x: usize,
    y: usize,
    direction: Direction,
    next_turn: Turn,
}


impl Cart {
    fn new(x: usize, y: usize, direction: Direction) -> Cart {
        Cart { x, y, direction, next_turn: Turn::Left }
    }

    fn next(&mut self, tracks: &HashMap<(usize, usize), Track>) {
        let new_x = match self.direction {
            Direction::Up => { self.x },
            Direction::Down => { self.x },
            Direction::Left => { self.x - 1 },
            Direction::Right => { self.x + 1 },
        };
        let new_y = match self.direction {
            Direction::Up => { self.y - 1 },
            Direction::Down => { self.y + 1 },
            Direction::Left => { self.y },
            Direction::Right => { self.y },
        };

        match tracks.get(&(new_x, new_y)).unwrap() {
            Track::Intersection => {
                match self.next_turn {
                    Turn::Left => {
                        self.direction = match self.direction {
                            Direction::Up => Direction::Left,
                            Direction::Down => Direction::Right,
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                        };
                        self.next_turn = Turn::Straight;
                    },
                    Turn::Straight => { self.next_turn = Turn::Right; },
                    Turn::Right => {
                        self.direction = match self.direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        };
                        self.next_turn = Turn::Left;
                    },
                }
            },
            Track::UpCurve => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                }
            },
            Track::DownCurve => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                }
            },
            _ => {},
        }

        self.x = new_x;
        self.y = new_y;
    }
}


fn read() -> (HashMap<(usize, usize), Track>, HashMap<(usize, usize), Cart>) {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    let tracks = lines.iter().enumerate().map(|(row, line)| {
        line.chars().enumerate().filter_map(|(col, c)| {
            match c {
                '-' | '<' | '>' => Some(((col, row), Track::Horizontal)),
                '|' | '^' | 'v' => Some(((col, row), Track::Vertical)),
                '+' => Some(((col, row), Track::Intersection)),
                '/' => Some(((col, row), Track::UpCurve)),
                '\\' => Some(((col, row), Track::DownCurve)),
                _ => None,
            }
        }).collect::<Vec<_>>()
    }).flatten().collect();

    let carts = lines.iter().enumerate().map(|(row, line)| {
        line.chars().enumerate().filter_map(|(col, c)| {
            match c {
                '<' => Some(((col, row), Cart::new(col, row, Direction::Left))),
                '>' => Some(((col, row), Cart::new(col, row, Direction::Right))),
                '^' => Some(((col, row), Cart::new(col, row, Direction::Up))),
                'v' => Some(((col, row), Cart::new(col, row, Direction::Down))),
                _ => None,
            }
        }).collect::<Vec<_>>()
    }).flatten().collect();

    (tracks, carts)
}


fn main() {
    let (tracks, mut carts) = read();

    loop {
        let mut to_process = carts.values().cloned().collect::<Vec<_>>();
        to_process.sort_by_key(|c| (c.y, c.x));

        for cart in to_process.iter_mut() {
            if carts.remove(&(cart.x, cart.y)).is_none() { continue; }

            cart.next(&tracks);
            if !carts.contains_key(&(cart.x, cart.y)) {
                carts.insert((cart.x, cart.y), *cart);
            } else {
                carts.remove(&(cart.x, cart.y));
            }
        }

        if carts.len() == 1 {
            let last = carts.values().last().unwrap();
            println!("last cart: ({},{})", last.x, last.y);
            break;
        }
    }
}
