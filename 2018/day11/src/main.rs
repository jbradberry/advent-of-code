use std::cmp;
use std::io;
use std::io::prelude::*;


fn power_level(x: i64, y: i64, serial_num: i64) -> i64 {
    let rack_id = x + 10;
    let level = rack_id * (y * rack_id + serial_num);

    (level / 100) % 10 - 5
}


fn subgrid_power(x: i64, y: i64, size: i64, serial_num: i64) -> i64 {
    (0..size).map(|a| {
        (0..size).map(|b| power_level(x + a, y + b, serial_num)).sum::<i64>()
    }).sum()
}


fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let serial_num: i64 = buffer.trim().parse().unwrap();

    let (power, x, y, size): (i64, i64, i64, i64) = (0..300).flat_map(|x| {
        (0..300).map(move |y| {
            let max_size = 300 - cmp::max(x, y);
            let (p, s) = (1..max_size + 1).map(|s| (subgrid_power(x, y, s, serial_num), s)).max().unwrap();

            (p, x, y, s)
        })
    }).max().unwrap();

    println!("power level = {}, coordinates = ({}, {}), size = {}", power, x, y, size);
}
