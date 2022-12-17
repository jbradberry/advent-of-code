use std::io;
use std::io::prelude::*;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;


fn read() -> (HashMap<usize, (usize, Vec<usize>)>, usize) {
    let re = Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([\w, ]+)$").unwrap();
    let stdin = io::stdin();

    let holding = stdin.lock().lines()
        .map(|l| {
            let line = l.unwrap();
            let cap = re.captures(&line).unwrap();
            (cap.get(1).unwrap().as_str().to_string(),
             cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
             cap.get(3).unwrap().as_str().to_string())
        })
        .collect::<Vec<_>>();
    let mapping = holding.iter()
        .enumerate()
        .map(|(i, (v, _, _))| (v.to_string(), i))
        .collect::<HashMap<_, _>>();

    (
        holding.iter()
            .map(|(v, r, t)| {
                (*mapping.get(&v.to_string()).unwrap(),
                 (*r, t.split(", ").map(|s| *mapping.get(s).unwrap()).collect::<Vec<_>>()))
            })
            .collect(),
        *mapping.get("AA").unwrap()
    )
}


fn main() {
    let (valves, start) = read();

    // println!("valves: {:?}", valves);

    let mut cross = valves.iter()
        .flat_map(|(k, (_, t))| t.iter().map(|v| ((*k, *v), 1)))
        .collect::<HashMap<(usize, usize), usize>>();

    // println!("cross: {:?}", cross);

    loop {
        let new = cross.iter()
            .permutations(2)
            .filter(|x| (x[0].0.1 == x[1].0.0) && (x[0].0.0 != x[1].0.1))
            .map(|x| (x[0].0.0, x[1].0.1, x[0].1 + x[1].1))
            .filter(|(a, b, d)| {
                let x = cross.get(&(*a, *b));
                x.is_none() || d < x.unwrap()
            })
            .collect::<HashSet<_>>();

        // println!("new: {:?}", new);

        if new.is_empty() { break; }

        for (a, b, d) in new.into_iter() {
            cross.entry((a, b))
                .and_modify(|e| { if d < *e { *e = d; }})
                .or_insert(d);
        }
    }

    let mut useful = valves.iter()
        .filter(|(_, v)| v.0 != 0)
        .map(|(k, v)| (*k, v.0))
        .collect::<Vec<(usize, usize)>>();
    useful.sort_by_key(|&(_, v)| v);
    useful.reverse();

    println!("useful: {:?}", useful);

    let mut best = 0;
    for p in useful.iter().permutations(8) {
        let mut location = start;
        let mut total = 0;
        let mut tick = 0;

        // println!("{:?}", p);

        for (i, (v, r)) in p.iter().enumerate() {
            let d = cross.get(&(location, *v)).unwrap();

            if *d < (30 - tick - 1) {
                // println!("{} gets {} ticks", v, 30 - tick - d - 1);
                total += (30 - tick - d - 1) * r;
            }
            tick += d + 1;
            if tick >= 30 { break; }

            // if there is no way we can catch up with what is left, skip the rest
            if total < best && (best - total) > (30 - tick - 1) * (p[(i + 1)..].iter().map(|(_, x)| x).sum::<usize>()) { break; }
            location = *v;
        }

        if total > best {
            best = total;
            println!("{:?}, {}", p, total);
        }
    }
}
