use std::io;
use std::io::prelude::*;
use std::str::FromStr;


#[derive(Debug, PartialEq)]
enum OpCode {
    Accumulate,
    Jump,
    NoOp,
}


impl FromStr for OpCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(OpCode::Accumulate),
            "jmp" => Ok(OpCode::Jump),
            "nop" => Ok(OpCode::NoOp),
            _ => Err(()),
        }
    }
}


#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: OpCode,
    value: i32,
}


fn read() -> Vec<Instruction> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|s| {
            let line = s.unwrap();
            let mut parts = line.split_whitespace();
            Instruction { opcode: parts.next().unwrap().parse().unwrap(),
                          value: parts.next().unwrap().parse().unwrap() }
        })
        .collect()
}


fn main() {
    let program = read();

    println!("{:?}", program);
}
