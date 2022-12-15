use std::io;
use std::io::prelude::*;

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

    println!("info: {:#?}", sensor_info);
}
