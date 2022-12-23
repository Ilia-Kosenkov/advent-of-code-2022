use std::{
    collections::HashSet,
    fmt::Debug,
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Pos(i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    direction: Direction,
    step_count: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct PosChange(i32, i32);

impl Into<PosChange> for Direction {
    fn into(self) -> PosChange {
        match self {
            Direction::Up => PosChange(0, 1),
            Direction::Right => PosChange(1, 0),
            Direction::Down => PosChange(0, -1),
            Direction::Left => PosChange(-1, 0),
        }
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", &self.0, &self.1))
    }
}

impl Pos {
    fn start() -> Pos {
        Pos(11, 5)
        //std::default::Default::default()
    }

    fn move_towards(&mut self, to: &Pos) {
        let d = self.distance(to);
        let abs_d = (d.0.abs(), d.1.abs());

        if abs_d.0 + abs_d.1 == 3 {
            self.0 += d.0.signum();
            self.1 += d.1.signum();
            return;
        }
        if abs_d.0 == 2 {
            self.0 += d.0.signum();
        }
        if abs_d.1 == 2 {
            self.1 += d.1.signum();
        }
    }

    fn distance(&self, other: &Pos) -> (i32, i32) {
        (other.0 - self.0, other.1 - self.1)
    }

    fn r#move(&mut self, dir: &PosChange) {
        self.0 += dir.0;
        self.1 += dir.1;
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace().filter(|ss| !ss.is_empty());

        let direction = iter
            .next()
            .ok_or(())
            .and_then(|c| c.parse::<char>().map_err(unit));
        let step_count = iter
            .next()
            .ok_or(())
            .and_then(|c| c.parse::<i32>().map_err(unit));

        match (direction, step_count) {
            (Ok(direction), Ok(step_count)) => match direction {
                'U' => Ok(Move {
                    direction: Direction::Up,
                    step_count: step_count,
                }),
                'R' => Ok(Move {
                    direction: Direction::Right,
                    step_count: step_count,
                }),
                'D' => Ok(Move {
                    direction: Direction::Down,
                    step_count: step_count,
                }),
                'L' => Ok(Move {
                    direction: Direction::Left,
                    step_count: step_count,
                }),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

fn unit<T>(_: T) {}

fn main() {
    let reader = io::BufReader::new(io::stdin());

    let input = reader
        .lines()
        .map(|l| l.map_err(unit).and_then(|s| s.parse::<Move>()));

    let pos_changes = input.flat_map(|m| match m {
        Ok(mv) => std::iter::repeat(<Direction as Into<PosChange>>::into(mv.direction))
            .take(mv.step_count as usize),
        _ => std::iter::repeat(PosChange::default()).take(0usize),
    });

    let n = 10;
    let mut rope: Vec<Pos> = std::iter::repeat_with(|| Pos::start()).take(n).collect();

    let mut history = HashSet::<Pos>::new();
    history.insert(rope[n - 1]);

    for pos_change in pos_changes {
        rope[0].r#move(&pos_change);
        for i in 1..n {
            let to = rope[i - 1];
            rope[i].move_towards(&to);
        }
        history.insert(rope[n - 1]);
    }

    println!("{}", history.len());
}
