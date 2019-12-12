use std::io;
use std::io::prelude::*;


fn read_license() -> Vec<u32> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.split_whitespace().map(|x| x.parse().unwrap()).collect()
}


fn walk_license(license: &[u32], mut index: usize) -> (u32, usize) {
    let children = license[index];
    let metadata = license[index + 1];
    index += 2;

    println!("children = {}, metadata = {}", children, metadata);

    let mut values = Vec::new();
    for _ in 0..children {
        let (v, i) = walk_license(&license, index);
        values.push(v);
        index = i;
    }

    let value = match values.is_empty() {
        false => (0..metadata).map(|i| match values.get(license[index + i as usize] as usize - 1) {
            Some(&x) => x,
            None => 0,
        }).sum(),
        true => (0..metadata).map(|i| license[index + i as usize]).sum(),
    };
    index += metadata as usize;

    println!("value = {}", value);
    (value, index)
}


fn main() {
    let license = read_license();

    let (value, index) = walk_license(&license, 0);
    assert_eq!(index, license.len());

    println!("sum = {}", value);
}
