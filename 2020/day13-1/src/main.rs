use std::io;
use std::io::prelude::*;


fn read() -> (u32, Vec<u32>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|s| s.unwrap());

    let earliest = lines.next().unwrap().parse().unwrap();
    let buses = lines.next().unwrap().split(",")
        .filter_map(|s| s.parse().ok())
        .collect();

    (earliest, buses)
}


fn main() {
    let (earliest, buses) = read();

    println!("earliest: {}, buses: {:?}", earliest, buses);
}
