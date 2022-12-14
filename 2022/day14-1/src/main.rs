use std::io;
use std::io::prelude::*;


fn read() -> Vec<Vec<(usize, usize)>> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .map(|l| {
            let line = l.unwrap();
            line.split(" -> ")
                .map(|c| {
                    let mut coord = c.split(',');
                    (coord.next().unwrap().parse().unwrap(), coord.next().unwrap().parse().unwrap())
                })
                .collect()
        })
        .collect()
}


fn main() {
    let paths = read();

    println!("paths: {:?}", paths);
}
