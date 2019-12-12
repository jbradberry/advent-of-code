use std::io;
use std::io::prelude::*;

use std::collections::HashSet;

use regex::Regex;


fn read_instructions() -> Vec<(String, String)> {
    let re = Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$").unwrap();
    let stdin = io::stdin();

    stdin.lock().lines().map(|x| {
        let line = x.unwrap();
        let cap = re.captures(&line).unwrap();

        (cap.get(1).unwrap().as_str().to_string(), cap.get(2).unwrap().as_str().to_string())
    }).collect()
}


fn topological_sort(edges: &Vec<(String, String)>) -> Option<String> {
    let mut edges = edges.to_vec();

    let mut result = String::new();
    let mut start = edges.iter().cloned().map(|(a, _)| a).collect::<HashSet<_>>();
    let non_start = edges.iter().cloned().map(|(_, b)| b).collect::<HashSet<_>>();
    start = start.difference(&non_start).cloned().collect();

    while !start.is_empty() {
        let node = start.iter().cloned().min().unwrap();
        start.remove(&node);
        result.push_str(&node);

        let removed = edges.iter().cloned().filter(|(a, _)| *a == node).collect::<HashSet<_>>();
        edges = edges.iter().cloned().filter(|t| !removed.contains(&t)).collect();
        let mut candidates = removed.iter().cloned().map(|(_, b)| b).collect::<HashSet<_>>();
        for (_, b) in &edges { candidates.remove(b); }
        start = start.union(&candidates).cloned().collect();
    }
    if !edges.is_empty() { return None }

    Some(result)
}


fn worker_sort(edges: &[(String, String)]) -> Option<String> {
    let mut edges = edges.to_vec();

    let mut result = String::new();
    let mut start = edges.iter().cloned().map(|(a, _)| a).collect::<HashSet<_>>();
    let non_start = edges.iter().cloned().map(|(_, b)| b).collect::<HashSet<_>>();
    start = start.difference(&non_start).cloned().collect();

    let mut workers: Vec<(u32, String)> = Vec::new();
    let mut time = 0;

    while !start.is_empty() || !workers.is_empty() {
        if workers.len() == 5 || start.is_empty() {
            // complete a piece of work, advance the time

            workers.sort();
            let (t, node) = workers.remove(0);
            time = t;
            result.push_str(&node);

            let removed = edges.iter().cloned().filter(|(a, _)| *a == node).collect::<HashSet<_>>();
            edges = edges.iter().cloned().filter(|t| !removed.contains(&t)).collect();
            let mut candidates = removed.iter().cloned().map(|(_, b)| b).collect::<HashSet<_>>();
            for (_, b) in &edges { candidates.remove(b); }
            start = start.union(&candidates).cloned().collect();
        } else {
            // start a piece of work

            let node = start.iter().cloned().min().unwrap();
            let t = node.chars().last().unwrap() as u32 - 4;
            start.remove(&node);
            workers.push((time + t, node));
        }
    }
    if !edges.is_empty() { return None }

    println!("time = {}", time);

    Some(result)
}


fn main() {
    let instructions = read_instructions();
    let sorted = worker_sort(&instructions).unwrap();

    println!("ordering = {}", sorted);
}
