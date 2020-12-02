fn main() {
    let coords = vec![(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];

    for (x, y) in &coords {
        println!("({}, {}): {}", x, y, -(*x as f64).atan2(*y as f64));
    }
}
