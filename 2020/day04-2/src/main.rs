use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use regex::Regex;


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


fn validate_passport(passport: &HashMap<String, String>) -> bool {
    let byr_valid = passport.get("byr")
        .ok_or("failure")
        .and_then(|v| v.parse::<u32>().map_err(|_| "failure"))
        .and_then(|v| Ok(1920 <= v && v <= 2002))
        .unwrap_or(false);

    let iyr_valid = passport.get("iyr")
        .ok_or("failure")
        .and_then(|v| v.parse::<u32>().map_err(|_| "failure"))
        .and_then(|v| Ok(2010 <= v && v <= 2020))
        .unwrap_or(false);

    let eyr_valid = passport.get("eyr")
        .ok_or("failure")
        .and_then(|v| v.parse::<u32>().map_err(|_| "failure"))
        .and_then(|v| Ok(2020 <= v && v <= 2030))
        .unwrap_or(false);

    let hgt_re = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
    let hgt_valid = passport.get("hgt")
        .ok_or("failure")
        .and_then(|v| match hgt_re.captures(v) {
            None => Err("failure"),
            Some(x) => {
                let value = x[1].parse().map_err(|_| "failure");
                match &x[2] {
                    "cm" => value.and_then(|h| Ok(150 <= h && h <= 193)),
                    "in" => value.and_then(|h| Ok(59 <= h && h <= 76)),
                    _ => Err("failure")
                }
            }
        })
        .unwrap_or(false);

    let hcl_re = Regex::new("^#[0-9a-f]{6}$").unwrap();
    let hcl_valid = passport.get("hcl")
        .ok_or("failure")
        .and_then(|v| Ok(hcl_re.is_match(v)))
        .unwrap_or(false);

    let ecl_re = Regex::new("^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    let ecl_valid = passport.get("ecl")
        .ok_or("failure")
        .and_then(|v| Ok(ecl_re.is_match(v)))
        .unwrap_or(false);

    let pid_re = Regex::new(r"^\d{9}$").unwrap();
    let pid_valid = passport.get("pid")
        .ok_or("failure")
        .and_then(|v| Ok(pid_re.is_match(v)))
        .unwrap_or(false);

    byr_valid &&
        iyr_valid &&
        eyr_valid &&
        hgt_valid &&
        hcl_valid &&
        ecl_valid &&
        pid_valid
}


fn main() {
    let passports = read();

    let valid = passports.into_iter()
        .filter(validate_passport)
        .count();

    println!("{} valid passports.", valid);
}
