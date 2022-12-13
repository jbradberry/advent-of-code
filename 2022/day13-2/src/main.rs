use std::io;
use std::io::prelude::*;

use std::cmp::Ordering;

use itertools::EitherOrBoth;
use itertools::Itertools;

use serde_json::{Result, Value};


fn read() -> Result<Vec<Value>> {
    let stdin = io::stdin();

    let mut results = Vec::new();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line == "" { continue; }

        results.push(serde_json::from_str(&line)?)
    }

    Ok(results)
}


fn compare(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(_), Value::Number(_)) => left.as_u64().unwrap().cmp(&right.as_u64().unwrap()),
        (Value::Array(_), Value::Array(_)) => {
            for x in left.as_array().unwrap().iter().zip_longest(right.as_array().unwrap().iter()) {
                match x {
                    EitherOrBoth::Both(l, r) => match compare(l, r) {
                        Ordering::Equal => { continue; },
                        o @ _ => { return o; }
                    },
                    EitherOrBoth::Left(_) => { return Ordering::Greater; },
                    EitherOrBoth::Right(_) => { return Ordering::Less; },
                }
            }
            Ordering::Equal
        },
        (Value::Number(_), Value::Array(_)) => compare(&Value::Array(vec![left.clone()]), right),
        (Value::Array(_), Value::Number(_)) => compare(left, &Value::Array(vec![right.clone()])),
        _ => unreachable!(),
    }
}


fn main() {
    let mut output = read().unwrap();

    // add in the special divider packets
    output.push(serde_json::from_str("[[2]]").unwrap());
    output.push(serde_json::from_str("[[6]]").unwrap());

    output.sort_by(compare);

    // println!("{:#?}", output);

    let loc2 = output.iter().map(|x| serde_json::to_string(x).unwrap()).position(|x| x == "[[2]]").unwrap() + 1;
    let loc6 = output.iter().map(|x| serde_json::to_string(x).unwrap()).position(|x| x == "[[6]]").unwrap() + 1;

    println!("decoder key: {}", loc2 * loc6);
}
