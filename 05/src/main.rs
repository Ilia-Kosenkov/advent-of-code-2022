use std::{
    collections::{self, HashMap, LinkedList},
    io::{self, BufRead},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Default)]
struct Crate(char);

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Stack(i32);

#[derive(Debug, Clone)]
struct Crates {
    stacks: collections::HashMap<Stack, collections::LinkedList<Crate>>,
}

#[derive(Debug, Clone, Copy, Default)]
struct Action {
    count: i32,
    from: Stack,
    to: Stack,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let move_idx = s.find("move");
        let from_idx = s.find("from");
        let to_idx = s.find("to");

        let count = parse_between(s, move_idx.map(|x| x + "move".len()), from_idx);
        let from = parse_between(s, from_idx.map(|x| x + "from".len()), to_idx);
        let to = parse_between(s, to_idx.map(|x| x + "to".len()), Some(s.len()));

        return match (count, from, to) {
            (Some(count), Some(from), Some(to)) => Ok(Action {
                from: Stack(from),
                to: Stack(to),
                count: count,
            }),
            _ => Err(()),
        };

        fn parse_between(s: &str, start: Option<usize>, end: Option<usize>) -> Option<i32> {
            match (start, end) {
                (Some(start), Some(end)) => Some(s[start..end].trim()),
                _ => None,
            }
            .and_then(|s| s.parse::<i32>().ok())
        }
    }
}

impl FromStr for Crate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('[').trim_end_matches(']');
        s.parse::<char>().map(|c| Crate(c)).map_err(|_| ())
    }
}

impl Crates {
    fn move_crates(&mut self, action: &Action) -> Option<()> {
        let moved_crates = {
            let from = self.stacks.get_mut(&action.from)?;
            let mut moved_crates = Vec::<Crate>::with_capacity(action.count as usize);
            for _ in 0..action.count {
                moved_crates.push(from.pop_front()?);
            }
            moved_crates
        };
        {
            let to = self.stacks.get_mut(&action.to)?;
            for c in moved_crates {
                to.push_front(c);
            }

            return Some(());
        }
    }

    fn move_crates_preserve_order(&mut self, action: &Action) -> Option<()> {
        let moved_crates = {
            let from = self.stacks.get_mut(&action.from)?;
            let mut moved_crates = Vec::<Crate>::with_capacity(action.count as usize);
            for _ in 0..action.count {
                moved_crates.push(from.pop_front()?);
            }
            moved_crates
        };
        {
            let to = self.stacks.get_mut(&action.to)?;
            for c in moved_crates.iter().rev() {
                to.push_front(*c);
            }

            return Some(());
        }
    }
}

fn main() {
    let reader = io::BufReader::new(io::stdin());
    let mut lines = reader.lines();

    let crates = parse_preamble(&mut lines);

    let actions = lines.map(|l| l.map_err(|_| ()).and_then(|s| s.parse::<Action>()));

    let result = if let Some(mut crates) = crates {
        move_crates_preserve_order(&mut crates, actions)
    } else {
        Err(())
    };

    println!("{}", result.unwrap_or("Err".to_string()));
}

fn get_top_crates(crates: &Crates) -> Result<String, ()> {
    crates
        .stacks
        .iter()
        .map(|(s, l)| (s, l.front()))
        .sorted_by_key(|(s, _)| *s)
        .map(|(_, l)| l)
        .collect::<Option<Vec<_>>>()
        .ok_or(())
        .map(|v| v.iter().map(|c| c.0).collect::<String>())
}

fn move_crates<T: Iterator<Item = Result<Action, ()>>>(
    crates: &mut Crates,
    actions: T,
) -> Result<String, ()> {
    for action in actions {
        if let Ok(action) = action {
            crates.move_crates(&action);
        } else {
            return Err(());
        }
    }
    return get_top_crates(crates);
}

fn move_crates_preserve_order<T: Iterator<Item = Result<Action, ()>>>(
    crates: &mut Crates,
    actions: T,
) -> Result<String, ()> {
    for action in actions {
        if let Ok(action) = action {
            crates.move_crates_preserve_order(&action);
        } else {
            return Err(());
        }
    }
    return get_top_crates(crates);
}

fn parse_preamble<T>(lines: &mut T) -> Option<Crates>
where
    T: Iterator<Item = io::Result<String>>,
{
    let header = lines
        .by_ref()
        .take_while(|l| match l {
            Ok(s) => !s.is_empty(),
            _ => false,
        })
        .collect::<Result<Vec<_>, _>>();

    if let Ok(strs) = header {
        let stacks = strs.last().and_then(|s| {
            s.split(' ')
                .map(|id| id.trim())
                .filter_map(|id| if id.is_empty() { None } else { Some(id) })
                .map(|id| id.parse::<i32>())
                .map_ok(|id| Stack(id))
                .collect::<Result<Vec<_>, _>>()
                .ok()
        });

        if let Some(stacks) = stacks {
            let n_rows = strs.len();
            let crate_rows = strs
                .iter()
                .enumerate()
                .take_while(|(i, _)| i + 1 < n_rows)
                .map(|(_, s)| {
                    s.chars()
                        .chunks(4)
                        .into_iter()
                        .map(|c| c.collect::<String>())
                        .map(|c| c.trim().parse::<Crate>().ok())
                        .collect::<Vec<_>>()
                });

            let mut stacks = stacks
                .iter()
                .map(|s| (*s, LinkedList::<Crate>::new()))
                .collect::<Vec<_>>();

            for row in crate_rows {
                for (s, c) in stacks.iter_mut().zip(row) {
                    if let Some(c) = c {
                        s.1.push_back(c)
                    }
                }
            }

            let stacks = HashMap::<Stack, LinkedList<Crate>>::from_iter(stacks);
            return Some(Crates { stacks: stacks });
        }
    }

    return None;
}
