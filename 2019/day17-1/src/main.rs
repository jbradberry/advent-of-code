use std::io;
use std::io::prelude::*;

use intcode;


fn read() -> Vec<i64> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.trim().split(',').map(|x| x.parse().unwrap()).collect()
}


fn main() {
    let code = read();
    let mut program = intcode::Program::from_code(&code);

    let mut output = String::new();
    while program.state != intcode::State::Halted {
        program.execute();

        if program.state == intcode::State::OBlock {
            output.push((program.output.unwrap() as u8) as char);
            program.output = None;
        }
    }

    println!("{}", output);
}
