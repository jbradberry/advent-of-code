use std::io;
use std::io::prelude::*;

use std::collections::HashMap;


fn read() -> HashMap<String, String> {
    let stdin = io::stdin();
    stdin.lock().lines().map(|x| {
        let line = x.unwrap();
        let bodies = line.split(')').collect::<Vec<_>>();
        (bodies[1].to_string(), bodies[0].to_string())
    }).collect()
}


fn parent_objects(orbitals: &HashMap<String, String>, object: &str) -> Vec<String> {
    let mut parents = Vec::new();
    let mut current = object;

    while orbitals.contains_key(current) {
        current = orbitals.get(current).unwrap();
        parents.push(current.to_string());
    }

    parents
}


fn main() {
    let orbitals = read();

    let my_parents = parent_objects(&orbitals, "YOU");
    let santa_parents = parent_objects(&orbitals, "SAN");

    let up = my_parents.iter().fold(0, |acc, x| { if !santa_parents.contains(x) { acc + 1 } else { acc } });
    let down = santa_parents.iter().fold(0, |acc, x| { if !my_parents.contains(x) { acc + 1 } else { acc } });

    println!("transfers needed: {}", up + down);
}
