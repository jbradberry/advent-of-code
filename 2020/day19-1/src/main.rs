use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;

use itertools::Itertools;
use regex::Regex;


fn read() -> (HashMap<u16, String>, Vec<String>) {
    let character_re = Regex::new(r"^(\d+): \x22([a-z]+)\x22$").unwrap();
    let subrule_re = Regex::new(r"^(\d+): (.*)$").unwrap();

    let mut rules = HashMap::new();
    let mut messages = Vec::new();

    let stdin = io::stdin();
    for s in stdin.lock().lines() {
        let line = s.unwrap();

        if line == "" { continue; }

        if character_re.is_match(&line) {
            let caps = character_re.captures(&line).unwrap();
            rules.insert(caps[1].parse().unwrap(), caps[2].to_string());
        } else if subrule_re.is_match(&line) {
            let caps = subrule_re.captures(&line).unwrap();
            rules.insert(caps[1].parse().unwrap(), caps[2].to_string());
        } else {
            messages.push(line);
        }
    }

    (rules, messages)
}


fn resolve_rule(rules: &HashMap<u16, String>, index: u16) -> Option<Vec<String>> {
    let rule = rules.get(&index);

    match rule {
        None => None,
        Some(r) => {
            match r.chars().next().unwrap() {
                'a'..='z' => { Some(vec![r.to_string()]) },
                '0'..='9' => {
                    let results = r.split(" | ")
                        .flat_map(|opt| {
                            opt.split_whitespace()
                                .map(|x| resolve_rule(rules, x.parse().unwrap()).unwrap())
                                .multi_cartesian_product()
                                .map(|v| v.iter().join(""))
                        })
                        .collect();
                    Some(results)
                },
                _ => panic!(),
            }
        }
    }
}


fn main() {
    let (rules, messages) = read();

    let zero = resolve_rule(&rules, 0).unwrap().into_iter().collect::<HashSet<String>>();
    let count = messages.iter()
        .filter(|m| zero.contains(m.as_str()))
        .count();

    println!("valid messages: {}", count);
}
