use std::collections::{HashSet, HashMap};

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


fn display(rope: &[(i16, i16); 10]) {
    let grid = rope.iter().enumerate().map(|(i, x)| (x, i)).collect::<HashMap<_, _>>();

    let min_x = rope.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = rope.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = rope.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = rope.iter().map(|(_, y)| *y).max().unwrap();

    for y in (min_y..=max_y).rev() {
        let row: String = (min_x..=max_x)
            .map(|x| match grid.get(&(x, y)) {
                Some(v) => v.to_string().chars().next().unwrap(),
                None if (x == 0 && y == 0) => 's',
                None => '.',
            })
            .collect();
        println!("{}", row);
    }
    println!("");
}


fn main() {
    let moves = read();

    let mut rope = [(0, 0); 10];
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for (mv, inc) in moves {
        for _ in 0..inc {
            rope[0] = match mv {
                Move::Up => (rope[0].0, rope[0].1 + 1),
                Move::Down => (rope[0].0, rope[0].1 - 1),
                Move::Left => (rope[0].0 - 1, rope[0].1),
                Move::Right => (rope[0].0 + 1, rope[0].1),
            };

            for i in 1..10 {
                let (mut x1, mut y1): (i16, i16) = rope[i];
                let (x0, y0): (i16, i16) = rope[i - 1];

                if (x0 - x1).abs() > 1 {
                    x1 += (x0 - x1) >> 1;
                    if y1 != y0 { y1 += (y0 - y1).signum(); }

                    // println!("x1: {:?}", rope[i]);
                }
                else if (y0 - y1).abs() > 1 {
                    y1 += (y0 - y1) >> 1;
                    if x1 != x0 { x1 += (x0 - x1).signum(); }

                    // println!("x1: {:?}", rope[i]);
                }

                rope[i] = (x1, y1);
            }
            visited.insert(rope[9]);
            // display(&rope);

            // println!("rope: {:?}", rope);
        }

        display(&rope);
    }

    println!("visited: {}", visited.len());
}
