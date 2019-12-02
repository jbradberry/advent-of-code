use std::io;
use std::io::prelude::*;


fn read() -> Vec<u32> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.trim().split(',').map(|x| x.parse().unwrap()).collect()
}


fn execute(program: &mut Vec<u32>) {
    let mut ip = 0;
    loop {
        let opcode = program[ip];
        let (value, position) = match opcode {
            1 => { (program[program[ip + 1] as usize] + program[program[ip + 2] as usize],
                    program[ip + 3]) },
            2 => { (program[program[ip + 1] as usize] * program[program[ip + 2] as usize],
                    program[ip + 3]) },
            99 => { break },
            _ => { unreachable!() },
        };
        program[position as usize] = value;
        ip += 4;
    }
}


fn main() {
    let program = read();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut current = program.to_vec();
            current[1] = noun;
            current[2] = verb;
            execute(&mut current);

            if current[0] == 19690720 {
                println!("noun: {}, verb: {}", noun, verb);
                println!("value: {}", 100 * noun + verb);
                break;
            }
        }
    }
}
