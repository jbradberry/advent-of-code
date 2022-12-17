use std::io;
use std::io::prelude::*;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;


// fn read() -> HashMap<String, (u32, Vec<String>)> {
//     let re = Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([\w, ]+)$").unwrap();
//     let stdin = io::stdin();

//     stdin.lock().lines()
//         .map(|l| {
//             let line = l.unwrap();
//             let cap = re.captures(&line).unwrap();

//             (cap.get(1).unwrap().as_str().to_string(),
//              (cap.get(2).unwrap().as_str().parse::<u32>().unwrap(),
//               cap.get(3).unwrap().as_str().split(", ").map(|s| s.to_string()).collect::<Vec<_>>()))
//         })
//         .collect()
// }


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
    // let valves = read();

    let (valves, start) = read();

    // println!("valves: {:?}", valves);

    // let mut cross = valves.iter()
    //     .flat_map(|(k, (_, t))| t.iter().map(|v| ((k.to_string(), v.to_string()), 1)))
    //     .collect::<HashMap<(String, String), usize>>();

    let mut cross = valves.iter()
        .flat_map(|(k, (_, t))| t.iter().map(|v| ((*k, *v), 1)))
        .collect::<HashMap<(usize, usize), usize>>();

    // println!("cross: {:?}", cross);

    loop {
        // let new = cross.iter()
        //     .permutations(2)
        //     .filter(|x| (x[0].0.1 == x[1].0.0) && (x[0].0.0 != x[1].0.1))
        //     .map(|x| (x[0].0.0.to_string(), x[1].0.1.to_string(), x[0].1 + x[1].1))
        //     .filter(|(a, b, d)| {
        //         let x = cross.get(&(a.to_string(), b.to_string()));
        //         x.is_none() || d < x.unwrap()
        //     })
        //     .collect::<HashSet<_>>();

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

    // println!("cross.len(): {}", cross.len());
    // println!("cross:");
    // for (k, v) in cross.iter() {
    //     println!("{}: {:?}", v, k);
    // }
    // println!();

    let useful = valves.iter()
        .filter(|(_, v)| v.0 != 0)
        .map(|(k, _)| *k)
        .collect::<Vec<usize>>();

    println!("useful: {:?}", useful);

    let mut best = 0;
    for p in useful.iter().permutations(useful.len()) {
        // let mut location = "AA".to_string();

        let mut location = start;
        let mut total = 0;
        let mut tick = 0;

        // println!("{:?}", p);

        for v in &p {
            // let d = cross.get(&(location.to_string(), v.to_string())).unwrap();

            let d = cross.get(&(location, **v)).unwrap();

            if *d < (30 - tick - 1) {
                // println!("{} gets {} ticks", v, 30 - tick - d - 1);
                total += (30 - tick - d - 1) * valves.get(&v).unwrap().0 as usize;
            }
            tick += d + 1;
            if tick >= 30 { break; }
            location = **v;
        }

        if total > best {
            best = total;
            println!("{:?}, {}", p, total);
        }
    }
}
