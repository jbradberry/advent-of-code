use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq)]
enum Space {
    Open,
    Wall,
    Explorer,
    Key(char),
    Door(char),
}


fn read() -> Vec<Vec<Space>> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .map(|s| {
            s.unwrap().chars()
                .map(|c| match c {
                    '.' => Space::Open,
                    '#' => Space::Wall,
                    '@' => Space::Explorer,
                    x if ('a'..='z').contains(&x) => Space::Key(x),
                    x if ('A'..='Z').contains(&x) => Space::Door(x),
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}


fn nearest_items(map: &Vec<Vec<Space>>, r: usize, c: usize) -> HashSet<(char, usize)> {
    let mut steps = 0;
    let mut found = HashSet::new();
    let mut explored: HashSet<(usize, usize)> = HashSet::new();
    let mut prev = HashSet::new();
    prev.insert((r, c));

    while !prev.is_empty() {
        steps += 1;
        explored.extend(&prev);

        let mut curr = HashSet::new();
        for (i, j) in prev {
            for (a, b) in &[(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let x = ((i as i64) + a) as usize;
                let y = ((j as i64) + b) as usize;
                if explored.contains(&(x, y)) { continue; }

                match map[x][y] {
                    Space::Wall => { continue; },
                    Space::Open => {},
                    Space::Explorer => { found.insert(('@', steps)); continue; },
                    Space::Key(v) | Space::Door(v) => { found.insert((v, steps)); continue; },
                }

                curr.insert((x, y));
            }
        }
        prev = curr;
    }

    found
}


fn path_pairs(map: &Vec<Vec<Space>>) -> HashMap<char, Vec<(char, usize)>> {
    map.iter().enumerate()
        .flat_map(|(i, r)| {
            r.iter().enumerate()
                .filter(|(_, &s)| s != Space::Open && s != Space::Wall)
                .map(move |(j, s)| {
                    let ch = match s {
                        Space::Explorer => '@',
                        Space::Key(value) | Space::Door(value) => *value,
                        _ => unreachable!(),
                    };
                    (ch, nearest_items(map, i, j).into_iter().collect())
                })
        })
        .collect()
}


// fn solve_step(
//     pairs: &HashMap<(char, char), u16>, keys: &HashSet<char>, path: &[char], dist: u16
// ) -> (Vec<char>, u16) {
//     let last = path.last().unwrap();
//     let mut options = pairs.keys().filter(|(k0, k1)| k0 == last).collect::<Vec<_>>();

//     if last.is_ascii_lowercase() {}

//     (Vec::new(), 0)
// }


// fn solve(maze: &HashMap<(usize, usize), char>, pairs: &HashMap<(char, char), u16>) -> u16 {
//     let chars = maze.iter()
//         .filter_map(|((x, y), c)| if *c != '.' { Some((*c, (*x, *y))) } else { None })
//         .collect::<HashMap<char, (usize, usize)>>();
//     let keys = chars.keys().cloned().filter(|k| k.is_ascii_lowercase()).collect::<HashSet<_>>();

//     let path = vec!['@'];

//     solve_step(pairs, &keys, &path, 0).1
// }


fn solve(pairs: &HashMap<char, Vec<(char, usize)>>) -> Vec<(usize, char, char)> {
    Vec::new()
}


fn main() {
    let maze = read();

    println!("{:?}", maze);
    println!("size: {} * {}", maze.len(), maze[0].len());

    let segments = path_pairs(&maze);
    println!("{:?}", segments);

    // solve(&segments);
}
