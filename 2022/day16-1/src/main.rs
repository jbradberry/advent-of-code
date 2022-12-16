use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use regex::Regex;


fn read() -> HashMap<String, (u32, Vec<String>)> {
    let re = Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([\w, ]+)$").unwrap();
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|l| {
            let line = l.unwrap();
            let cap = re.captures(&line).unwrap();

            (cap.get(1).unwrap().as_str().to_string(),
             (cap.get(2).unwrap().as_str().parse::<u32>().unwrap(),
              cap.get(3).unwrap().as_str().split(", ").map(|s| s.to_string()).collect::<Vec<_>>()))
        })
        .collect()
}


fn main() {
    let valves = read();

    println!("valves: {:?}", valves);
}
