use std::io;
use std::io::prelude::*;

use std::str::FromStr;


#[derive(Debug)]
enum OpCode {
    Noop,
    Addx(i16),
}


impl FromStr for OpCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" ").collect::<Vec<_>>();
        match split[0] {
            "noop" => Ok(OpCode::Noop),
            "addx" => Ok(OpCode::Addx(split[1].parse().unwrap())),
            _ => Err(()),
        }
    }
}


fn read() -> Vec<OpCode> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .map(|l| OpCode::from_str(&l.unwrap()).unwrap())
        .collect()
}


fn main() {
    let instructions = read();
    println!("instructions: {:#?}", instructions);
}
