use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use counter::Counter;


fn parse_name(desc: &str) -> String {
    let mut parts = desc.split(' ');
    parts.next();
    parts.next().unwrap().to_string()
}


fn parse_quant(desc: &str) -> u32 {
    let mut parts = desc.split(' ');
    parts.next().unwrap().parse().unwrap()
}


fn read() -> HashMap<String, (u32, HashMap<String, u32>)> {
    let stdin = io::stdin();
    stdin.lock().lines().map(|x| {
        let equation = x.unwrap().split("=>").map(|s| s.trim().to_string()).collect::<Vec<_>>();
        let inputs = equation[0].split(", ")
            .map(|s| (parse_name(&s), parse_quant(&s))).collect();

        (parse_name(&equation[1]), (parse_quant(&equation[1]), inputs))
    }).collect()
}


fn main() {
    let mut data = read();

    let (_, mut inputs) = data.remove("FUEL").unwrap();
    while inputs.len() > 1 {
        println!("inputs: {:?}", inputs);

        let counts = data.iter()
            .map(|(_, (_, si))| si.keys())
            .flatten()
            .collect::<Counter<_>>();

        println!("counts: {:?}", counts.most_common());

        let key = inputs.keys().cloned()
            .filter(|k| k != "ORE")
            .min_by_key(|k| counts.get(k).unwrap_or(&0)).unwrap();
        let value = inputs.remove(&key).unwrap();
        let (quant, sub_inputs) = data.remove(&key).unwrap();
        let (mut multiplier, remainder) = (value / quant, value % quant);
        if remainder > 0 { multiplier += 1; }

        for (k_s, v_s) in sub_inputs.iter() {
            *inputs.entry(k_s.to_string()).or_insert(0) += multiplier * v_s;
        }
    }

    println!("{:?}", inputs);
}
