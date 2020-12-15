use std::io;
use std::io::prelude::*;
use std::collections::HashMap;


fn read() -> Vec<u64> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.trim().split(",").map(|s| s.parse().unwrap()).collect()
}


fn say(last_said: &mut HashMap<u64, usize>, num: u64, counter: usize) -> u64 {
    // println!("say -- num: {}, counter: {}", num, counter);
    let prev = *last_said.get(&num).unwrap_or(&counter);
    last_said.insert(num, counter);

    (counter - prev) as u64
}


fn main() {
    let list = read();
    let mut previous = list.iter().enumerate().map(|(i, n)| (*n, i + 1)).collect::<HashMap<_, _>>();

    let mut last = *list.iter().last().unwrap();
    let mut counter = list.len();
    loop {
        last = say(&mut previous, last, counter);
        counter += 1;

        // println!("turn: {}, number: {}", counter, last);

        if counter >= 2020 { break; }
    }

    println!("number 2020: {}", last);
}
