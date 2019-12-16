use std::io;
use std::io::prelude::*;

use itertools::Itertools;


fn read() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    buffer.trim().to_string()
}


fn parse_list(input: &str) -> Vec<u8> {
    input.chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect()
}


fn calculate_phase(digits: &[u8]) -> Vec<u8> {
    (0..digits.len())
        .map(|i| {
            let mut seq = [0, 1, 0, -1].iter().flat_map(|x| (0..i + 1).map(move |_| x)).cycle();
            seq.next();

            (digits.iter()
             .zip(seq)
             .map(|(&a, &b)| a as i32 * b)
             .sum::<i32>().abs() % 10) as u8
        })
        .collect()
}


fn main() {
    let input = read();
    let mut data = parse_list(&input);

    for _ in 0..100 {
        data = calculate_phase(&data);
    }

    println!("Part 1: {}", data.iter().take(8).join(""));

    let new_input = input.repeat(10_000);
    let offset = new_input.chars().take(7).join("").parse().unwrap();
    data = parse_list(&new_input);

    for _ in 0..100 {
        data = calculate_phase(&data);
    }

    println!("Part 2: {}", data.iter().skip(offset).take(8).join(""));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let mut data = parse_list("12345678");
        data = calculate_phase(&data);
        assert_eq!(data, [4, 8, 2, 2, 6, 1, 5, 8]);
        data = calculate_phase(&data);
        assert_eq!(data, [3, 4, 0, 4, 0, 4, 3, 8]);
        data = calculate_phase(&data);
        assert_eq!(data, [0, 3, 4, 1, 5, 5, 1, 8]);
        data = calculate_phase(&data);
        assert_eq!(data, [0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_big1() {
        let mut data = parse_list("80871224585914546619083218645595");
        for _ in 0..100 {
            data = calculate_phase(&data);
        }

        assert_eq!(data[0..8], [2, 4, 1, 7, 6, 1, 7, 6]);
    }

    #[test]
    fn test_big2() {
        let mut data = parse_list("19617804207202209144916044189917");
        for _ in 0..100 {
            data = calculate_phase(&data);
        }

        assert_eq!(data[0..8], [7, 3, 7, 4, 5, 4, 1, 8]);
    }

    #[test]
    fn test_big3() {
        let mut data = parse_list("69317163492948606335995924319873");
        for _ in 0..100 {
            data = calculate_phase(&data);
        }

        assert_eq!(data[0..8], [5, 2, 4, 3, 2, 1, 3, 3]);
    }
}
