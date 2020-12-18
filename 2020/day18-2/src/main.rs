use std::io;
use std::io::prelude::*;

use itertools::chain;


#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    Add,
    Mul,
    Left,
    Right,
    Int(u64),
    End,
}


fn read() -> Vec<Vec<Token>> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|s| {
            chain(
                s.unwrap().chars()
                    .filter_map(|c| {
                        match c {
                            '0'..='9' => Some(Token::Int(c.to_digit(10).unwrap().into())),
                            '+' => Some(Token::Add),
                            '*' => Some(Token::Mul),
                            '(' => Some(Token::Left),
                            ')' => Some(Token::Right),
                            _ => None,
                        }
                    }),
                vec![Token::End]
            ).collect()

        })
        .collect()
}


fn evaluate(problem: &[Token]) -> u64 {
    let mut stack = Vec::new();

    for t in problem {
        stack.push(*t);

        loop {
            if stack.len() > 3 {
                let l = stack.len();
                match (stack[l - 4], stack[l - 3], stack[l - 2], stack[l - 1]) {
                    (Token::Int(a), Token::Mul, Token::Int(b), x) if x != Token::Add => {
                        stack.pop(); stack.pop(); stack.pop(); stack.pop();
                        stack.push(Token::Int(a * b));
                        stack.push(x);
                    },
                    _ => {},
                }
            }

            if stack.len() >= 3 {
                let l = stack.len();
                match (stack[l - 3], stack[l - 2], stack[l - 1]) {
                    (Token::Left, Token::Int(a), Token::Right) => {
                        stack.pop(); stack.pop(); stack.pop();
                        stack.push(Token::Int(a));
                    },
                    (Token::Int(a), Token::Add, Token::Int(b)) => {
                        stack.pop(); stack.pop(); stack.pop();
                        stack.push(Token::Int(a + b));
                    },
                    _ => { break; },
                }
            }

            if stack.len() < 3 { break; }
        }
    }

    match stack[0] {
        Token::Int(x) => x,
        _ => { println!("{:?}", stack); unreachable!(); },
    }
}


fn main() {
    let problems = read();

    let sum = problems.iter()
        .map(|p| evaluate(p))
        .sum::<u64>();

    println!("sum: {}", sum);
}
