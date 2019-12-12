use std::cmp::Ordering;
use std::fmt;
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


impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{},{},{},{}", self.x, self.y, self.z, self.vx, self.vy, self.vz)
    }
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
    let mut moon_records = vec![moons.to_vec()];

    for _ in 0..2_000_000 {
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

        moon_records.push(moons.to_vec());
    }

    let mut cycles = Vec::new();
    for m_i in 0..4 {
        for field in 0..3 {
            let candidates = moon_records[0..1_000_000].iter()
                .enumerate()
                .filter_map(|(i, rec)| match field {
                    0 => if rec[m_i].x == moon_records[0][m_i].x { Some(i) } else { None },
                    1 => if rec[m_i].y == moon_records[0][m_i].y { Some(i) } else { None },
                    2 => if rec[m_i].z == moon_records[0][m_i].z { Some(i) } else { None },
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            let mut cand_iter = candidates.iter();
            cand_iter.next();
            let mut candidate = 1;
            for &c in cand_iter {
                if moon_records[0..c].iter()
                    .zip(moon_records[c..2*c].iter())
                    .all(|(a, b)| match field {
                        0 => a[m_i].x == b[m_i].x,
                        1 => a[m_i].y == b[m_i].y,
                        2 => a[m_i].z == b[m_i].z,
                        _ => unreachable!(),
                    }) {
                        candidate = c;
                        break;
                    }
            }
            cycles.push(candidate);
        }
    }

    println!("cycle steps: {:?}", cycles);
    println!("steps to repeat: {}", cycles.iter().fold(1, |a, b| a.lcm(&b)));
}
