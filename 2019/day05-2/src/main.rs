use std::io;
use std::io::prelude::*;


#[derive(Clone, Copy)]
enum Mode {
    Position,
    Immediate,
}


impl Mode {
    fn from_int(value: i32) -> Mode {
        match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("Unrecognized operand mode: {}", value),
        }
    }
}


struct Program {
    code: Vec<i32>,
    ip: usize,
    input: i32,
}


impl Program {
    fn from_str(buffer: &str) -> Program {
        Program { code: buffer.trim().split(',').map(|x| x.parse().unwrap()).collect(), ip: 0, input: 5 }
    }

    fn read(&mut self, mode: Mode) -> i32 {
        let operand = self.code[self.ip];
        self.ip += 1;

        match mode {
            Mode::Position => self.code[operand as usize],
            Mode::Immediate => operand,
        }
    }

    fn write(&mut self, value: i32, mode: Mode) {
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
            let modes = (2..5).map(|x| Mode::from_int(self.code[self.ip] / 10i32.pow(x) % 10)).collect::<Vec<_>>();
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
        self.write(self.input, modes[0]);
    }

    fn opcode_4(&mut self, modes: &[Mode]) {
        println!("output: {}", self.read(modes[0]));
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


fn read() -> Program {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    Program::from_str(&buffer)
}


fn main() {
    let mut program = read();
    program.execute();
}
