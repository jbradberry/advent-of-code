use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

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


fn main() {
    let (rules, messages) = read();

    println!("rules: {:?}", rules);
    println!("messages: {:?}", messages);
}
