use std::io;
use std::io::prelude::*;

use std::cmp;
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


fn unmask(items: &Vec<(usize, usize)>, mask: u64) -> Vec<(usize, usize, usize)> {
    items.iter()
        .enumerate()
        .map(|(i, (a, b))| (i, *a, *b))
        .filter(|(i, _, _)| (mask & (1 << i)) == 0)
        .collect()
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

    let useful = valves.iter()
        .filter(|(_, v)| v.0 != 0)
        .map(|(k, v)| (*k, v.0))
        .collect::<Vec<(usize, usize)>>();

    // println!("useful: {:?}", useful);

    let mut best = 0;
    let mut current = HashSet::from([(0, 26, 26, start, start, 0)]);
    loop {
        let mut new = HashSet::new();
        // println!("current: {:?}", current);

        for (total, rem1, rem2, loc1, loc2, mask) in current {
            let tmp = unmask(&useful, mask);
            for &(i, valve, rate) in &tmp {
                // consider me
                let d = cross.get(&(loc1, valve)).unwrap();
                if d + 1 <= rem1 {
                    let new_total = total + rate * (rem1 - d - 1);
                    if new_total > best {
                        best = new_total;
                        println!("update: {}", best);
                    }
                    let new_rem = rem1 - d - 1;
                    let max_rem = cmp::max(new_rem, rem2);
                    if tmp.iter()
                        .filter(|(i2, _, _)| *i2 != i)
                        .any(|(_, v, _)| cross.get(&(valve, *v)).unwrap() < &max_rem) {
                        new.insert(
                            (new_total, new_rem, rem2, valve, loc2, mask | (1 << i))
                        );
                    }
                }

                // consider the elephant
                let d = cross.get(&(loc2, valve)).unwrap();
                if d + 1 <= rem2 {
                    let new_total = total + rate * (rem2 - d - 1);
                    if new_total > best {
                        best = new_total;
                        println!("update: {}", best);
                    }
                    let new_rem = rem2 - d - 1;
                    let max_rem = cmp::max(rem1, new_rem);
                    if tmp.iter()
                        .filter(|(i2, _, _)| *i2 != i)
                        .any(|(_, v, _)| cross.get(&(valve, *v)).unwrap() < &max_rem) {
                        new.insert(
                            (new_total, rem1, new_rem, loc1, valve, mask | (1 << i))
                        );
                    }
                }
            }
        }
        if new.is_empty() { break; }
        current = new;
    }

    println!("best pressure: {}", best);
}
