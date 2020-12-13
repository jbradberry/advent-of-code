use std::io;
use std::io::prelude::*;


fn read() -> (u64, Vec<u64>) {
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

    let (min_wait, min_bus) = buses.iter().map(|&b| (b - earliest % b, b)).min().unwrap();

    println!("earliest: {}, buses: {:?}", earliest, buses);
    println!("least wait time: {}, bus #: {}", min_wait, min_bus);
    println!("solution: {}", min_wait as u64 * min_bus);
}
