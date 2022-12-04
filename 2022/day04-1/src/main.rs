use std::io;
use std::io::prelude::*;

use regex::Regex;


fn read() -> Vec<(u16, u16, u16, u16)> {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|x| {
            let line = x.unwrap();
            let cap = re.captures(&line).unwrap();

            (cap.get(1).unwrap().as_str().parse().unwrap(),
             cap.get(2).unwrap().as_str().parse().unwrap(),
             cap.get(3).unwrap().as_str().parse().unwrap(),
             cap.get(4).unwrap().as_str().parse().unwrap())
        })
        .collect()
}


fn full_overlap(x: &(u16, u16, u16, u16)) -> bool {
    if (x.0 <= x.2) && (x.1 >= x.3) { return true }
    if (x.2 <= x.0) && (x.3 >= x.1) { return true }
    false
}


fn main() {
    let assignments = read();

    let overlaps = assignments.into_iter()
        .filter(full_overlap)
        .count();

    println!("overlaps: {}", overlaps);
}
