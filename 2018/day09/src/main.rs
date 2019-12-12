use std::io;
use std::io::prelude::*;

use regex::Regex;


fn read_game() -> (usize, u32) {
    let re = Regex::new(r"^(\d+) .* (\d+)").unwrap();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let cap = re.captures(&buffer).unwrap();
    (cap.get(1).unwrap().as_str().parse().unwrap(), cap.get(2).unwrap().as_str().parse().unwrap())
}


fn main() {
    let (players, marble) = read_game();
    let last_marble = 100 * marble;

    let mut player_scores: Vec<u32> = (0..players).map(|_| 0).collect();
    let mut circle: Vec<u32> = Vec::with_capacity((last_marble + 1) as usize);

    let mut next_marble = 0;
    let mut index = 0;
    'outer: loop {
        'inner: for player in 0..players {
            if next_marble == 0 {
                circle.push(next_marble);
            } else if next_marble % 23 == 0 {
                index = (index - 7) % circle.len();
                player_scores[player] += next_marble + circle.remove(index);
            } else {
                index += 2;
                if index > circle.len() { index %= circle.len(); }
                circle.insert(index, next_marble);
            }
            next_marble += 1;
            if next_marble % 10000 == 0 { println!("{}", next_marble); }
            if next_marble > last_marble { break 'outer; }
        }
    }

    println!("high score = {}", player_scores.iter().max().unwrap());
}
