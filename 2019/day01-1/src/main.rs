use std::io;
use std::io::prelude::*;


fn read() -> Vec<u32> {
    let stdin = io::stdin();
    stdin.lock().lines().map(|x| x.unwrap().parse().unwrap()).collect()
}


fn main() {
    let mass = read();

    println!("{:?}", mass);
}
