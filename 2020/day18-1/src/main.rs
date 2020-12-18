use std::io;
use std::io::prelude::*;


#[derive(Debug, Copy, Clone)]
enum Token {
    Add,
    Mul,
    Left,
    Right,
    Int(u64),
}


fn read() -> Vec<Vec<Token>> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|s| {
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
                })
                .collect()
        })
        .collect()
}


fn evaluate(problem: &[Token]) -> u64 {
    let mut stack = Vec::new();

    for t in problem {
        stack.push(*t);

        loop {
            let l = stack.len();
            if l < 3 { break; }

            match (stack.pop().unwrap(), stack.pop().unwrap(), stack.pop().unwrap()) {
                (Token::Right, Token::Int(a), Token::Left) => { stack.push(Token::Int(a)); },
                (Token::Int(a), Token::Add, Token::Int(b)) => { stack.push(Token::Int(a + b)); },
                (Token::Int(a), Token::Mul, Token::Int(b)) => { stack.push(Token::Int(a * b)); },
                (x, y, z) => { stack.push(z); stack.push(y); stack.push(x); break; }
            }
        }
    }

    match stack[0] {
        Token::Int(x) => x,
        _ => unreachable!(),
    }
}


fn main() {
    let problems = read();

    let sum = problems.iter()
        .map(|p| evaluate(p))
        .sum::<u64>();

    println!("sum: {}", sum);
}
