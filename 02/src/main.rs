use std::{
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct MyHand(Hand);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct OpponentHand(Hand);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Round {
    opponent_hand: OpponentHand,
    my_hand: MyHand,
}

impl FromStr for OpponentHand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .next()
            .and_then(|ch| match ch {
                'A' => Some(Self(Hand::Rock)),
                'B' => Some(Self(Hand::Paper)),
                'C' => Some(Self(Hand::Scissors)),
                _ => None,
            })
            .ok_or(())
    }
}

impl FromStr for MyHand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .next()
            .and_then(|ch| match ch {
                'X' => Some(Self(Hand::Rock)),
                'Y' => Some(Self(Hand::Paper)),
                'Z' => Some(Self(Hand::Scissors)),
                _ => None,
            })
            .ok_or(())
    }
}

fn main() {
    let reader = io::BufReader::new(io::stdin());
    let result = reader
        .lines()
        .filter_map(|l| {
            l.ok().and_then(|s| {
                s[..1].parse::<OpponentHand>().ok().and_then(|op_h| {
                    s[2..].parse::<MyHand>().ok().and_then(|my_h| {
                        Some(Round {
                            opponent_hand: op_h,
                            my_hand: my_h,
                        })
                    })
                })
            })
        })
        .collect::<Vec<Round>>();

    dbg! { result };
}
