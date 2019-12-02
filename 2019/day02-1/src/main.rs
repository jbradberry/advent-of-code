use std::io;
use std::io::prelude::*;


fn read() -> Vec<u32> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.trim().split(',').map(|x| x.parse().unwrap()).collect()
}


fn main() {
    let mut program = read();

    // replicate the original conditions
    program[1] = 12;
    program[2] = 2;

    let mut index = 0;

    loop {
        let opcode = program[index];
        let (value, position) = match opcode {
            1 => { (program[program[index + 1] as usize] + program[program[index + 2] as usize],
                    program[index + 3]) },
            2 => { (program[program[index + 1] as usize] * program[program[index + 2] as usize],
                    program[index + 3]) },
            99 => { break },
            _ => { unreachable!() },
        };
        program[position as usize] = value;
        index += 4;
    }

    println!("value at position 0: {}", program[0]);
}
