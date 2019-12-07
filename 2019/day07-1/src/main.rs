use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

use itertools::Itertools;


#[derive(Clone, Copy)]
enum Mode {
    Position,
    Immediate,
}


impl Mode {
    fn from_int(value: i64) -> Mode {
        match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("Unrecognized operand mode: {}", value),
        }
    }
}


struct Program {
    code: Vec<i64>,
    ip: usize,
    input: VecDeque<i64>,
    output: Vec<i64>,
}


impl Program {
    fn from_code(code: &[i64], input: &[i64]) -> Program {
        Program { code: Vec::from(code), ip: 0, input: VecDeque::from_iter(input.iter().cloned()),
                  output: Vec::new() }
    }

    fn read(&mut self, mode: Mode) -> i64 {
        let operand = self.code[self.ip];
        self.ip += 1;

        match mode {
            Mode::Position => self.code[operand as usize],
            Mode::Immediate => operand,
        }
    }

    fn write(&mut self, value: i64, mode: Mode) {
        let operand = self.code[self.ip];
        self.ip += 1;

        match mode {
            Mode::Position => { self.code[operand as usize] = value; },
            Mode::Immediate => panic!("Writes can only happen in position mode."),
        }
    }

    fn execute(&mut self) {
        loop {
            let opcode = self.code[self.ip] % 100;
            let modes = (2..5).map(|x| Mode::from_int(self.code[self.ip] / 10i64.pow(x) % 10)).collect::<Vec<_>>();
            self.ip += 1;

            match opcode {
                1 => self.opcode_1(&modes),
                2 => self.opcode_2(&modes),
                3 => self.opcode_3(&modes),
                4 => self.opcode_4(&modes),
                5 => self.opcode_5(&modes),
                6 => self.opcode_6(&modes),
                7 => self.opcode_7(&modes),
                8 => self.opcode_8(&modes),
                99 => break,
                _ => panic!("Unrecognized opcode: {}", opcode),
            }
        }
    }

    fn opcode_1(&mut self, modes: &[Mode]) {
        let op1 = self.read(modes[0]);
        let op2 = self.read(modes[1]);
        self.write(op1 + op2, modes[2]);
    }

    fn opcode_2(&mut self, modes: &[Mode]) {
        let op1 = self.read(modes[0]);
        let op2 = self.read(modes[1]);
        self.write(op1 * op2, modes[2]);
    }

    fn opcode_3(&mut self, modes: &[Mode]) {
        let value = self.input.pop_front().unwrap();
        self.write(value, modes[0]);
    }

    fn opcode_4(&mut self, modes: &[Mode]) {
        let value = self.read(modes[0]);
        self.output.push(value);
    }

    fn opcode_5(&mut self, modes: &[Mode]) {
        let op1 = self.read(modes[0]);
        let op2 = self.read(modes[1]);

        if op1 > 0 {
            self.ip = op2 as usize;
        }
    }

    fn opcode_6(&mut self, modes: &[Mode]) {
        let op1 = self.read(modes[0]);
        let op2 = self.read(modes[1]);

        if op1 == 0 {
            self.ip = op2 as usize;
        }
    }

    fn opcode_7(&mut self, modes: &[Mode]) {
        let op1 = self.read(modes[0]);
        let op2 = self.read(modes[1]);

        if op1 < op2 { self.write(1, modes[2]); }
        else { self.write(0, modes[2]); }
    }

    fn opcode_8(&mut self, modes: &[Mode]) {
        let op1 = self.read(modes[0]);
        let op2 = self.read(modes[1]);

        if op1 == op2 { self.write(1, modes[2]); }
        else { self.write(0, modes[2]); }
    }
}


fn read() -> Vec<i64> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.trim().split(',').map(|x| x.parse().unwrap()).collect()
}


fn main() {
    let code = read();
    let mut max_value = 0;

    for settings in (0..=4).permutations(5) {
        let mut input_signal = 0;
        let mut output = 0;

        for x in settings {
            let mut program = Program::from_code(&code, &[x, input_signal]);
            program.execute();
            output = program.output[0];
            input_signal = output;
        }
        if output > max_value { max_value = output; }
    }

    println!("maximum output: {}", max_value);
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ex1() {
        let code = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];

        let mut p1 = Program::from_code(&code, &[4, 0]);
        p1.execute();
        let mut p2 = Program::from_code(&code, &[3, p1.output[0]]);
        p2.execute();
        let mut p3 = Program::from_code(&code, &[2, p2.output[0]]);
        p3.execute();
        let mut p4 = Program::from_code(&code, &[1, p3.output[0]]);
        p4.execute();
        let mut p5 = Program::from_code(&code, &[0, p4.output[0]]);
        p5.execute();

        assert_eq!(p5.output[0], 43210);
    }

    #[test]
    fn test_ex2() {
        let code = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];

        let mut p1 = Program::from_code(&code, &[0, 0]);
        p1.execute();
        let mut p2 = Program::from_code(&code, &[1, p1.output[0]]);
        p2.execute();
        let mut p3 = Program::from_code(&code, &[2, p2.output[0]]);
        p3.execute();
        let mut p4 = Program::from_code(&code, &[3, p3.output[0]]);
        p4.execute();
        let mut p5 = Program::from_code(&code, &[4, p4.output[0]]);
        p5.execute();

        assert_eq!(p5.output[0], 54321);
    }

    #[test]
    fn test_ex3() {
        let code = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                        1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];

        let mut p1 = Program::from_code(&code, &[1, 0]);
        p1.execute();
        let mut p2 = Program::from_code(&code, &[0, p1.output[0]]);
        p2.execute();
        let mut p3 = Program::from_code(&code, &[4, p2.output[0]]);
        p3.execute();
        let mut p4 = Program::from_code(&code, &[3, p3.output[0]]);
        p4.execute();
        let mut p5 = Program::from_code(&code, &[2, p4.output[0]]);
        p5.execute();

        assert_eq!(p5.output[0], 65210);
    }
}
