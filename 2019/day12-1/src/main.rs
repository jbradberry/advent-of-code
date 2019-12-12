use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;

use itertools::Itertools;
use regex::Regex;


#[derive(Debug, Clone, Copy)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}


fn read() -> Vec<Moon> {
    let re = Regex::new(r"^<x=([-0-9]+),[ ]*y=([-0-9]+),[ ]*z=([-0-9]+)>$").unwrap();
    let stdin = io::stdin();
    stdin.lock().lines().map(|x| {
        let line = x.unwrap();
        let cap = re.captures(&line).unwrap();

        Moon {
            x: cap.get(1).unwrap().as_str().parse().unwrap(),
            y: cap.get(2).unwrap().as_str().parse().unwrap(),
            z: cap.get(3).unwrap().as_str().parse().unwrap(),
            vx: 0, vy: 0, vz: 0,
        }
    }).collect()
}


fn display_energy(moons: &[Moon]) {
    let mut total = Vec::new();
    for moon in moons {
        let pot = vec![moon.x.abs(), moon.y.abs(), moon.z.abs()];
        let kin = vec![moon.vx.abs(), moon.vy.abs(), moon.vz.abs()];
        let pot_sum: i32 = pot.iter().sum();
        let kin_sum: i32 = kin.iter().sum();

        println!(
            "pot: {} = {}; kin: {} = {};   total: {} * {} = {}",
            pot.iter().join(" + "), pot_sum,
            kin.iter().join(" + "), kin_sum,
            pot_sum, kin_sum, pot_sum * kin_sum
        );
        total.push(pot_sum * kin_sum);
    }
    println!("Sum of total energy: {} = {}", total.iter().join(" + "), total.iter().sum::<i32>());
}


fn main() {
    let mut moons = read();

    for _step in 0..1000 {
        for pair in (0..4).combinations(2) {
            let mut m1 = moons[pair[0]];
            let mut m2 = moons[pair[1]];

            match m1.x.cmp(&m2.x) {
                Ordering::Less => { m1.vx += 1; m2.vx -= 1; },
                Ordering::Greater => { m1.vx -= 1; m2.vx += 1; },
                Ordering::Equal => {},
            }
            match m1.y.cmp(&m2.y) {
                Ordering::Less => { m1.vy += 1; m2.vy -= 1; },
                Ordering::Greater => { m1.vy -= 1; m2.vy += 1; },
                Ordering::Equal => {},
            }
            match m1.z.cmp(&m2.z) {
                Ordering::Less => { m1.vz += 1; m2.vz -= 1; },
                Ordering::Greater => { m1.vz -= 1; m2.vz += 1; },
                Ordering::Equal => {},
            }

            moons[pair[0]] = m1;
            moons[pair[1]] = m2;
        }

        for moon in &mut moons {
            moon.x += moon.vx;
            moon.y += moon.vy;
            moon.z += moon.vz;
        }
    }

    display_energy(&moons);
}
