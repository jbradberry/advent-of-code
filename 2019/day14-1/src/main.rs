use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;


fn parse_name(desc: &str) -> String {
    let mut parts = desc.split(' ');
    parts.next();
    parts.next().unwrap().to_string()
}


fn parse_quant(desc: &str) -> i32 {
    let mut parts = desc.split(' ');
    parts.next().unwrap().parse().unwrap()
}


fn read() -> Vec<HashMap<String, i32>>{
    let stdin = io::stdin();
    stdin.lock().lines().map(|x| {
        let equation = x.unwrap().split("=>").map(|s| s.trim().to_string()).collect::<Vec<_>>();
        let mut row = equation[0].split(", ")
            .map(|s| (parse_name(&s), parse_quant(&s))).collect::<HashMap<String, i32>>();
        row.insert(parse_name(&equation[1]), -parse_quant(&equation[1]));

        row
    }).collect()
}


fn main() {
    let mut data = read();
    let vars = data.iter().map(|r| r.keys())
        .flatten()
        .cloned()
        .filter(|x| x != "FUEL" && x != "ORE")
        .collect::<HashSet<String>>();
    let mut columns = vars.into_iter().collect::<Vec<_>>();
    columns.insert(0, "FUEL".to_string());

    assert_eq!(columns.len(), data.len());

    columns.push("ORE".to_string());

    let mut matrix = data.iter()
        .map(|r| columns.iter().cloned().map(|c| *r.get(&c).unwrap_or(&0) as f64).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let n = data.len();
    for i in 1..=(n - 1) {
        let p = (i..=n).filter(|x| matrix[x - 1][i - 1] != 0.0).min().expect("No unique solution exists.");
        if p != i { matrix.swap(p - 1, i - 1); }
        for j in (i + 1)..=n {
            let m = matrix[j - 1][i - 1] / matrix[i - 1][i - 1];
            for x in 1..=(n + 1) {
                matrix[j - 1][x - 1] = matrix[j - 1][x - 1] - m * matrix[i - 1][x - 1];
            }
        }
    }
    if matrix[n - 1][n - 1] == 0.0 { panic!("No unique solution exists."); }
    let mut x = vec![0.0; n];
    x[n - 1] = matrix[n - 1][n] / matrix[n - 1][n - 1];
    for i in 1..n {
        x[n - i - 1] = (matrix[i - 1][n] - ((n - i)..=n).map(|p| matrix[n - i - 1][p - 1] * x[p - 1]).sum::<f64>())
            / matrix[n - i - 1][n - i - 1];
    }

    println!("{:?}", x);
}
