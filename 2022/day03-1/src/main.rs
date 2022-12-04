use std::collections::HashSet;
use std::io;
use std::io::prelude::*;


fn read() -> Vec<Vec<String>> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|line| line.unwrap())
        .map(|s| {
            let (x, y) = s.split_at(s.len() / 2);
            vec![x.to_string(), y.to_string()]
        })
        .collect()
}


fn priority(c: char) -> u32 {
    match c {
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        _ => 0
    }
}


fn double_packed(rucksack: &Vec<String>) -> HashSet<char> {
    let (a, b) = (rucksack[0].chars().collect::<HashSet<char>>(), rucksack[1].chars().collect::<HashSet<char>>());

    a.intersection(&b).cloned().collect()
}


fn main() {
    let backpacks = read();

    let double_items = backpacks.iter().map(|b| double_packed(b)).collect::<Vec<HashSet<char>>>();
    let total_priority = double_items.iter().map(|s| s.iter().cloned().map(priority).sum::<u32>()).sum::<u32>();
    println!("total priority: {}", total_priority);
}
