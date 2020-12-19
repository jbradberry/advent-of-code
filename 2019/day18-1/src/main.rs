use std::collections::{HashMap, HashSet, VecDeque};
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


fn nearest_items(map: &Vec<Vec<Space>>, r: usize, c: usize) -> HashMap<char, usize> {
    let mut steps = 0;
    let mut found = HashMap::new();
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
                    Space::Explorer => { found.entry('@').or_insert(steps); continue; },
                    Space::Key(v) | Space::Door(v) => { found.entry(v).or_insert(steps); continue; },
                }

                curr.insert((x, y));
            }
        }
        prev = curr;
    }

    found
}


fn path_pairs(map: &Vec<Vec<Space>>) -> HashMap<char, HashMap<char, usize>> {
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
                    (ch, nearest_items(map, i, j))
                })
        })
        .collect()
}


fn solve(pairs: &HashMap<char, HashMap<char, usize>>) -> Option<usize> {
    let all_keys = pairs.keys()
        .filter(|k| k.is_ascii_lowercase())
        .fold(0, |acc, k| acc | (1 << (k.to_digit(36).unwrap() - 10)));
    let mut distances = HashMap::new();
    distances.insert(('@', 0), 0);

    let mut queue = VecDeque::new();
    queue.push_back(('@', 0));
    let mut solution = None;

    while let Some((node, keys)) = queue.pop_front() {
        let distance = distances[&(node, keys)];

        if let Some(dst) = solution {
            if distance >= dst { continue; }
        }

        for (next_node, delta) in &pairs[&node] {
            if let Some(dst) = solution {
                if distance + delta >= dst { continue; }
            }

            let mut next_keys = keys;
            if next_node.is_ascii_lowercase() {
                next_keys |= 1 << (next_node.to_digit(36).unwrap() - 10);
            }

            match distances.get(&(*next_node, next_keys)) {
                Some(dst) => {
                    if distance + delta < *dst {
                        distances.insert((*next_node, next_keys), distance + delta);
                    } else {
                        continue;
                    }
                },
                None => {
                    distances.insert((*next_node, next_keys), distance + delta);
                },
            }

            if next_keys == all_keys {
                solution = Some(distance + delta);
            } else {
                queue.push_back((*next_node, next_keys));
            }
        }
    }

    solution
}


fn main() {
    let maze = read();
    // println!("{:?}", maze);
    println!("size: {} * {}", maze.len(), maze[0].len());

    let segments = path_pairs(&maze);
    // println!("segments: {:?}", segments);

    let dist = solve(&segments).unwrap();
    // println!("solution: {:?}", solution);
    println!("distance: {}", dist);
}
