use std::collections::HashSet;

use std::io;
use std::io::prelude::*;


#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}


fn read() -> Vec<(Move, u8)> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .map(|x| {
            let line = x.unwrap();
            let split = line.split(" ").collect::<Vec<_>>();
            match split[0] {
                "U" => (Move::Up, split[1].parse().unwrap()),
                "D" => (Move::Down, split[1].parse().unwrap()),
                "L" => (Move::Left, split[1].parse().unwrap()),
                "R" => (Move::Right, split[1].parse().unwrap()),
                _ => unreachable!()
            }
        })
        .collect()
}


fn main() {
    let moves = read();

    println!("moves: {:?}", moves);

    let mut head: (i16, i16) = (0, 0);
    let mut tail: (i16, i16) = (0, 0);

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for (mv, inc) in moves {
        for _ in 0..inc {
            match mv {
                Move::Up => {
                    head = (head.0, head.1 + 1);
                    if head.1 > tail.1 + 1 {
                        tail = (tail.0, tail.1 + 1);
                        if tail.0 < head.0 { tail = (tail.0 + 1, tail.1); }
                        if tail.0 > head.0 { tail = (tail.0 - 1, tail.1); }
                    }
                },
                Move::Down => {
                    head = (head.0, head.1 - 1);
                    if head.1 < tail.1 - 1 {
                        tail = (tail.0, tail.1 - 1);
                        if tail.0 < head.0 { tail = (tail.0 + 1, tail.1); }
                        if tail.0 > head.0 { tail = (tail.0 - 1, tail.1); }
                    }
                },
                Move::Left => {
                    head = (head.0 - 1, head.1);
                    if head.0 < tail.0 - 1 {
                        tail = (tail.0 - 1, tail.1);
                        if tail.1 < head.1 { tail = (tail.0, tail.1 + 1); }
                        if tail.1 > head.1 { tail = (tail.0, tail.1 - 1); }
                    }
                },
                Move::Right => {
                    head = (head.0 + 1, head.1);
                    if head.0 > tail.0 + 1 {
                        tail = (tail.0 + 1, tail.1);
                        if tail.1 < head.1 { tail = (tail.0, tail.1 + 1); }
                        if tail.1 > head.1 { tail = (tail.0, tail.1 - 1); }
                    }
                },
            };
            visited.insert(tail);
            println!("head: {:?}, tail: {:?}", head, tail);
        }
    }

    // println!("head: {:?}, tail: {:?}", head, tail);
    println!("visited: {}", visited.len());
}
