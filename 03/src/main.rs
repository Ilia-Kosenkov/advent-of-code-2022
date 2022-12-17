use std::{
    collections::*,
    io::{self, BufRead},
};

fn main() {
    let reader = io::BufReader::new(io::stdin());
    let split_lines = reader.lines().filter_map(|l| {
        l.ok()
            .and_then(|s| Some((s[..s.len() / 2].to_string(), s[s.len() / 2..].to_string())))
    });

    let sets = split_lines.map(|(p1, p2)| {
        (
            HashSet::<_>::from_iter(p1.chars()),
            HashSet::<_>::from_iter(p2.chars()),
        )
    });

    let items = sets.map(|(s1, s2)| s1.intersection(&s2).map(|c| *c).collect::<Vec<char>>());

    let priorities = items.map(|p| p.iter().map(get_priority).sum::<i32>());

    let sum = priorities.sum::<i32>();

    println!("{}", sum);
}

fn get_priority(c: &char) -> i32 {
    if c.is_lowercase() {
        (*c as i32) - ('a' as i32) + 1i32
    } else {
        (*c as i32) - ('A' as i32) + 27i32
    }
}
