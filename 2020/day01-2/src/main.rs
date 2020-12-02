use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

use itertools::Itertools;


fn read() -> HashSet<i64> {
    let stdin = io::stdin();
    stdin.lock().lines().map(|x| x.unwrap().parse().unwrap()).collect()
}


fn main() {
    let expenses = read();

    for (a, b) in expenses.iter().tuple_combinations() {
        let compliment = 2020 - a - b;
        if expenses.contains(&compliment) {
            println!("{} * {} * {} = {}", a, b, compliment, a * b * compliment);
            return
        }
    }
}
