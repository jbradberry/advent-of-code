use std::io;
use std::io::prelude::*;

use std::collections::HashSet;

use regex::Regex;


fn read() -> Vec<((isize, isize), (isize, isize))> {
    let re = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|l| {
            let line = l.unwrap();
            let cap = re.captures(&line).unwrap();

            ((cap.get(1).unwrap().as_str().parse().unwrap(), cap.get(2).unwrap().as_str().parse().unwrap()),
             (cap.get(3).unwrap().as_str().parse().unwrap(), cap.get(4).unwrap().as_str().parse().unwrap()))
        })
        .collect()
}


fn main() {
    let mut sensor_info = read();
    sensor_info.sort_by_key(|(x1, x2)| (x1.0 - x2.0).abs() + (x1.1 - x2.1).abs());

    // let boundary: isize = 20;
    let boundary: isize = 4_000_000;

    // let mut results = HashSet::new();
    let mut result = (0, 0);
    for i in 0..sensor_info.len() {
        // If we assume that there is exactly one possible square for
        // the beacon, it stands to reason that the possible location
        // must be in the ring of distance d + 1 for at least one
        // sensor.  So, build up each sensor's ring and prune it based
        // on the exclusion zone for every other sensor.

        let ((x1, y1), (x2, y2)) = sensor_info[i];
        let d1 = (x1 - x2).abs() + (y1 - y2).abs() + 1;

        let mut s = (-d1..=d1)
            .flat_map(|r| vec![(x1 - (d1 - r.abs()), y1 + r), (x1 + (d1 - r.abs()), y1 + r)])
            .filter(|(a, b)| (a >= &0) && (a <= &boundary) && (b >= &0) && (b <= &boundary))
            .collect::<HashSet<(isize, isize)>>();

        for j in 0..sensor_info.len() {
            if i == j { continue; }
            println!("i: {}, j: {}, s.len(): {}", i, j, s.len());

            let ((x3, y3), (x4, y4)) = sensor_info[j];
            let d2 = (x3 - x4).abs() + (y3 - y4).abs();

            s.retain(|(a, b)| (x3 - a).abs() + (y3 - b).abs() > d2);
            if s.is_empty() { break; }
        }

        if !s.is_empty() {
            result = s.into_iter().next().unwrap();
            println!("result: {:?}", result);
            break;
        }
    }

    println!("tuning frequency: {}", 4_000_000 * result.0 + result.1);
}
