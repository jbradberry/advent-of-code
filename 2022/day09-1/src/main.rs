use std::io;
use std::io::prelude::*;


#[derive(Debug)]
enum Move {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}


fn read() -> Vec<Move> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .map(|x| {
            let line = x.unwrap();
            let split = line.split(" ").collect::<Vec<_>>();
            match split[0] {
                "U" => Move::Up(split[1].parse().unwrap()),
                "D" => Move::Down(split[1].parse().unwrap()),
                "L" => Move::Left(split[1].parse().unwrap()),
                "R" => Move::Right(split[1].parse().unwrap()),
                _ => unreachable!()
            }
        })
        .collect()
}


fn main() {
    let moves = read();

    println!("moves: {:?}", moves);
}
