use std::io;
use std::io::prelude::*;


#[derive(Debug)]
enum Action {
    NewStack,
    Deal(u32),
    Cut(i32),
}


impl Action {
    fn shuffle(&self, deck: &mut Vec<u16>) {
        match self {
            Action::NewStack => { deck.reverse(); },
            Action::Cut(x) => {
                if *x >= 0 { deck.rotate_left(*x as usize); }
                else { deck.rotate_right(-*x as usize); }
            },
            Action::Deal(n) => {
                let mut i = 0;
                let len = deck.len();
                deck.sort_by_key(|c| { let a = i; i += 1; a % len })
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

    println!("{:?}", actions);
}
