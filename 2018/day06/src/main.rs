use std::io;
use std::io::prelude::*;

use regex::Regex;


fn distance(point_a: (i32, i32), point_b: (i32, i32)) -> i32 {
    (point_a.0 - point_b.0).abs() + (point_a.1 - point_b.1).abs()
}


fn read_coordinates() -> Vec<(i32, i32)> {
    let re = Regex::new(r"^(-?\d+), (-?\d+)$").unwrap();
    let stdin = io::stdin();

    stdin.lock().lines().map(|x| {
        let line = x.unwrap();
        let cap = re.captures(&line).unwrap();

        (cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
         cap.get(2).unwrap().as_str().parse::<i32>().unwrap())
    }).collect()
}


fn main() {
    let coordinates = read_coordinates();

    let x_coordinates = coordinates.iter().map(|&(x, _)| x).collect::<Vec<i32>>();
    let y_coordinates = coordinates.iter().map(|&(_, y)| y).collect::<Vec<i32>>();
    let x_min = *x_coordinates.iter().min().unwrap();
    let x_max = *x_coordinates.iter().max().unwrap();
    let y_min = *y_coordinates.iter().min().unwrap();
    let y_max = *y_coordinates.iter().max().unwrap();

    let mut total = 0u32;
    for x in (x_min - 10000)..(x_max + 10001) {
        for y in (y_min - 10000)..(y_max + 10001) {
            if coordinates.iter().map(|&(m, n)| distance((x, y), (m, n))).sum::<i32>() < 10000 { total += 1; }
        }
    }
    println!("region size = {}", total);
}
