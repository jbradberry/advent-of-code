use std::collections::{HashMap, HashSet};
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
    IBlock,
    OBlock,
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

        let address = match mode {
            Mode::Position => operand as usize,
            Mode::Immediate => { return operand; },
            Mode::Relative => { operand as usize + self.rb },
        };

        if address >= self.code.len() { self.code.resize(address + 1, 0); }
        self.code[address]
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
            State::IBlock => { if self.input.is_some() { self.state = State::Executing; } },
            State::OBlock => { if self.output.is_none() { self.state = State::Executing; } },
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
            None => { self.state = State::IBlock; self.ip -= 1; },
        }
    }

    fn opcode_4(&mut self, modes: &[Mode]) {
        let value = self.read(modes[0]);
        match self.output {
            Some(_) => { self.ip -= 2; },
            None => { self.output = Some(value); },
        }
        self.state = State::OBlock;
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


#[derive(Debug, Clone, Copy)]
enum Direction {
    North = 1,
    South,
    West,
    East,
}


impl Direction {
    fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}


enum Status {
    Collision = 0,
    Moved,
    Found,
}


impl Status {
    fn from_int(value: i64) -> Status {
        match value {
            0 => Status::Collision,
            1 => Status::Moved,
            2 => Status::Found,
            _ => panic!("Unrecognized robot status: {}", value),
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
enum Space {
    Empty,
    Wall,
    OxygenSystem,
    Unknown,
}


struct Robot {
    x: i64,
    y: i64,
}


impl Robot {
    fn movement(&self, dir: Direction) -> (i64, i64) {
        match dir {
            Direction::North => (self.x, self.y + 1),
            Direction::South => (self.x, self.y - 1),
            Direction::West => (self.x - 1, self.y),
            Direction::East => (self.x + 1, self.y),
        }
    }
}


fn display(visited: &HashMap<(i64, i64), Space>, robot: &Robot) {
    print!("\x1b[{};{}H\x1b[2J", 0, 0);
    let x_min = *visited.keys().map(|(x, _)| x).min().unwrap();
    let x_max = *visited.keys().map(|(x, _)| x).max().unwrap();
    let y_min = *visited.keys().map(|(_, y)| y).min().unwrap();
    let y_max = *visited.keys().map(|(_, y)| y).max().unwrap();

    for y in y_min..=y_max {
        let line = (x_min..=x_max)
            .map(|x| {
                if x == 0 && y == 0 { return 'o'; }
                if robot.x == x && robot.y == y { return 'D'; }
                let space = visited.get(&(x, y)).unwrap_or(&Space::Unknown);
                match space {
                    Space::Unknown => ' ',
                    Space::Empty => '.',
                    Space::Wall => '#',
                    Space::OxygenSystem => '*',
                }
            })
            .collect::<String>();
        println!("{}", line);
    }
}


fn find_shortest_path_len(visited: &HashMap<(i64, i64), Space>) -> usize {
    let mut dist = 0;
    let mut last = [(0, 0)].iter().cloned().collect::<HashSet<_>>();
    let mut checked = HashSet::new();

    loop {
        dist += 1;
        let mut current = HashSet::new();
        for (x, y) in &last { checked.insert((*x, *y)); }
        for (x, y) in &last {
            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy);
                let space = visited.get(&(nx, ny)).unwrap_or(&Space::Unknown);
                if *space == Space::OxygenSystem { return dist; }
                if *space != Space::Wall && !checked.contains(&(nx, ny)) {
                    current.insert((nx, ny));
                }
            }
        }

        last = current;
    }
}


fn find_longest_path_len(visited: &HashMap<(i64, i64), Space>) -> usize {
    let (x_o, y_o) = visited.iter()
        .filter_map(|((x, y), s)| if *s == Space::OxygenSystem { Some((*x, *y)) } else { None })
        .next().unwrap();

    let mut dist = 0;
    let mut last = HashSet::new();
    last.insert((x_o, y_o));
    let mut checked = HashSet::new();

    loop {
        dist += 1;
        let mut current = HashSet::new();
        for (x, y) in &last { checked.insert((*x, *y)); }
        for (x, y) in &last {
            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy);
                let space = visited.get(&(nx, ny)).unwrap_or(&Space::Unknown);
                if *space != Space::Wall && !checked.contains(&(nx, ny)) {
                    current.insert((nx, ny));
                }
            }
        }

        if current.is_empty() { return dist - 1; }
        last = current;
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

    let mut direction = Direction::North;
    program.input = Some(direction as i64);

    let mut robot = Robot { x: 0, y: 0 }; 
    let mut status;
    let mut visited = HashMap::new();
    visited.insert((0, 0), Space::Empty);
    let mut unvisited =
        [Direction::North, Direction::South, Direction::West, Direction::East]
        .iter()
        .map(|&d| robot.movement(d))
        .collect::<HashSet<_>>();
    let mut path: Vec<Direction> = Vec::new();

    while program.state != State::Halted && !unvisited.is_empty() {
        program.execute();

        if program.state == State::IBlock {
            let mut adjacent = Vec::new();
            for &dir in &[Direction::North, Direction::South, Direction::West, Direction::East] {
                let (x, y) = robot.movement(dir);
                let space = visited.get(&(x, y)).unwrap_or(&Space::Unknown);
                match space {
                    Space::Unknown => { unvisited.insert((x, y)); adjacent.push(dir); },
                    _ => {},
                }
            }

            if adjacent.is_empty() {
                // going back the way we came, until we come up adjacent to other unknown spaces
                direction = path.pop().unwrap().reverse();
            } else {
                direction = adjacent[0];
                path.push(direction);
            }
            program.input = Some(direction as i64);
            program.execute();
        }

        if program.state == State::OBlock {
            status = Status::from_int(program.output.unwrap());
            program.output = None;
            let (x, y) = robot.movement(direction);
            unvisited.remove(&(x, y));
            match status {
                Status::Collision => {
                    visited.insert((x, y), Space::Wall);
                    path.pop();
                },
                Status::Moved => {
                    visited.insert((x, y), Space::Empty);
                    robot.x = x;
                    robot.y = y;
                },
                Status::Found => {
                    visited.insert((x, y), Space::OxygenSystem);
                    robot.x = x;
                    robot.y = y;
                },
            }
        }
        display(&visited, &robot);
    }

    println!("\nPart 1 -- Shortest path: {}", find_shortest_path_len(&visited));
    println!("\nPart 2 -- Longest path: {}", find_longest_path_len(&visited));
}
