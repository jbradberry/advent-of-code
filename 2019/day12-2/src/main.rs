use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;

use itertools::Itertools;
use regex::Regex;
use num::Integer;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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


fn main() {
    let mut moons = read();
    let original_moons = moons.to_vec();
    let mut cycles = (0..24).map(|_| 0).collect::<Vec<u64>>();

    let mut step = 0u64;
    loop {
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

        step += 1;
        for m_i in 0..4 {
            let m = moons[m_i];
            let m_o = original_moons[m_i];
            let values = vec![(m.x, m_o.x), (m.y, m_o.y), (m.z, m_o.z),
                              (m.vx, m_o.vx), (m.vy, m_o.vy), (m.vz, m_o.vz)];
            for (i, (c, c_o)) in values.iter().enumerate() {
                if cycles[6 * m_i + i] == 0 && c == c_o {
                    cycles[6 * m_i + i] = step;
                }
            }
        }

        if cycles.iter().all(|&x| x > 0) { break; }
    }

    println!("cycle steps: {:?}", cycles);
    println!("steps to repeat: {}", cycles.iter().fold(1, |a, b| a.lcm(&b)));
}
