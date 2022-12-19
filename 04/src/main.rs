use std::{
    io::{self, BufRead},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Default)]
struct IdRange {
    start: i32,
    end: i32,
}

#[derive(Debug, Clone, Copy, Default)]
struct RangePair(IdRange, IdRange);

impl FromStr for IdRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split('-')
            .collect_tuple::<(_, _)>()
            .and_then(|(s1, s2)| match (s1.parse::<i32>(), s2.parse::<i32>()) {
                (Ok(start), Ok(end)) => Some(IdRange {
                    start: start,
                    end: end,
                }),
                _ => None,
            })
            .ok_or(())
    }
}

impl FromStr for RangePair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .collect_tuple::<(_, _)>()
            .and_then(
                |(s1, s2)| match (s1.parse::<IdRange>(), s2.parse::<IdRange>()) {
                    (Ok(left), Ok(right)) => Some(RangePair(left, right)),
                    _ => None,
                },
            )
            .ok_or(())
    }
}

impl IdRange {
    fn fully_contains(&self, other: &IdRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &IdRange) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (self.start <= other.end && other.end <= self.end)
    }
}

impl RangePair {
    fn fully_contains(&self) -> bool {
        self.0.fully_contains(&self.1) || self.1.fully_contains(&self.0)
    }

    fn overlaps(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }
}

fn main() {
    let reader = io::BufReader::new(io::stdin());
    let mut ranges = reader
        .lines()
        .map(|l| l.map_err(|_| ()).and_then(|s| s.parse::<RangePair>()));

    let result = ranges.fold_ok((0i32, 0i32), |(cnt, ovp), pair| {
        (
            if pair.fully_contains() { cnt + 1 } else { cnt },
            if pair.overlaps() { ovp + 1 } else { ovp },
        )
    });

    let result = result.unwrap_or((-1i32, -1i32));

    println!("{}", result.0);
    println!("{}", result.1);
}
