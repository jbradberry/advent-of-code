use std::io;
use std::io::prelude::*;


fn read() -> Vec<u64> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.trim().split(",").map(|s| s.parse().unwrap()).collect()
}


fn main() {
    let list = read();

    println!("{:?}", list);
}
