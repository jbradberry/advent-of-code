use std::io;
use std::io::prelude::*;

use std::str::FromStr;

use itertools::Itertools;


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

    let crt: String = cycles.iter()
        .enumerate()
        .map(|(i, x)| {
            let col = (i % 40) as i16;
            if x - 1 <= col && col <= x + 1 { '#' } else { '.' }
        })
        .collect();

    let lines = crt.chars()
        .chunks(40)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<_>>();

    for line in lines {
        println!("{}", line);
    }
}
