use std::io;
use std::io::prelude::*;


#[derive(Clone, Copy)]
enum Mode {
    Position,
    Immediate,
    Relative,
}


impl Mode {
    fn from_int(value: i64) -> Mode {
        match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Unrecognized operand mode: {}", value),
        }
    }
}


#[derive(PartialEq, Eq)]
enum State {
    Executing,
    IOBlocked,
    Halted,
}


struct Program {
    code: Vec<i64>,
    ip: usize,
    rb: usize,
    state: State,
    input: Option<i64>,
    output: Option<i64>,
}


impl Program {
    fn from_code(code: &[i64]) -> Program {
        Program { code: Vec::from(code), ip: 0, rb: 0, state: State::Executing, input: None, output: None }
    }

    fn read(&mut self, mode: Mode) -> i64 {
        let operand = self.code[self.ip];
        self.ip += 1;

        match mode {
            Mode::Position => self.code[operand as usize],
            Mode::Immediate => operand,
            Mode::Relative => self.code[operand as usize + self.rb],
        }
    }

    fn write(&mut self, value: i64, mode: Mode) {
        let operand = self.code[self.ip] as usize;
        self.ip += 1;

        let address = match mode {
            Mode::Position => { operand },
            Mode::Relative => { operand + self.rb },
            Mode::Immediate => panic!("Writes can only happen in position or relative mode."),
        };

        if address >= self.code.len() { self.code.resize(address + 1, 0); }
        self.code[address] = value;
    }

    fn execute(&mut self) {
        match self.state {
            State::Executing => {},
            State::IOBlocked => { if self.input.is_some() { self.state = State::Executing; } },
            State::Halted => { return },
        }

        while self.state == State::Executing {
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
                9 => self.opcode_9(&modes),
                99 => { self.state = State::Halted; self.ip -= 1; },
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
        match self.input {
            Some(input) => { self.write(input, modes[0]); self.input = None; self.state = State::Executing; },
            None => { self.state = State::IOBlocked; self.ip -= 1; },
        }
    }

    fn opcode_4(&mut self, modes: &[Mode]) {
        let value = self.read(modes[0]);
        match self.output {
            Some(output) => panic!("Previous output not read: {}", output),
            None => { self.output = Some(value); },
        }
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

    fn opcode_9(&mut self, modes: &[Mode]) {
        let value = self.read(modes[0]);
        self.rb = (self.rb as i64 + value) as usize;
    }
}


fn read() -> Vec<i64> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.trim().split(',').map(|x| x.parse().unwrap()).collect()
}


fn main() {
    let code = read();
    let mut program = Program::from_code(&code);
    program.input = Some(1);
    program.execute();

    match program.output {
        Some(output) => println!("output: {}", output),
        None => println!("missing output!"),
    };
}
