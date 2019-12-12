use std::io;
use std::io::prelude::*;

use regex::Regex;


#[derive(Debug)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
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
        }
    }).collect()
}


fn main() {
    let moons = read();

    println!("{:?}", moons);
}
