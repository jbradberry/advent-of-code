use std::io;
use std::io::prelude::*;

use std::collections::HashSet;


fn read() -> (Vec<(char, u16)>, Vec<(char, u16)>) {
    let stdin = io::stdin();
    let mut wires = stdin.lock().lines().map(|x| {
        x.unwrap().trim().split(',').map(|item| {
            let mut item_chars = item.chars();
            let dir = item_chars.next().unwrap();
            let dist = item_chars.collect::<String>().parse().unwrap();

            (dir, dist)
        }).collect()
    }).collect::<Vec<_>>();

    let wires1 = wires.pop().unwrap();
    let wires0 = wires.pop().unwrap();

    (wires0, wires1)
}


fn dist(w1: &[(i16, i16)], w2: &[(i16, i16)], x: i16, y: i16) -> i32 {
    (w1.iter().position(|(a, b)| *a == x && *b == y).unwrap() as i32 +
     w2.iter().position(|(a, b)| *a == x && *b == y).unwrap() as i32 + 2)
}


fn wire_to_points(wire: &[(char, u16)]) -> Vec<(i16, i16)> {
    let mut set = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for (dir, l) in wire {
        for _ in 1..=*l {
            match dir {
                'R' => { x += 1; },
                'L' => { x -= 1; },
                'U' => { y += 1; },
                'D' => { y -= 1; },
                _ => unreachable!(),
            }
            set.push((x, y));
        }
    }

    set
}


fn main() {
    let (wire1, wire2) = read();
    let w1 = wire_to_points(&wire1);
    let w2 = wire_to_points(&wire2);
    let ws1 = w1.iter().collect::<HashSet<_>>();
    let ws2 = w2.iter().collect::<HashSet<_>>();

    let min_dist = ws1.intersection(&ws2).map(|(x, y)| dist(&w1, &w2, *x, *y)).min().unwrap();

    println!("minimum distance: {}", min_dist);
}
