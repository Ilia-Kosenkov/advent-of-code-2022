use std::{
    io::{self, BufRead},
    str::FromStr,
};

use itertools::Itertools;

fn unit<T>(_: T) {}

#[derive(Debug)]
enum Command {
    Ls,
    Cd { path: String },
}

#[derive(Debug)]
enum Item {
    File { name: String, size: usize },
    Dir { name: String },
}

#[derive(Debug)]
enum Input {
    Command(Command),
    Item(Item),
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ ls") {
            Ok(Input::Command(Command::Ls))
        } else if s.starts_with("$ cd") {
            Ok(Input::Command(Command::Cd {
                path: s[4..].trim().to_string(),
            }))
        } else if s.starts_with("dir") {
            Ok(Input::Item(Item::Dir {
                name: s[3..].trim().to_string(),
            }))
        } else {
            let mut split = s.trim().split_whitespace();
            let size = split.next().and_then(|s| s.parse::<usize>().ok());
            let name = split.next().map(|s| s.to_string());

            match (size, name) {
                (Some(size), Some(name)) => Ok(Input::Item(Item::File {
                    name: name,
                    size: size,
                })),
                _ => Err(()),
            }
        }
    }
}

fn main() {
    let reader = io::BufReader::new(io::stdin());
    let lines = reader
        .lines()
        .map(|l| l.map_err(unit).and_then(|s| s.parse::<Input>()));

    dbg! { lines.collect::<Vec<_>>() };
}
