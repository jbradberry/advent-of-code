use std::io;
use std::io::prelude::*;

use itertools::Itertools;


fn read() -> Vec<u32> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.trim().chars().map(|c| c.to_digit(10).unwrap()).collect()
}


fn main() {
    let data = read();
    let layers = data.into_iter().chunks(25 * 6)
        .into_iter()
        .map(|c| c.collect::<Vec<u32>>())
        .collect::<Vec<_>>();

    let pixels = layers.into_iter().fold1(|layer1, layer2| {
        layer1.into_iter().zip(layer2.into_iter())
            .map(|(a, b)| vec![a, b].into_iter().filter(|&x| x != 2).next().unwrap_or(2))
            .collect::<Vec<_>>()
    }).unwrap();

    &pixels.into_iter().chunks(25)
        .into_iter()
        .map(|c| c.into_iter().map(|x| match x { 0 => ".", 1 => "#", _ => " " }).join(""))
        .for_each(|line| println!("{}", line));
}
