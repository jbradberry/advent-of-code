use std::io;
use std::io::prelude::*;

use regex::Regex;


#[derive(Debug)]
struct Star {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}


fn read_stars() -> Vec<Star> {
    let re = Regex::new(r"^position=<[ ]*([-0-9]+),[ ]*([-0-9]+)> velocity=<[ ]*([-0-9]+),[ ]*([-0-9]+)>$")
        .unwrap();
    let stdin = io::stdin();

    stdin.lock().lines().map(|x| {
        let line = x.unwrap();
        let cap = re.captures(&line).unwrap();

        Star {
            x: cap.get(1).unwrap().as_str().parse().unwrap(),
            y: cap.get(2).unwrap().as_str().parse().unwrap(),
            vx: cap.get(3).unwrap().as_str().parse().unwrap(),
            vy: cap.get(4).unwrap().as_str().parse().unwrap(),
        }
    }).collect()
}


fn animate_stars(stars: &[Star]) {
    for t in 0..10000000 {
        let x_min = stars.iter().map(|s| s.x + t * s.vx).min().unwrap();
        let x_max = stars.iter().map(|s| s.x + t * s.vx).max().unwrap();
        let y_min = stars.iter().map(|s| s.y + t * s.vy).min().unwrap();
        let y_max = stars.iter().map(|s| s.y + t * s.vy).max().unwrap();

        if y_max - y_min <= 18 {
            let height = y_max - y_min + 1;
            let width = x_max - x_min + 1;
            let mut display = Vec::new();

            for _ in 0..height {
                for _ in 0..width {
                    display.push(46);  // '.'
                }

                // '\n'
                display.push(13);
                display.push(10);
            }

            for s in stars {
                let y = s.y + t * s.vy - y_min;
                let x = s.x + t * s.vx - x_min;
                display[(y * (width + 2) + x) as usize] = 35;
            }

            println!("t = {}", t);
            println!("{}", String::from_utf8(display).unwrap());

            break;
        }
    }
}


fn main() {
    let stars = read_stars();
    animate_stars(&stars);
}
