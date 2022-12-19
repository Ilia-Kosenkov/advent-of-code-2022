use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use itertools::Itertools;

fn unit<T>(_: T) {}

fn main() {
    let reader = io::BufReader::new(io::stdin());
    let result = reader
        .lines()
        .exactly_one()
        .map_err(unit)
        .and_then(|x| x.map_err(unit))
        .and_then(|l| {
            l.chars()
                .tuple_windows::<(_, _, _, _)>()
                .enumerate()
                .filter_map(|(i, w)| {
                    if are_all_unique(&w) {
                        Some(i + 4)
                    } else {
                        None
                    }
                })
                .take(1)
                .at_most_one()
                .map_err(unit)
                .and_then(|i| i.ok_or(()))
        });

    println!("{:?}", result.unwrap_or(<usize>::MAX));
}

fn are_all_unique(x: &(char, char, char, char)) -> bool {
    let mut set = HashSet::<char>::with_capacity(4);
    set.insert(x.0) && set.insert(x.1) && set.insert(x.2) && set.insert(x.3)
}
