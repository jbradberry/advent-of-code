use std::collections::HashMap;
use std::io;
use std::io::prelude::*;


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

    let mut final_substitution = false;
    let (_, mut inputs) = data.remove("FUEL").unwrap();
    while inputs.len() > 1 {
        println!("{:?}", inputs);

        let mut substitution = false;
        let mut new_inputs = HashMap::new();
        for (k, v) in inputs.drain() {
            if k == "ORE" {
                *new_inputs.entry(k).or_insert(0) += v;
                continue;
            }

            let (quant, sub_inputs) = data.get(&k).unwrap();
            let (mut multiplier, remainder) = (v / quant, v % quant);
            if remainder > 0 {
                if !final_substitution && sub_inputs.contains_key("ORE") {
                    *new_inputs.entry(k).or_insert(0) += v;
                    continue;
                }
                multiplier += 1;
            }
            for (k_s, v_s) in sub_inputs.iter() {
                *new_inputs.entry(k_s.to_string()).or_insert(0) += multiplier * v_s;
                substitution = true;
            }
        }
        inputs = new_inputs;
        final_substitution = !substitution;
    }

    println!("{:?}", inputs);
}
