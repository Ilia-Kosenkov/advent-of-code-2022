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
    opponent_hand: OpponentHand,
    round_result: RoundResult,
}

impl KnownRound {
    fn play(&self) -> RoundResult {
        use Hand::*;
        use RoundResult::*;

        match (self.my_hand.0, self.opponent_hand.0) {
            (Rock, Paper) => Loss,
            (Rock, Scissors) => Win,

            (Paper, Rock) => Win,
            (Paper, Scissors) => Loss,

            (Scissors, Rock) => Loss,
            (Scissors, Paper) => Win,

            _ => Draw,
        }
    }
}

impl UnknownRound {
    fn pick_hand(&self) -> Hand {
        use Hand::*;
        use RoundResult::*;

        match self.round_result {
            Win => match self.opponent_hand.0 {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
            Loss => match self.opponent_hand.0 {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
            Draw => self.opponent_hand.0,
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

impl FromStr for KnownRound {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s[..1].parse::<OpponentHand>().and_then(|op_h| {
            s[2..].parse::<MyHand>().and_then(|my_h| {
                Ok(KnownRound {
                    opponent_hand: op_h,
                    my_hand: my_h,
                })
            })
        })
    }
}

impl FromStr for UnknownRound {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s[..1].parse::<OpponentHand>().and_then(|op_h| {
            s[2..].parse::<RoundResult>().and_then(|res| {
                Ok(UnknownRound {
                    opponent_hand: op_h,
                    round_result: res,
                })
            })
        })
    }
}

fn main() {
    let reader = io::BufReader::new(io::stdin());
    let rounds = reader.lines().filter_map(|l| {
        let l = l.map_err(|_| ());
        l.and_then(
            |s| match (s.parse::<KnownRound>(), s.parse::<UnknownRound>()) {
                (Ok(known), Ok(unknown)) => Ok((known, unknown)),
                _ => Err(()),
            },
        )
        .ok()
    });

    let scores = rounds.map(|r| {
        (
            r.0.play() as i32 + r.0.my_hand.0 as i32,
            r.1.pick_hand() as i32 + r.1.round_result as i32,
        )
    });

    let total_score = scores.fold((0, 0), |(acc1, acc2), (r1, r2)| (acc1 + r1, acc2 + r2));

    println!("{}", total_score.0);
    println!("{}", total_score.1);
}
