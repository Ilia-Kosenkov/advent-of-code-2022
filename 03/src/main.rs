use std::{
    collections::*,
    io::{self, BufRead},
};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct RucksackPockets(HashSet<char>, HashSet<char>);

#[derive(Debug, Clone)]
struct RucksackBunch(RucksackPockets, RucksackPockets, RucksackPockets);

impl Into<RucksackBunch> for (RucksackPockets, RucksackPockets, RucksackPockets) {
    fn into(self) -> RucksackBunch {
        RucksackBunch(self.0, self.1, self.2)
    }
}

impl RucksackPockets {
    fn get_unique_items(&self) -> HashSet<char> {
        self.0.union(&self.1).cloned().collect()
    }

    fn get_repeating_items(&self) -> HashSet<char> {
        self.0.intersection(&self.1).cloned().collect()
    }
}

impl RucksackBunch {
    fn find_common(&self) -> char {
        *self
            .0
            .get_unique_items()
            .intersection(&self.1.get_unique_items())
            .cloned()
            .collect::<HashSet<char>>()
            .intersection(&self.2.get_unique_items())
            .exactly_one()
            .unwrap()
    }
}

fn main() {
    let reader = io::BufReader::new(io::stdin());
    let split_lines = reader.lines().filter_map(|l| {
        l.ok().and_then(|s| {
            Some((
                s[..(s.len() / 2)].to_string(),
                s[(s.len() / 2)..].to_string(),
            ))
        })
    });

    let sets = split_lines.map(|(p1, p2)| {
        RucksackPockets(
            HashSet::<_>::from_iter(p1.chars()),
            HashSet::<_>::from_iter(p2.chars()),
        )
    });

    let bunches = sets.tuples::<(_, _, _)>().map(Into::<RucksackBunch>::into);

    let items = bunches.map(|b| {
        (
            b.find_common(),
            b.0.get_repeating_items(),
            b.1.get_repeating_items(),
            b.2.get_repeating_items(),
        )
    });

    let priorities = items.map(|(c, r1, r2, r3)| {
        (
            get_priority(&c),
            r1.iter().map(get_priority).collect::<Vec<i32>>(),
            r2.iter().map(get_priority).collect::<Vec<i32>>(),
            r3.iter().map(get_priority).collect::<Vec<i32>>(),
        )
    });

    let sum = priorities.fold((0i32, 0i32), |acc, (c, r1, r2, r3)| {
        (
            acc.0 + c,
            acc.1 + r1.iter().sum::<i32>() + r2.iter().sum::<i32>() + r3.iter().sum::<i32>(),
        )
    });

    println!("{}", sum.1);
    println!("{}", sum.0);
}

fn get_priority(c: &char) -> i32 {
    if c.is_lowercase() {
        (*c as i32) - ('a' as i32) + 1i32
    } else {
        (*c as i32) - ('A' as i32) + 27i32
    }
}
