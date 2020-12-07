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


fn main() {
    let rules = read();

    println!("{:?}", rules);
}
