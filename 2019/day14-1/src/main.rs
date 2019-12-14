use std::collections::HashMap;
use std::io;
use std::io::prelude::*;


fn parse_name(desc: &str) -> String {
    let mut parts = desc.split(' ');
    parts.next();
    parts.next().unwrap().to_string()
}


fn parse_quant(desc: &str) -> i32 {
    let mut parts = desc.split(' ');
    parts.next().unwrap().parse().unwrap()
}


fn read() -> Vec<HashMap<String, i32>>{
    let stdin = io::stdin();
    stdin.lock().lines().map(|x| {
        let equation = x.unwrap().split("=>").map(|s| s.trim().to_string()).collect::<Vec<_>>();
        let mut row = equation[0].split(", ")
            .map(|s| (parse_name(&s), parse_quant(&s))).collect::<HashMap<String, i32>>();
        row.insert(parse_name(&equation[1]), -parse_quant(&equation[1]));

        row
    }).collect()
}


fn main() {
    let mut data = read();
}
