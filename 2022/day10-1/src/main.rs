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
    // println!("instructions: {:?}", instructions);

    let mut current = 1;
    let cycles = instructions.iter()
        .flat_map(|inst| {
            match inst {
                OpCode::Noop => { vec![current] },
                OpCode::Addx(x) => {
                    let old = current;
                    current += x;
                    vec![old, old]
                },
            }
        })
        .collect::<Vec<_>>();

    // println!("cycles: {:#?}", cycles);

    let results = cycles.iter()
        .enumerate()
        .filter(|(i, _)| (i + 1) % 40 == 20)
        .map(|(i, x)| (i + 1) as i32 * (*x) as i32)
        .sum::<i32>();

    println!("results: {}", results);
}
