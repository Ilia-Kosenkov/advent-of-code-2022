use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    io::{self, BufRead},
    iter::repeat,
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    AddX(i32),
    NoOp,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let cmd = iter.next();
        let val = iter
            .next()
            .ok_or(())
            .and_then(|ss| ss.parse::<i32>().map_err(unit));
        let end = iter.next();

        if end.is_some() {
            return Err(());
        }

        match (cmd, val) {
            (Some("noop"), _) => Ok(Command::NoOp),
            (Some("addx"), Ok(val)) => Ok(Command::AddX(val)),
            _ => Err(()),
        }
    }
}

impl Command {
    fn new_state(&self, x: i32) -> i32 {
        match self {
            Self::NoOp => x,
            Self::AddX(val) => x + val,
        }
    }

    fn n_cycles(&self) -> i32 {
        match self {
            Self::NoOp => 1,
            Command::AddX(_) => 2,
        }
    }
}

fn unit<T>(_: T) {}

fn main() {
    let reader = io::BufReader::new(io::stdin());
    let input = reader
        .lines()
        .map(|l| l.map_err(unit).and_then(|s| s.parse::<Command>()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let cycles = HashSet::<i32>::from_iter(vec![20, 60, 100, 140, 180, 220]);

    let mut results = HashMap::<i32, i32>::new();

    let mut state = 1;
    let mut cycle = 1;

    let mut buffer = repeat('.').take(240).collect_vec();

    for cmd in input.iter() {
        let new_state = cmd.new_state(state);
        let new_cycle = cycle + cmd.n_cycles();

        set_pixel(&mut buffer, cycle, state);
        if let Command::AddX(_) = cmd {
            set_pixel(&mut buffer, cycle + 1, state);
        }

        state = new_state;
        cycle = new_cycle;

        if let Some(mark_cycle) = cycles.get(&cycle).or_else(|| cycles.get(&(cycle + 1))) {
            results.insert(*mark_cycle, state);
        }
    }

    let sum = results.iter().map(|(k, v)| k * v).sum::<i32>();
    println!("{}", sum);

    draw(&buffer);
}

fn draw(display: &Vec<char>) {
    for line in display.iter().chunks(40).into_iter() {
        let line = line.collect::<String>();
        println!("{}", line);
    }
}

fn set_pixel(buffer: &mut Vec<char>, cycle: i32, state: i32) {
    let cycle = cycle - 1;

    if (cycle % 40 - state).abs() <= 1 {
        buffer[cycle as usize] = '#';
    }
}
