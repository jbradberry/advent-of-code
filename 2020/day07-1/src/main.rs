use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use regex::Regex;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Carry {
    Allowed,
    Forbidden,
    Unresolved,
}


fn read() -> HashMap<String, Vec<String>> {
    let rule_re = Regex::new("^(.*) bags contain (.*)$").unwrap();
    let bag_re = Regex::new(r"(\d+|no) ((?:\w+ \w+)|other)").unwrap();
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|r| {
            let line = r.unwrap();
            let cap = rule_re.captures(&line).unwrap();
            (
                cap[1].to_string(),
                bag_re.find_iter(&cap[2])
                    .map(|m| bag_re.captures(m.as_str()).unwrap())
                    .filter_map(|c| match (&c[1], &c[2]) {
                        ("no", _) => None,
                        (_, color) => Some(color.to_string()),
                    })
                    .collect()
            )
        })
        .collect()
}


fn main() {
    let rules = read();

    let mut allowance: HashMap<String, Carry> = rules.keys()
        .map(|k| (k.to_string(), Carry::Unresolved))
        .collect();
    allowance.insert("shiny gold".to_string(), Carry::Allowed);

    loop {
        for (k, r) in &rules {
            if allowance[k] != Carry::Unresolved { continue; }

            let resolution = r.iter()
                .map(|c| allowance[c])
                .fold(Carry::Forbidden, |acc, x| match (acc, x) {
                    (Carry::Forbidden, Carry::Forbidden) => Carry::Forbidden,
                    (Carry::Allowed, _) => Carry::Allowed,
                    (_, Carry::Allowed) => Carry::Allowed,
                    (_, _) => Carry::Unresolved,
                });

            allowance.insert(k.to_string(), resolution);
        }

        if !allowance.values().any(|v| *v == Carry::Unresolved) { break; }
    }
    allowance.remove("shiny gold");

    println!("{} bag colors can carry shiny gold bags.",
             allowance.values().filter(|&v| *v == Carry::Allowed).count());
}
