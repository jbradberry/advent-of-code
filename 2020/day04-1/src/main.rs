use std::io;
use std::io::prelude::*;

use std::collections::HashMap;


fn read() -> Vec<HashMap<String, String>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.split("\n\n")
        .map(|s|
            s.split_whitespace()
                .map(|item| {
                    let mut kv_iter = item.split(":");
                    (kv_iter.next().unwrap().to_string(),
                     kv_iter.next().unwrap().to_string())
                })
                .collect()
        )
        .collect()
}


fn main() {
    let passports = read();

    let valid = passports.iter()
        .filter(|p| {
            p.contains_key("byr") &&
                p.contains_key("iyr") &&
                p.contains_key("eyr") &&
                p.contains_key("hgt") &&
                p.contains_key("hcl") &&
                p.contains_key("ecl") &&
                p.contains_key("pid")
        })
        .count();

    println!("{} valid passports.", valid);
}
