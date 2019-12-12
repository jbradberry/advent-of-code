use std::char;
use std::io;
use std::io::prelude::*;


fn part01() -> Result<(), std::io::Error> {
    let mut polymer = String::new();
    io::stdin().read_to_string(&mut polymer)?;
    polymer = polymer.trim().to_string();

    loop {
        let mut new_polymer = String::new();
        let mut modified = false;

        for c in polymer.chars() {
            match new_polymer.chars().last() {
                Some(last) => {
                    if (c.to_lowercase().to_string() == last.to_lowercase().to_string()) && (c != last) {
                        new_polymer.pop();
                        modified = true;
                    } else {
                        new_polymer.push(c);
                    }
                },
                None => { new_polymer.push(c); },
            }
        }

        polymer = new_polymer;
        if !modified { break; }
    }

    println!("polymer = '{}'", polymer);
    println!("length = {}", polymer.len());

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let mut polymer = String::new();
    io::stdin().read_to_string(&mut polymer)?;

    let mut results = Vec::new();
    for ord in 97..123 {
        let mut current = polymer.trim().to_string();

        let remove = char::from_u32(ord).unwrap().to_string();

        loop {
            let mut new_polymer = String::new();
            let mut modified = false;

            for c in current.chars() {
                if c.to_lowercase().to_string() == remove { continue; }

                match new_polymer.chars().last() {
                    Some(last) => {
                        if (c.to_lowercase().to_string() == last.to_lowercase().to_string()) && (c != last) {
                            new_polymer.pop();
                            modified = true;
                        } else {
                            new_polymer.push(c);
                        }
                    },
                    None => { new_polymer.push(c); },
                }
            }

            current = new_polymer;
            if !modified { break; }
        }

        results.push((remove, current.len()));
    }

    let (remove, len) = results.iter().min_by_key(|(c, l)| l).unwrap();
    println!("removed = {}, length = {}", remove, len);

    Ok(())
}
