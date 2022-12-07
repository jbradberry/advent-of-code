use std::collections::HashMap;
use std::io;
use std::io::prelude::*;


#[derive(Debug)]
enum FSObject {
    Dir(String),
    File(String, u64)
}


fn read() -> HashMap<String, Vec<FSObject>> {
    let mut fs = HashMap::new();
    let mut current = String::new();

    let stdin = io::stdin();

    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let tokens = line.split_whitespace().collect::<Vec<_>>();

        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                match tokens[2] {
                    "/" => { current = "/".to_string(); },
                    ".." => {
                        current = current
                            .trim_end_matches('/')
                            .rsplit_once('/').unwrap()
                            .0.to_string() + "/";
                    },
                    _ => { current = current + tokens[2] + "/"; }
                }
            }
            // we don't care about 'ls'
        } else {
            if tokens[0] == "dir" {
                fs.entry(current.to_string())
                    .or_insert_with(Vec::new)
                    .push(FSObject::Dir(tokens[1].to_string()));
            } else {
                fs.entry(current.to_string())
                    .or_insert_with(Vec::new)
                    .push(FSObject::File(tokens[1].to_string(), tokens[0].parse().unwrap()));
            }
        }
    }

    fs
}


fn path_size(fs: &HashMap<String, Vec<FSObject>>, path: &str) -> u64 {
    fs.get(path).unwrap()
        .iter()
        .map(|x| {
            match x {
                FSObject::Dir(d) => path_size(fs, &(path.to_owned() + d + "/")),
                FSObject::File(_, s) => *s
            }
        })
        .sum::<u64>()
}


fn main() {
    let fs = read();

    println!("fs: {:?}", fs);

    println!("size: {}", path_size(&fs, "/"));
}
