use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use regex::Regex;


#[derive(Debug, Clone)]
enum Instruction {
    Mask(u64, u64),
    Mem(usize, u64),
}


fn read() -> Vec<Instruction> {
    let re_mask = Regex::new(r"^mask = ([01X]+)$").unwrap();
    let re_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|s| {
            let line = s.unwrap();
            if re_mask.is_match(&line) {
                let caps = re_mask.captures(&line).unwrap();
                let mask = &caps[1];

                Instruction::Mask(
                    u64::from_str_radix(&mask.replace("1", "0").replace("X", "1"), 2).unwrap(),
                    u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap()
                )
            }
            else {
                let caps = re_mem.captures(&line).unwrap();
                Instruction::Mem(caps[1].parse().unwrap(), caps[2].parse().unwrap())
            }
        })
        .collect()
}


fn main() {
    let program = read();

    let mut memory = HashMap::new();
    let mut disable = 0;
    let mut force = 0;

    for inst in program {
        match inst {
            Instruction::Mask(a, b) => { disable = a; force = b; },
            Instruction::Mem(loc, val) => { memory.insert(loc, val & disable | force); },
        }
    }

    println!("sum: {}", memory.values().sum::<u64>());
}
