use std::collections::HashSet;
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

    let lines = output.split('\n')
        .filter_map(|s| { if s.len() > 0 { Some(s.chars().collect::<Vec<_>>()) } else { None } })
        .collect::<Vec<_>>();

    let mut intersections = HashSet::new();
    for y in 1..(lines.len() as i16 - 1) {
        for x in 1..(lines[0].len() as i16 - 1) {
            if [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)].iter().all(|(a, b)| {
                lines[(y + b) as usize][(x + a) as usize] == '#'
            }) {
                intersections.insert((x, y));
            }
        }
    }

    println!("Part 1: {}", intersections.iter().map(|(x, y)| x * y).sum::<i16>());
}
