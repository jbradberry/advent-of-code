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


fn main() {
    let assignments = read();

    println!("assignments: {:?}", assignments);
}
