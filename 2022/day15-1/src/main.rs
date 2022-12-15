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
    let sensor_info = read();
    let test_line: isize = 2_000_000;

    // println!("info: {:#?}", sensor_info);

    let excluded = sensor_info.iter()
        .map(|(x1, x2)| {
            let d = (x1.0 - x2.0).abs() + (x1.1 - x2.1).abs();
            (x1, d)
        })
        .filter(|(x1, d)| ((x1.1 - d)..=(x1.1 + d)).contains(&test_line))
        .flat_map(|(x1, d)| {
            let d2 = d - (x1.1 - test_line).abs();
            (x1.0 - d2)..=(x1.0 + d2)
        })
        .collect::<HashSet<isize>>();
    let beacons = sensor_info.iter()
        .filter(|(_, x2)| x2.1 == test_line)
        .map(|(_, x2)| x2.0)
        .collect::<HashSet<isize>>();

    println!("excluded at y={}: {}", test_line, excluded.difference(&beacons).collect::<Vec<_>>().len());
}
