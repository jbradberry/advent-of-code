use std::io;
use std::io::prelude::*;


fn read() -> Vec<u32> {
    let stdin = io::stdin();
    stdin.lock().lines().map(|x| x.unwrap().parse().unwrap()).collect()
}


fn fuel_req(mass: u32) -> u32 {
    let third = mass / 3;
    if third >= 2 { third - 2 }
    else { 0 }
}


fn rocket_fuel_req(mass: u32) -> u32 {
    let mut total_fuel = 0;
    let mut fuel = mass;

    loop {
        fuel = fuel_req(fuel);
        if fuel > 0 { total_fuel += fuel; }
        else { break }
    }

    total_fuel
}


fn main() {
    let mass = read();
    let total: u32 = mass.iter().map(|&m| rocket_fuel_req(m)).sum();

    println!("total fuel requirement: {}", total);
}
