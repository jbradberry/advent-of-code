use std::io;
use std::io::prelude::*;


fn read() -> Vec<(usize, i64)> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|s| s.unwrap());
    lines.next();

    lines.next().unwrap()
        .split(",")
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| (i, s.parse().unwrap()))
        .collect()
}


fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    match a {
        0 => (b, 0, 1),
        _ => {
            let (g, x, y) = egcd(b % a, a);
            (g, y - (b / a) * x, x)
        },
    }
}


fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    match g {
        1 => Some((x % n + n) % n),
        _ => None,
    }
}


fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod: i64 = modulii.iter().product();
    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p;
    }

    Some(sum % prod)
}


fn main() {
    let buses = read();
    let a_i: Vec<i64> = buses.iter().map(|(i, b)| (b - (*i as i64 % b)) % b).collect();
    let n_i: Vec<i64> = buses.iter().map(|(_, b)| *b).collect();

    let x = chinese_remainder(&a_i, &n_i).unwrap();
    println!("timestamp: {}", x)
}
