use std::io;
use std::io::prelude::*;

use counter::Counter;
use itertools::Itertools;
use regex::Regex;


fn part01() {
    let stdin = io::stdin();
    let patterns = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    let mut counts: Counter<(u16, u16), u16> = Counter::new();
    let re = Regex::new(r"^[#](\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    for line in patterns {
        let caps = re.captures(&line).unwrap();
        let x0 = caps.get(2).unwrap().as_str().parse().unwrap();
        let x1: u16 = caps.get(4).unwrap().as_str().parse().unwrap();
        let y0 = caps.get(3).unwrap().as_str().parse().unwrap();
        let y1: u16 = caps.get(5).unwrap().as_str().parse().unwrap();

        counts += (x0..x0+x1).cartesian_product(y0..y0+y1);
    }

    let overlaps = counts.values().filter(|&&x| x > 1).count();
    println!("overlaps = {}", overlaps);
}


fn main() {
    let stdin = io::stdin();
    let patterns = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    let mut counts: Counter<(u16, u16), u16> = Counter::new();
    let re = Regex::new(r"^[#](\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    for line in &patterns {
        let caps = re.captures(&line).unwrap();
        let x0 = caps.get(2).unwrap().as_str().parse().unwrap();
        let x1: u16 = caps.get(4).unwrap().as_str().parse().unwrap();
        let y0 = caps.get(3).unwrap().as_str().parse().unwrap();
        let y1: u16 = caps.get(5).unwrap().as_str().parse().unwrap();

        counts += (x0..x0+x1).cartesian_product(y0..y0+y1);
    }

    for line in &patterns {
        let caps = re.captures(&line).unwrap();
        let id = caps.get(1).unwrap().as_str();
        let x0 = caps.get(2).unwrap().as_str().parse().unwrap();
        let x1: u16 = caps.get(4).unwrap().as_str().parse().unwrap();
        let y0 = caps.get(3).unwrap().as_str().parse().unwrap();
        let y1: u16 = caps.get(5).unwrap().as_str().parse().unwrap();

        if (x0..x0+x1)
            .cartesian_product(y0..y0+y1)
            .any(|x| counts.get(&x).unwrap() > &1) { continue; }

        println!("unique id = {}", id);
        return
    }
}
