use std::{
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RoundResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct MyHand(Hand);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct OpponentHand(Hand);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct KnownRound {
    opponent_hand: OpponentHand,
    my_hand: MyHand,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct UnknownRound {
    opponent_hand : OpponentHand,
    round_result : RoundResult
}

impl KnownRound {
    fn play(&self) -> RoundResult {
        use Hand::*;
        use RoundResult::*;

        match (self.my_hand.0, self.opponent_hand.0) {
            (Rock, Paper) => RoundResult::Loss,
            (Rock, Scissors) => RoundResult::Win,

            (Paper, Rock) => Win,
            (Paper, Scissors) => Loss,

            (Scissors, Rock) => Loss,
            (Scissors, Paper) => Win,

            _ => Draw
        }
    }
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

impl FromStr for RoundResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .next()
            .and_then(|ch| match ch {
                'X' => Some(RoundResult::Loss),
                'Y' => Some(RoundResult::Draw),
                'Z' => Some(RoundResult::Win),
                _ => None,
            })
            .ok_or(())
    }
}

fn main() {
    let reader = io::BufReader::new(io::stdin());
    let rounds = reader
        .lines()
        .filter_map(|l| {
            l.ok().and_then(|s| {
                s[..1].parse::<OpponentHand>().ok().and_then(|op_h| {
                    s[2..].parse::<MyHand>().ok().and_then(|my_h| {
                        Some(KnownRound {
                            opponent_hand: op_h,
                            my_hand: my_h,
                        })
                    })
                })
            })
        });

    let scores = rounds.map(|r| r.play() as i32 + r.my_hand.0 as i32);

    let total_score : i32 = scores.sum();

    println!("{}", total_score);
}
