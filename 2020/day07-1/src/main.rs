use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use regex::Regex;


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


fn resolve(rules: &HashMap<String, Vec<String>>, key: &str) -> bool {
    if key == "shiny gold" { return true; }

    rules[key].iter().any(|v| resolve(rules, v))
}


fn main() {
    let rules = read();
    let allowed = rules.keys().filter(|k| k.as_str() != "shiny gold" && resolve(&rules, k)).count();

    println!("{} bag colors can carry shiny gold bags.", allowed);
}
