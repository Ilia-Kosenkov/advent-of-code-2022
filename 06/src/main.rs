use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use itertools::Itertools;
use sliding_windows::{IterExt, Storage, Window};

fn unit<T>(_: T) {}

fn main() {
    let size = 14;
    let mut storage = Storage::<char>::new(size);

    let reader = io::BufReader::new(io::stdin());
    let result = reader
        .lines()
        .exactly_one()
        .map_err(unit)
        .and_then(|x| x.map_err(unit))
        .and_then(|l| {
            l.chars()
                .sliding_windows(&mut storage)
                .enumerate()
                .filter_map(|(i, w)| {
                    if are_all_unique(&w) {
                        Some(i + size)
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

fn are_all_unique(window: &Window<char>) -> bool {
    let mut set = HashSet::<char>::new();
    for c in window {
        if !set.insert(*c) {
            return false;
        }
    }
    return true;
}
