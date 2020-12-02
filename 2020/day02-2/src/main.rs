use std::io;
use std::io::prelude::*;

use regex::Regex;


fn read() -> Vec<(usize, usize, String, String)> {
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|x| {
            let line = x.unwrap();
            let cap = re.captures(&line).unwrap();

            (cap.get(1).unwrap().as_str().parse().unwrap(),
             cap.get(2).unwrap().as_str().parse().unwrap(),
             cap.get(3).unwrap().as_str().to_string(),
             cap.get(4).unwrap().as_str().to_string())
        })
        .collect()
}


fn main() {
    let passwords = read();

    let valid = passwords.iter()
        .filter(|(lower, upper, c, password)| {
            let chars: Vec<_> = password.split("").collect();
            (chars[*lower] == c) != (chars[*upper] == c)
        })
        .count();

    println!("{} valid passwords", valid);
}
