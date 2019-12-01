use std::io;
use std::io::prelude::*;


fn read() -> Vec<u32> {
    let stdin = io::stdin();
    stdin.lock().lines().map(|x| x.unwrap().parse().unwrap()).collect()
}


fn fuel_req(mass: u32) -> u32 { mass / 3 - 2 }


fn main() {
    let mass = read();
    let total: u32 = mass.iter().map(|&m| fuel_req(m)).sum();

    println!("total fuel requirement: {}", total);
}
