use std::io;
use std::io::prelude::*;

use std::collections::HashMap;
use std::ops::RangeInclusive;

use regex::Regex;


fn read() -> (HashMap<String, Vec<RangeInclusive<u16>>>, Vec<Vec<u16>>) {
    let field_re = Regex::new(r"^([[:alpha:] ]+): (\d+-\d+(?: or \d+-\d+)*)$").unwrap();
    let mut constraints = HashMap::new();
    let mut tickets = Vec::new();

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        match field_re.captures(&line) {
            Some(c) => {
                constraints.insert(
                    c[1].to_string(),
                    c[2].split(" or ")
                        .map(|s| {
                            let ends: Vec<_> = s.split("-").collect();
                            RangeInclusive::new(ends[0].parse().unwrap(), ends[1].parse().unwrap())
                        })
                        .collect()
                );
            },
            None => {
                match line.as_str() {
                    "" | "your ticket:" | "nearby tickets:" => { continue; },
                    _ => { tickets.push(line.split(",").map(|s| s.parse().unwrap()).collect()); },
                }
            },
        }
    }

    (constraints, tickets)
}


fn main() {
    let (constraints, tickets) = read();

    let invalid: Vec<u16> = tickets.into_iter()
        .flat_map(|t| {
            t.into_iter()
                .filter(|v| {
                    !constraints.values()
                        .any(|field| { field.iter().any(|r| r.contains(v)) })
                })
        })
        .collect();

    println!("scanning error rate: {}", invalid.iter().sum::<u16>());
}
