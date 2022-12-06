use std::io;

use itertools::Itertools;


fn find_header(s: &str) -> usize {
    s.chars()
        .tuple_windows::<(_, _, _, _)>()
        .enumerate()
        .find_map(|(i, (a, b, c, d))| {
            if (a == b) || (a == c) || (a == d) || (b == c) || (b == d) || (c == d) { return None }
            Some(i + 4)
        })
        .unwrap()
}


fn main() {
    let mut datastream = String::new();
    io::stdin()
        .read_line(&mut datastream)
        .expect("Failure to read data stream.");

    println!("data stream: {:?}", datastream);

    let index = find_header(&datastream);

    println!("index of header: {}", index);
}
