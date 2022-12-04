use std::io;
use std::io::prelude::*;


fn read() -> Vec<Vec<String>> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|line| line.unwrap())
        .map(|s| {
            let (x, y) = s.split_at(s.len() / 2);
            vec![x.to_string(), y.to_string()]
        })
        .collect()
}


fn main() {
    let backpacks = read();

    println!("backpacks: {:?}", backpacks);
}
