use std::{
    collections::LinkedList,
    io::{self, BufRead},
    str::FromStr,
};

use itertools::Itertools;

fn unit<T>(_: T) {}

#[derive(Debug, Clone)]
struct Monkey {
    id: i32,
    items: LinkedList<i32>,
    operation: BinOperation,
    div_condition: i32,
    throw_to: ThrowTo,
}

#[derive(Debug, Clone, Copy)]
struct ThrowTo {
    on_true: i32,
    on_false: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OpTarget {
    Constant(i32),
    Variable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OpType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BinOperation {
    lhs: OpTarget,
    rhs: OpTarget,
    op: OpType,
}

impl FromStr for OpType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(OpType::Addition),
            "-" => Ok(OpType::Subtraction),
            "*" => Ok(OpType::Multiplication),
            "/" => Ok(OpType::Division),
            _ => Err(()),
        }
    }
}

impl FromStr for OpTarget {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if let Ok(val) = s.parse::<i32>() {
            Ok(OpTarget::Constant(val))
        } else {
            Ok(OpTarget::Variable)
        }
    }
}

impl FromStr for BinOperation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split_whitespace();
        let lhs = iter.next().ok_or(())?.parse::<OpTarget>()?;
        let op = iter.next().ok_or(())?.parse::<OpType>()?;
        let rhs = iter.next().ok_or(())?.parse::<OpTarget>()?;

        Ok(BinOperation {
            lhs: lhs,
            rhs: rhs,
            op: op,
        })
    }
}

impl OpTarget {
    fn eval(&self, old: i32) -> i32 {
        match self {
            OpTarget::Variable => old,
            OpTarget::Constant(val) => *val,
        }
    }
}

impl OpType {
    fn eval(&self, lhs: i32, rhs: i32) -> i32 {
        match self {
            OpType::Addition => lhs + rhs,
            OpType::Subtraction => lhs - rhs,
            OpType::Multiplication => lhs * rhs,
            OpType::Division => lhs / rhs,
        }
    }
}

impl BinOperation {
    fn eval(&self, old: i32) -> i32 {
        self.op.eval(self.lhs.eval(old), self.rhs.eval(old))
    }
}

fn main() {
    let reader = io::BufReader::new(io::stdin());

    let input = reader.lines().chunks(7);

    let parsed_input = input.into_iter().map(|x| parse_input(x.take(6)));

    dbg! {parsed_input.collect_vec()};
}

fn parse_input<T, TErr>(mut input: T) -> Result<Monkey, ()>
where
    T: Iterator<Item = Result<String, TErr>>,
{
    let monkey_id = input.next().ok_or(())?.map_err(unit)?;

    let monkey_id = monkey_id[7..(monkey_id.len() - 1)]
        .parse::<i32>()
        .map_err(unit)?;

    let items = input.next().ok_or(())?.map_err(unit)?;
    let items = items[17..]
        .split(',')
        .map(|s| s.trim().parse::<i32>().map_err(unit))
        .collect::<Result<LinkedList<_>, _>>()?;

    let op = input.next().ok_or(())?.map_err(unit)?[18..].parse::<BinOperation>()?;

    let div_condition = input.next().ok_or(())?.map_err(unit)?[20..]
        .trim()
        .parse::<i32>()
        .map_err(unit)?;

    let on_true = input.next().ok_or(())?.map_err(unit)?[28..]
        .trim()
        .parse::<i32>()
        .map_err(unit)?;
    let on_false = input.next().ok_or(())?.map_err(unit)?[29..]
        .trim()
        .parse::<i32>()
        .map_err(unit)?;

    Ok(Monkey {
        id: monkey_id,
        items: items,
        operation: op,
        div_condition: div_condition,
        throw_to: ThrowTo {
            on_true: on_true,
            on_false: on_false,
        },
    })
}
