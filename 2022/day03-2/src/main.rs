use std::collections::HashSet;
use std::io;
use std::io::prelude::*;


fn read() -> Vec<String> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|line| line.unwrap())
        .collect()
}


fn priority(c: char) -> u32 {
    match c {
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        _ => 0
    }
}


fn main() {
    let backpacks = read();
    let mut total: u32 = 0;

    for g in backpacks.chunks_exact(3) {
        let g0 = g[0].chars().collect::<HashSet<_>>();
        let g1 = g[1].chars().collect::<HashSet<_>>();
        let g2 = g[2].chars().collect::<HashSet<_>>();

        let mut badges = g0.intersection(&g1).cloned().collect::<HashSet<_>>();
        badges = badges.intersection(&g2).cloned().collect();
        total += badges.iter().cloned().map(priority).sum::<u32>();
    }

    println!("total priority for badges: {}", total);
}
