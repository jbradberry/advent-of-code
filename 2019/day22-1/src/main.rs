use std::io;
use std::io::prelude::*;


#[derive(Debug)]
enum Action {
    NewStack,
    Deal(u32),
    Cut(i32),
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
