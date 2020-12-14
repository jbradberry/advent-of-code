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
    let mut exes = 0;
    let mut ones = 0;

    // println!("{:?}", program);

    for inst in program {
        match inst {
            Instruction::Mask(a, b) => { exes = a; ones = b; },
            Instruction::Mem(loc, val) => {
                let new_loc = (loc & !(exes as usize)) | (ones as usize);
                // println!("new_loc: {:#b}", new_loc);
                let bits = (0..64).rev().filter(|b| (exes & (1 << b)) > 0).collect::<Vec<u8>>();
                // println!("bits: {:?}", bits);
                let mut set = vec![0; bits.len()];

                loop {
                    // println!("set: {:?}", set);
                    let mask = bits.iter().zip(&set)
                        .map(|(b, x)| x << b)
                        .fold(0, |acc, x| acc | x);
                    // println!("mem[{}] = {}", new_loc | mask, val);
                    memory.insert(new_loc | mask, val);

                    if set.iter().all(|&b| b == 1) { break; }

                    for i in (0..bits.len()).rev() {
                        match set[i] {
                            0 => { set[i] = 1; break; },
                            1 => { set[i] = 0; },
                            _ => unreachable!(),
                        }
                    }
                }
            },
        }
    }

    // println!("{:?}", memory);
    println!("sum: {}", memory.values().sum::<u64>());
}

