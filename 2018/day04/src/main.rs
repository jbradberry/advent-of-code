use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

use counter::Counter;
use regex::Regex;


fn part01() -> Result<(), std::io::Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let re = Regex::new(
        r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] .*(asleep|wakes|#\d+).*"
    ).unwrap();

    let mut records = re.captures_iter(&buffer).filter_map(|cap| {
        let groups = (cap.get(1), cap.get(2), cap.get(3), cap.get(4), cap.get(5), cap.get(6));
        match groups {
            (Some(y), Some(m), Some(d), Some(hh), Some(mm), Some(t)) => Some(
                (y.as_str(), m.as_str(), d.as_str(), hh.as_str(), mm.as_str(), t.as_str())
            ),
            _ => None,
        }
    }).collect::<Vec<_>>();
    records.sort();

    let mut intervals = vec![];
    let mut counts = HashMap::new();
    let mut guard = 0;
    let mut start = 0;
    let mut stop: u16;
    for (_y, _m, d, _hh, mm, t) in &records {
        match t.as_ref() {
            "asleep" => {
                start = mm.parse().unwrap();
            },
            "wakes" => {
                stop = mm.parse().unwrap();
                *counts.entry(guard).or_insert(0) += stop - start;
                intervals.push((d.parse::<u16>().unwrap(), guard, start, stop));
            },
            _ => {
                guard = t.replace("#", "").parse::<u16>().unwrap();
            },
        }
    }

    let (guard, minutes) = counts.iter().fold((0, 0), |(guard, m), (&key, &val)| {
        if val > m { (key, val) }
        else { (guard, m) }
    });

    println!("guard = {}, minutes = {}", guard, minutes);

    let mut minute_count: Counter<u16, u16> = Counter::new();

    for (d, g, start, stop) in &intervals {
        if *g == guard {
            minute_count += (*start..*stop).collect::<Counter<u16, u16>>();
        }
    }

    let chosen_minute = minute_count.most_common_ordered()[0].0;
    println!("chosen minute = {}, product = {}", chosen_minute, guard * chosen_minute);

    Ok(())
}


fn main() -> Result<(), std::io::Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let re = Regex::new(
        r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] .*(asleep|wakes|#\d+).*"
    ).unwrap();

    let mut records = re.captures_iter(&buffer).filter_map(|cap| {
        let groups = (cap.get(1), cap.get(2), cap.get(3), cap.get(4), cap.get(5), cap.get(6));
        match groups {
            (Some(y), Some(m), Some(d), Some(hh), Some(mm), Some(t)) => Some(
                (y.as_str(), m.as_str(), d.as_str(), hh.as_str(), mm.as_str(), t.as_str())
            ),
            _ => None,
        }
    }).collect::<Vec<_>>();
    records.sort();

    let mut intervals = vec![];
    let mut guard = 0;
    let mut start = 0;
    let mut stop: u16;
    for (_y, _m, d, _hh, mm, t) in &records {
        match t.as_ref() {
            "asleep" => {
                start = mm.parse().unwrap();
            },
            "wakes" => {
                stop = mm.parse().unwrap();
                intervals.push((d.parse::<u16>().unwrap(), guard, start, stop));
            },
            _ => {
                guard = t.replace("#", "").parse::<u16>().unwrap();
            },
        }
    }

    let mut minute_count: Counter<(u16, u16), u16> = Counter::new();
    for (d, g, start, stop) in &intervals {
        minute_count += (*start..*stop).map(|x| (*g, x));
    }

    let (guard, chosen_minute) = minute_count.most_common_ordered()[0].0;
    println!("guard = {}, chosen minute = {}, product = {}", guard, chosen_minute, guard * chosen_minute);

    Ok(())
}
