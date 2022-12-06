use std::io;

use itertools::Itertools;


fn find_header(s: &str, n: usize) -> usize {
    let chars = s.chars().collect::<Vec<_>>();
    let mut intermediate = chars.iter().map(|_| true).collect::<Vec<_>>();

    for x in 1..n {
        intermediate = intermediate.iter()
            .tuple_windows::<(_, _)>()
            .enumerate()
            .map(|(i, (a, b))| (chars[i] != chars[i + x]) && *a && *b)
            .collect();
        // println!("*** {:?}", intermediate);
    }

    intermediate.iter().position(|x| *x).unwrap() + n
}


fn main() {
    let mut datastream = String::new();
    io::stdin()
        .read_line(&mut datastream)
        .expect("Failure to read data stream.");

    println!("data stream: {:?}", datastream);

    let index = find_header(&datastream, 14);

    println!("index of header: {}", index);
}
