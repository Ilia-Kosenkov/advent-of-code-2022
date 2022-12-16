use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Accumulator {
    current: i32,
    running_max: i32,
}

fn main() {
    let reader = io::BufReader::new(io::stdin());

    let strings = reader.lines().into_iter().filter_map(|l| l.ok());
    let quantities = strings.map(|s| s.parse::<i32>().ok());

    let result = quantities
        .fold(
            Accumulator {
                current: 0i32,
                running_max: 0i32,
            },
            |acc, x| {
                x.map_or_else(
                    || Accumulator {
                        current: 0i32,
                        running_max: acc.running_max.max(acc.current),
                    },
                    |y| Accumulator {
                        current: acc.current + y,
                        ..acc
                    },
                )
            },
        )
        .running_max;

    println!("{}", result);
}
