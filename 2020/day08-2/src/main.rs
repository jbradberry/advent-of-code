use std::io;
use std::io::prelude::*;
use std::str::FromStr;

use std::collections::HashSet;


#[derive(Debug, Clone, Copy, PartialEq)]
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


#[derive(Debug, Clone, Copy, PartialEq)]
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

    let indexes = program.iter().enumerate()
        .filter_map(|(i, x)| match x {
            Instruction { opcode: OpCode::Accumulate, .. } => None,
            _ => Some(i),
        });

    for i in indexes {
        let mut acc: i32 = 0;
        let mut ip: usize = 0;
        let mut visited = HashSet::new();

        loop {
            if ip >= program.len() {
                println!("accumulator: {}", acc);
                break;
            }
            if visited.contains(&ip) { break; }
            visited.insert(ip);

            let instruction = match ip == i {
                true => Instruction {
                    opcode: match program[ip].opcode {
                        OpCode::NoOp => OpCode::Jump,
                        OpCode::Jump => OpCode::NoOp,
                        OpCode::Accumulate => OpCode::Accumulate,
                    },
                    value: program[ip].value
                },
                false => program[ip],
            };

            match instruction {
                Instruction { opcode: OpCode::NoOp, .. } => {},
                Instruction { opcode: OpCode::Accumulate, value: x } => { acc += x; },
                Instruction { opcode: OpCode::Jump, value: x } => {
                    ip = ((ip as i32) + x) as usize;
                    continue;
                },
            }
            ip += 1;
        }
    }
}
