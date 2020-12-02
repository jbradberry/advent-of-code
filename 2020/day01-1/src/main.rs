use std::io;
use std::io::prelude::*;
use std::collections::HashSet;


fn read() -> HashSet<u32> {
    let stdin = io::stdin();
    stdin.lock().lines().map(|x| x.unwrap().parse().unwrap()).collect()
}


fn main() {
    let expenses = read();

    for &x in expenses.iter() {
        let compliment = 2020 - x;
        if expenses.contains(&compliment) {
            println!("{} * {} = {}", x, compliment, x * compliment);
            return
        }
    }
}
