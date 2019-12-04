use std::io;
use std::io::prelude::*;
use std::ops::RangeInclusive;

use itertools::Itertools;


fn read() -> RangeInclusive<u32> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let items = buffer.trim().split('-').map(|x| x.parse().unwrap()).collect::<Vec<_>>();
    RangeInclusive::new(items[0], items[1])
}


fn main() {
    let range = read();
    let mut count: u16 = 0;

    for digits in (1..=9).combinations_with_replacement(6) {
        let combo = digits.iter().fold(0, |acc, d| 10 * acc + d);
        if digits.into_iter().dedup().count() < 6 && range.contains(&combo) {
            // println!("combo: {}", combo);
            count += 1;
        }
    }

    println!("number of valid combinations: {}", count);
}
