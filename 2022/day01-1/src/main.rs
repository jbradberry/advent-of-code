use std::io;
use std::io::prelude::*;


fn read() -> Vec<Vec<u32>> {
    let mut supplies = Vec::new();
    let mut current = Vec::new();

    let stdin = io::stdin();
    for s in stdin.lock().lines() {
        let line = s.unwrap();

        if line == "" {
            supplies.push(current);
            current = Vec::new();
        } else {
            current.push(line.parse().unwrap());
        }
    }
    supplies.push(current);

    supplies
}


fn main() {
    let supplies = read();

    let max_cals = supplies.iter().map(|v| v.iter().sum::<u32>()).max();

    println!("maximum calories: {}", max_cals.unwrap());
}
