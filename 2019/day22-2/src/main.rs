use std::io;
use std::io::prelude::*;


#[derive(Debug)]
enum Action {
    NewStack,
    Deal(usize),
    Cut(i32),
}


impl Action {
    fn shuffle(&self, deck: &mut Vec<u32>) {
        match self {
            Action::NewStack => { deck.reverse(); },
            Action::Cut(x) => {
                if *x >= 0 { deck.rotate_left(*x as usize); }
                else { deck.rotate_right(-*x as usize); }
            },
            Action::Deal(n) => {
                let len = deck.len();
                let mut new_deck = Vec::new();
                new_deck.resize(len, None);

                for (i, c) in deck.iter().enumerate() { new_deck[(i * n) % len] = Some(*c); }
                for (i, c) in new_deck.iter().enumerate() { deck[i] = c.unwrap(); }
            },
        }
    }
}


fn read() -> Vec<Action> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .filter_map(|line| {
            let s = line.unwrap();
            if s.starts_with("deal into new") { Some(Action::NewStack) }
            else {
                let num = s.split_whitespace().last().unwrap();
                if s.starts_with("deal with") { Some(Action::Deal(num.parse().unwrap())) }
                else if s.starts_with("cut") { Some(Action::Cut(num.parse().unwrap())) }
                else { None }
            }
        })
        .collect()
}


fn main() {
    let actions = read();
    let mut deck = (0..119_315_717_514_047).collect::<Vec<u32>>();

    for action in actions {
        action.shuffle(&mut deck);
    }

    let position = deck.iter().position(|&x| x == 2019).unwrap();
    println!("position of card 2019: {}", position);
}
