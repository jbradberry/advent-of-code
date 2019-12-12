use std::io;
use std::io::prelude::*;

use counter::Counter;


fn part01() {
    let stdin = io::stdin();
    let sum: i32 = stdin.lock().lines().map(|x| x.unwrap().parse::<i32>().unwrap()).sum();
    println!("result = {}", sum);
}


fn main() {
    let stdin = io::stdin();
    let lst: Vec<i32> = stdin.lock().lines().map(|x| x.unwrap().parse().unwrap()).collect();
    let mut current = 0i32;
    let mut visited = HashSet::new();

    loop {
        for x in &lst {
            visited.insert(current);
            current += x;
            if visited.contains(&current) {
                println!("first revisited frequency = {}", current);
                return
            }
        }
    }
}
