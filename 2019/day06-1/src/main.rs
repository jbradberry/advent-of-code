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


fn main() {
    let orbits = read();
}
