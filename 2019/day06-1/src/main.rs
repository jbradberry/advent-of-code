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


fn count_orbits(orbitals: &HashMap<String, String>, object: &str) -> u32 {
    if orbitals.contains_key(object) { count_orbits(orbitals, orbitals.get(object).unwrap()) + 1 }
    else { 0 }
}


fn main() {
    let orbitals = read();
    let mut orbits = 0;

    for object in orbitals.keys() {
        orbits += count_orbits(&orbitals, object);
    }

    println!("orbits: {}", orbits);
}
