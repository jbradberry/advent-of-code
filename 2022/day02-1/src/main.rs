use std::io;
use std::io::prelude::*;


fn read() -> Vec<Vec<char>> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(
            |x| x.unwrap()
                .split(" ")
                .map(|c| c.parse().unwrap()).collect()
        )
        .collect()
}


fn main() {
    let strategy = read();

    println!("strategy: {:?}", strategy);
}
