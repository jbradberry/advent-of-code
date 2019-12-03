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


fn dist(x0: i16, y0: i16, x1: i16, y1: i16) -> i16 {
    (x1 - x0).abs() + (y1 - y0).abs()
}


fn wire_to_points(wire: &[(char, u16)]) -> HashSet<(i16, i16)> {
    let mut set = HashSet::new();
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
            set.insert((x, y));
        }
    }

    set
}


fn main() {
    let (wire1, wire2) = read();
    let ws1 = wire_to_points(&wire1);
    let ws2 = wire_to_points(&wire2);

    let min_dist = ws1.intersection(&ws2).map(|(x, y)| dist(0, 0, *x, *y)).min().unwrap();

    println!("minimum distance: {}", min_dist);
}
