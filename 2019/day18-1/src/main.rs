use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;


fn read() -> HashMap<(usize, usize), char> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.unwrap().char_indices()
                .filter(|(_, c)| *c != '#')
                .map(move |(x, c)| ((x, y), c)).collect::<Vec<_>>()
        })
        .collect()
}


fn path_pairs(maze: &HashMap<(usize, usize), char>) -> HashMap<(char, char), u16> {
    let chars = maze.iter()
        .filter_map(|((x, y), c)| if *c != '.' { Some((*c, (*x, *y))) } else { None })
        .collect::<HashMap<char, (usize, usize)>>();

    let mut result = HashMap::new();
    for (&chr, &(x, y)) in chars.iter() {
        let mut prev = vec![(x, y)];
        let mut explored: HashSet<(usize, usize)> = prev.iter().cloned().collect();
        for d in 1.. {
            let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
            let curr = prev.iter().cloned()
                .flat_map(|(a0, b0)| {
                    directions.iter().cloned()
                        .filter_map(|(dx, dy)| {
                            let a1 = ((a0 as i32) + dx) as usize;
                            let b1 = ((b0 as i32) + dy) as usize;

                            // If this point is part of the wall of the maze, skip it.
                            if !maze.contains_key(&(a1, b1)) { return None; }

                            // If we had a shorter path to this spot, ignore.
                            if explored.contains(&(a1, b1)) { return None; }

                            // If this spot is a key or a door or the beginning, end the segment.
                            let &c = maze.get(&(a1, b1)).unwrap_or(&'.');
                            if c != '.' {
                                result.entry((chr, c)).or_insert(d);
                                result.entry((c, chr)).or_insert(d);
                                return None;
                            }
                            Some((a1, b1))
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>();

            prev = curr.iter().cloned().collect();
            if prev.is_empty() { break; }
            explored.extend(prev.iter().cloned());
        }
    }

    result
}


fn solve(maze: &HashMap<(usize, usize), char>, pairs: &HashMap<(char, char), u16>) -> u16 {
    let chars = maze.iter()
        .filter_map(|((x, y), c)| if *c != '.' { Some((*c, (*x, *y))) } else { None })
        .collect::<HashMap<char, (usize, usize)>>();

    let mut path = vec!['@'];

    0
}


fn main() {
    let maze = read();

    println!("{:?}", maze);
    println!("length: {}", maze.len());

    let segments = path_pairs(&maze);

    println!("{:?}", segments);
}
