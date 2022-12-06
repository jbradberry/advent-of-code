use std::io;


fn main() {
    let mut datastream = String::new();
    io::stdin()
        .read_line(&mut datastream)
        .expect("Failure to read data stream.");

    println!("data stream: {:?}", datastream);
}
