use std::io;
use std::io::prelude::*;

use regex::Regex;


fn read() -> (Vec<Vec<char>>, Vec<(u16, u16, u16)>) {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let stdin = io::stdin();

    let lines = stdin.lock().lines().map(|s| s.unwrap()).collect::<Vec<_>>();
    let mut sections = lines.split(|s| s == "");

    let mut diagram = sections.next().unwrap().iter().collect::<Vec<_>>();
    diagram.reverse();

    let mut stacks = diagram[0]
        .split(" ")
        .filter(|s| *s != "")
        .map(|_| Vec::new())
        .collect::<Vec<_>>();
    for line in diagram {
        for (i, c) in line.chars().enumerate() {
            match c {
                'A'..='Z' => { stacks[((i - 1) / 4) as usize].push(c); },
                _ => { continue; }
            }
        }
    }

    let instructions = sections.next().unwrap()
        .iter()
        .map(|line| {
            let cap = re.captures(&line).unwrap();

            (cap.get(1).unwrap().as_str().parse().unwrap(),
             cap.get(2).unwrap().as_str().parse().unwrap(),
             cap.get(3).unwrap().as_str().parse().unwrap())
        })
        .collect();

    (stacks, instructions)
}


fn action(stacks: &mut Vec<Vec<char>>, action: &(u16, u16, u16)) {
    for _ in 0..action.0 {
        let c = stacks[(action.1 - 1) as usize].pop().unwrap();
        stacks[(action.2 - 1) as usize].push(c);
    }
}


fn main() {
    let (mut stacks, instructions) = read();
    println!("instructions: {:?}", instructions);
    println!("stacks before: {:?}", stacks);

    for inst in &instructions {
        action(&mut stacks, inst);
    }

    println!("stacks after: {:?}", stacks);

    let result = stacks.iter().map(|x| x.last().unwrap()).collect::<String>();
    println!("result: {}", result);
}
