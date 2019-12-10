use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use gcd::Gcd;


fn read() -> HashSet<(i16,i16)> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .enumerate()
        .map(|(y, line)| {
            line.unwrap().chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| (x as i16, y as i16))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}


fn main() {
    let asteroids = read();

    let mut best = 0;
    let mut best_coordinates = (0, 0);
    for (x, y) in &asteroids {
        let mut current = 0;

        for (x2, y2) in &asteroids {
            if x == x2 && y == y2 { continue; }
            if x == x2 {
                if !(1..(y2 - y).abs())
                    .any(|n| asteroids.contains(&(*x, if y2 >= y { y + n } else { y - n }))) { current += 1; }
                continue;
            }
            if y == y2 {
                if !(1..(x2 - x).abs())
                    .any(|n| asteroids.contains(&(if x2 >= x { x + n } else { x - n }, *y))) { current += 1; }
                continue;
            }

            let (dx, dy) = ((*x - *x2).abs() as u16, (*y - *y2).abs() as u16);

            let divisor = dx.gcd(dy);
            if divisor != 1 {
                let (x_step, y_step) = ((dx / divisor) as i16, (dy / divisor) as i16);

                assert_ne!(x_step, 0, "({}, {}), ({}, {})", x, y, x2, y2);
                assert_ne!(y_step, 0, "({}, {}), ({}, {})", x, y, x2, y2);

                if (1..divisor as i16)
                    .any(|s| {
                        asteroids.contains(&(if x2 >= x { *x + s * x_step } else { *x - s * x_step },
                                             if y2 >= y { *y + s * y_step } else { *y - s * y_step }))
                    }) { continue; }
            }

            current += 1;
        }

        if current > best { best = current; best_coordinates = (*x, *y); }
    }

    println!("coordinates: ({}, {}), detected: {}", best_coordinates.0, best_coordinates.1, best);
}
