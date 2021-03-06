use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use counter::Counter;


fn parse_name(desc: &str) -> String {
    let mut parts = desc.split(' ');
    parts.next();
    parts.next().unwrap().to_string()
}


fn parse_quant(desc: &str) -> u64 {
    let mut parts = desc.split(' ');
    parts.next().unwrap().parse().unwrap()
}


fn read() -> HashMap<String, (u64, HashMap<String, u64>)> {
    let stdin = io::stdin();
    stdin.lock().lines().map(|x| {
        let equation = x.unwrap().split("=>").map(|s| s.trim().to_string()).collect::<Vec<_>>();
        let inputs = equation[0].split(", ")
            .map(|s| (parse_name(&s), parse_quant(&s))).collect();

        (parse_name(&equation[1]), (parse_quant(&equation[1]), inputs))
    }).collect()
}


fn solve(data: &HashMap<String, (u64, HashMap<String, u64>)>, fuel: u64) -> u64 {
    let mut data_copy = data.clone();

    let (_, mut inputs) = data_copy.remove("FUEL").unwrap();
    for (_, value) in inputs.iter_mut() {
        *value *= fuel;
    }

    while inputs.len() > 1 {
        let counts = data_copy.iter()
            .map(|(_, (_, si))| si.keys())
            .flatten()
            .collect::<Counter<_>>();

        let key = inputs.keys().cloned()
            .filter(|k| k != "ORE")
            .min_by_key(|k| counts.get(k).unwrap_or(&0)).unwrap();
        let value = inputs.remove(&key).unwrap();
        let (quant, sub_inputs) = data_copy.remove(&key).unwrap();
        let (mut multiplier, remainder) = (value / quant, value % quant);
        if remainder > 0 { multiplier += 1; }

        for (k_s, v_s) in sub_inputs.iter() {
            *inputs.entry(k_s.to_string()).or_insert(0) += multiplier * v_s;
        }
    }

    *inputs.get("ORE").unwrap()
}


fn main() {
    let data = read();
    let ore = solve(&data, 1);

    println!("Part 1");
    println!("ore: {}", ore);

    let mut lower = 1_000_000_000_000 / ore;
    assert!(solve(&data, lower) < 1_000_000_000_000);
    let mut upper = 2 * lower;
    assert!(solve(&data, upper) > 1_000_000_000_000);

    let mut fuel = lower + (upper - lower) / 2;
    while fuel > lower {
        if solve(&data, fuel) <= 1_000_000_000_000 { lower = fuel; } else { upper = fuel; }
        fuel = lower + (upper - lower) / 2;
    }

    println!("Part 2");
    println!("1 trillion ORE => fuel: {}", fuel);
}
