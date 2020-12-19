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


fn reachable_keys(pairs: &HashMap<char, HashMap<char, usize>>,
                  path: &[char], dist: usize) -> Vec<(Vec<char>, usize)> {
    let mut results = Vec::new();
    let mut found = HashMap::new();
    let endpoint = path.iter().last().unwrap();
    let mut queue = VecDeque::new();
    queue.push_back((vec![*endpoint], dist));

    while let Some((curr_path, curr_dist)) = queue.pop_front() {
        let curr_end = curr_path.iter().last().unwrap();

        for (conn, delta) in &pairs[curr_end] {
            if curr_path.contains(&conn) { continue; }
            if conn.is_ascii_uppercase() && !path.contains(&conn.to_ascii_lowercase()) { continue; }

            let mut new_path = curr_path.to_vec();
            new_path.push(*conn);

            if conn.is_ascii_lowercase() && !path.contains(&conn) {
                if found.contains_key(conn) && found[conn] <= curr_dist + delta { continue; }

                let mut total_path = path.to_vec();
                total_path.extend(new_path[1..].iter().copied());
                results.push((total_path, curr_dist + delta));
                found.insert(conn, curr_dist + delta);
            } else {
                queue.push_back((new_path, curr_dist + delta));
            }
        }
    }

    results.sort_by_key(|t| t.1);
    results
}


fn solve(pairs: &HashMap<char, HashMap<char, usize>>) -> Option<(Vec<char>, usize)> {
    let mut queue = VecDeque::new();
    queue.push_back((vec!['@'], 0));
    let mut solution = None;

    while let Some((path, distance)) = queue.pop_front() {
        // println!("{:?}", path);
        for (new_path, new_dist) in reachable_keys(pairs, &path, distance) {
            // println!("{:?}", new_path);

            // if we have a solution already:
            //   if this is a solution and is better, replace
            //   otherwise:
            //     if it is already greater than the existing solution, ignore
            //     otherwise, push to queue
            // otherwise:
            //   if this is a solution, store
            //   otherwise, push to queue

            let remaining_keys = pairs.keys()
                .filter(|k| k.is_ascii_lowercase() && !new_path.contains(k))
                .count();

            if let Some((_, dst)) = solution {
                if remaining_keys == 0 && new_dist < dst {
                    solution = Some((new_path, new_dist));
                    continue;
                }

                if new_dist >= dst { continue; }
            }

            if remaining_keys == 0 {
                solution = Some((new_path, new_dist));
                continue;
            }

            queue.push_back((new_path, new_dist));
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

    let (solution, dist) = solve(&segments).unwrap();
    println!("solution: {:?}", solution);
    println!("distance: {}", dist);
}
