use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use gcd::Gcd;
use itertools::Itertools;


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


fn find_best(asteroids: &HashSet<(i16, i16)>) -> (i16, i16) {
    let mut best = 0;
    let mut x_best = 0;
    let mut y_best = 0;

    for (x, y) in asteroids {
        let mut current = 0;

        for (x2, y2) in asteroids {
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

                if (1..divisor as i16)
                    .any(|s| {
                        asteroids.contains(&(if x2 >= x { *x + s * x_step } else { *x - s * x_step },
                                             if y2 >= y { *y + s * y_step } else { *y - s * y_step }))
                    }) { continue; }
            }

            current += 1;
        }

        if current > best { best = current; x_best = *x; y_best = *y; }
    }

    (x_best, y_best)
}


fn main() {
    let original_asteroids = read();
    let (x_best, y_best) = find_best(&original_asteroids);

    let mut asteroids = original_asteroids.iter()
        .filter(|(x, y)| !(*x == x_best && *y == y_best))
        .map(|(x, y)| {
            let (dx, dy) = (*x - x_best, *y - y_best);
            let divisor = (dx.abs() as u16).gcd(dy.abs() as u16) as i16;
            (dx / divisor, dy / divisor, divisor, *x, *y)
        })
        .collect::<Vec<(i16, i16, i16, i16, i16)>>();
    asteroids.sort_by(|a, b| {
        let a_polar = -(a.0 as f64).atan2(a.1 as f64);
        let b_polar = -(b.0 as f64).atan2(b.1 as f64);

        let cmp = a_polar.partial_cmp(&b_polar).unwrap();
        match cmp {
            Ordering::Equal => (a.2).cmp(&b.2),
            _ => cmp,
        }
    });

    let mut count = 0;
    let mut destroyed = HashSet::new();
    let mut x_d;
    let mut y_d;

    'outer: loop {
        for (_, group) in &asteroids.iter().group_by(|a| (a.0, a.1)) {
            for asteroid in group {
                x_d = asteroid.3;
                y_d = asteroid.4;
                let coordinates = (x_d, y_d);
                if !destroyed.contains(&coordinates) {
                    destroyed.insert(coordinates);
                    count += 1;
                    if count == 200 { break 'outer; }
                    break;
                }
            }
        }
    }

    println!("station coordinates: ({}, {})", x_best, y_best);
    println!("coordinates of 200th destroyed asteroid: ({}, {})", x_d, y_d);
}
