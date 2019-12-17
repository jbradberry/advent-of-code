use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;

use intcode;


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
    let mut program = intcode::Program::from_code(&code);

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

    while program.state != intcode::State::Halted && !unvisited.is_empty() {
        program.execute();

        if program.state == intcode::State::IBlock {
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

        if program.state == intcode::State::OBlock {
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
