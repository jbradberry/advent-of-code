use std::io;
use std::io::prelude::*;

use regex::Regex;


#[derive(Debug, Default)]
struct Monkey {
    items: Vec<u32>,
    operation: (String, String, String),
    test: u32,
    test_true: usize,
    test_false: usize,
}


fn read() -> Vec<Monkey> {
    let re_start = Regex::new(r"Starting items: (\d+(?:, \d+)*)$").unwrap();
    let re_operation = Regex::new(r"Operation: new = (old|\d+) ([+*]) (old|\d+)$").unwrap();
    let re_test = Regex::new(r"Test: divisible by (\d+)$").unwrap();
    let re_true = Regex::new(r"If true: throw to monkey (\d+)$").unwrap();
    let re_false = Regex::new(r"If false: throw to monkey (\d+)$").unwrap();

    let mut results = Vec::new();
    let mut monkey: Monkey = Default::default();

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line == "" {
            results.push(monkey);
            monkey = Default::default();
            continue;
        }

        if re_start.is_match(&line) {
            monkey.items = re_start.captures(&line).unwrap()[1]
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect();
        }
        if re_operation.is_match(&line) {
            let cap = re_operation.captures(&line).unwrap();
            monkey.operation = (cap[1].to_string(), cap[2].to_string(), cap[3].to_string());
        }
        if re_test.is_match(&line) {
            monkey.test = re_test.captures(&line).unwrap()[1].parse().unwrap();
        }
        if re_true.is_match(&line) {
            monkey.test_true = re_true.captures(&line).unwrap()[1].parse().unwrap();
        }
        if re_false.is_match(&line) {
            monkey.test_false = re_false.captures(&line).unwrap()[1].parse().unwrap();
        }
    }
    results.push(monkey);

    results
}


fn main() {
    let mut monkeys = read();

    // println!("monkeys: {:#?}", monkeys);

    let mut inspections = vec![0; monkeys.len()];
    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let mut throws = Vec::new();
            {
                let current_monkey = &monkeys[i];
                for item in &current_monkey.items {
                    let op = match current_monkey.operation.1.as_str() {
                        "+" => u32::overflowing_add,
                        "*" => u32::overflowing_mul,
                        _ => unreachable!(),
                    };
                    let a = match current_monkey.operation.0.as_str() {
                        "old" => *item,
                        x @ _ => x.parse().unwrap(),
                    };
                    let b = match current_monkey.operation.2.as_str() {
                        "old" => *item,
                        x @ _ => x.parse().unwrap(),
                    };

                    let (mut new_level, _) = op(a, b);
                    new_level = new_level.div_euclid(3);

                    let new_monkey = match new_level % current_monkey.test == 0 {
                        true => current_monkey.test_true,
                        false => current_monkey.test_false,
                    };
                    throws.push((new_monkey, new_level));
                }
            }
            inspections[i] += monkeys[i].items.len();
            monkeys[i].items.clear();
            for (i2, item) in throws.into_iter() {
                monkeys[i2].items.push(item);
            }
        }

        // for monkey in &monkeys {
        //     println!("{:?}", monkey.items);
        // }
        // println!("");
    }

    for (i, insp) in inspections.iter().enumerate() {
        println!("Monkey {} inspected items {} times.", i, insp);
    }
    println!("");
    inspections.sort();
    inspections.reverse();

    println!("monkey business: {}", inspections[0] * inspections[1]);
}
