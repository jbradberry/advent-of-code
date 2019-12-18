use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use itertools::Itertools;

use intcode;


#[derive(Debug, Clone, Copy)]
enum Action {
    Forward,
    Left,
    Right,
}


#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}


impl Direction {
    fn forward(&self, scaffold: &HashSet<(usize, usize)>, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::North => {
                if y > 0 && scaffold.contains(&(x, y - 1)) { Some((x, y - 1)) }
                else { None }
            },
            Direction::South => {
                if scaffold.contains(&(x, y + 1)) { Some((x, y + 1)) }
                else { None }
            },
            Direction::West => {
                if x > 0 && scaffold.contains(&(x - 1, y)) { Some((x - 1, y)) }
                else { None }
            },
            Direction::East => {
                if scaffold.contains(&(x + 1, y)) { Some((x + 1, y)) }
                else { None }
            },
        }
    }

    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct Robot {
    x: usize,
    y: usize,
    direction: Direction,
}


impl Robot {
    fn step(&mut self, scaffold: &HashSet<(usize, usize)>) {
        let (x, y) = self.direction.forward(scaffold, self.x, self.y).unwrap();
        self.x = x;
        self.y = y;
    }
}


fn read() -> Vec<i64> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.trim().split(',').map(|x| x.parse().unwrap()).collect()
}


fn find_intersections(lines: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut intersections = Vec::new();
    for y in 1..(lines.len() as i16 - 1) {
        for x in 1..(lines[0].len() as i16 - 1) {
            if [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)].iter().all(|(a, b)| {
                lines[(y + b) as usize][(x + a) as usize] == '#'
            }) {
                intersections.push((x as usize, y as usize));
            }
        }
    }

    intersections
}


fn find_robot(lines: &Vec<Vec<char>>) -> Robot {
    lines.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, c)| {
                    match *c {
                        '^' => Some(Robot { x, y, direction: Direction::North }),
                        'v' => Some(Robot { x, y, direction: Direction::South }),
                        '<' => Some(Robot { x, y, direction: Direction::West }),
                        '>' => Some(Robot { x, y, direction: Direction::East }),
                        _ => None,
                    }
                })
        })
        .next().unwrap()
}


fn find_scaffold(lines: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    lines.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, c)| if *c != '.' { Some((x, y)) } else { None })
        })
        .collect::<HashSet<_>>()
}


fn path(scaffold: &HashSet<(usize, usize)>, robot: &mut Robot) -> Vec<Action> {
    // This is kind of a half-assed solution, since it doesn't try to optimize at all

    let mut actions = Vec::new();
    loop {
        // can we go forward?
        match robot.direction.forward(scaffold, robot.x, robot.y) {
            None => {
                if robot.direction.left().forward(scaffold, robot.x, robot.y).is_some() {
                    actions.push(Action::Left);
                    robot.direction = robot.direction.left();
                }
                else if robot.direction.right().forward(scaffold, robot.x, robot.y).is_some() {
                    actions.push(Action::Right);
                    robot.direction = robot.direction.right();
                }
                else { break; }
            },
            Some(_) => {
                actions.push(Action::Forward);
                robot.step(scaffold);
            },
        }
    }

    actions
}


fn path_to_routine(path: Vec<Action>) -> String {
    String::new()
}


fn main() {
    let code = read();

    // Part 1

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

    let intersections = find_intersections(&lines);
    println!("Part 1: {}", intersections.iter().map(|(x, y)| x * y).sum::<usize>());

    // Part 2

    let mut code2 = code.to_vec();
    code2[0] = 2;  // wake up the robot
    program = intcode::Program::from_code(&code2);

    let original_robot = find_robot(&lines);
    let scaffold = find_scaffold(&lines);

    let mut robot = original_robot.clone();
    let p = path(&scaffold, &mut robot);

    println!("Part 2: {:?}", p);

    // let movement = solve(&lines, &intersections);
    // let mut movement_chars = movement.chars();

    // // let mut output = String::new();
    // while program.state != intcode::State::Halted {
    //     program.execute();

    //     if program.state == intcode::State::IBlock {
    //         program.input = Some(movement_chars.next().unwrap() as i64);
    //     }

    //     if program.state == intcode::State::OBlock {
    //         // output.push((program.output.unwrap() as u8) as char);
    //         program.output = None;
    //     }
    // }

}
