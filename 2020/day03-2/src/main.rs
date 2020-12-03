use std::io;
use std::io::prelude::*;


fn read() -> Vec<Vec<char>> {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|x| x.unwrap().chars().collect())
        .collect()
}


fn trees_encountered(map: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    (0..map.len()).step_by(down)
        .enumerate()
        .filter(|(i, r)| map[*r][(right * i) % map[0].len()] == '#')
        .count()
}


fn main() {
    let map = read();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut prod: usize = 1;
    for (right, down) in slopes {
        let trees = trees_encountered(&map, right, down);
        println!("right: {}, down: {}, trees: {}", right, down, trees);
        prod *= trees;
    }

    println!("product: {}", prod);
}
