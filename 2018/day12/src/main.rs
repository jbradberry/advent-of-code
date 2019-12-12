use std::collections::{HashMap, HashSet};

use std::io;
use std::io::prelude::*;


fn plant(c: char) -> bool {
    match c {
        '#' => true,
        _ => false,
    }
}


fn parse_pots(line: &str) -> HashSet<i64> {
    let chars = line.chars().collect::<Vec<_>>();

    chars[15..].iter().enumerate().filter_map(|(i, &c)| match c { '#' => Some(i as i64), _ => None }).collect()
}


fn parse_rules(lines: &[String]) -> HashMap<Vec<bool>, bool> {
    let mut rules = HashMap::new();

    for line in lines {
        let input = line.chars().take(5).map(|c| plant(c)).collect::<Vec<_>>();
        let output = plant(line.chars().last().unwrap());

        rules.insert(input, output);
    }

    rules
}


fn read() -> (HashSet<i64>, HashMap<Vec<bool>, bool>) {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    (parse_pots(&lines[0]), parse_rules(&lines[2..]))
}


fn format_pots(pots: &HashSet<i64>) -> String {
    let mut plants = String::new();
    let min = pots.iter().min().unwrap();
    let max = pots.iter().max().unwrap();

    for x in *min..(*max + 1) {
        plants.push(match pots.contains(&x) { true => '#', false => '.' });
    }

    plants
}


fn main() {
    let (original_pots, rules) = read();

    let mut pots: HashSet<i64> = original_pots.iter().cloned().collect();

    for generation in 0..50000000000u64 {
        // println!("{}", format_pots(&pots));

        let min = pots.iter().min().unwrap();
        let max = pots.iter().max().unwrap();

        let new = ((*min - 2)..(*max + 3)).filter(|x| {
            let v = ((x - 2)..(x + 3)).map(|a| pots.contains(&a)).collect::<Vec<_>>();

            *rules.get(&v).unwrap()
        }).collect::<HashSet<_>>();

        if new == pots { break }
        if generation % 1000000 == 0 {
            println!("{}: sum = {}", generation, pots.iter().sum::<i64>());
        }
        pots = new;
    }

    // println!("{}", format_pots(&pots));
    println!("sum = {}", pots.iter().sum::<i64>());
}
