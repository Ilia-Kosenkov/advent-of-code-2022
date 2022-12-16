use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct RunningMax(i32, i32, i32);

impl RunningMax {
    fn update(&self, new_value : i32) -> Self {
        if self.2 < new_value {
            RunningMax(self.0, self.1, new_value)
        }
        else if self.1 < new_value {
            RunningMax(self.0, new_value, self.2)
        }
        else if self.0 < new_value {
            RunningMax(new_value, self.1, self.2)
        }
        else {
            self.clone()
        }
    }

    fn sum(&self) -> i32 {
        self.0 + self.1 + self.2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Accumulator {
    current: i32,
    running_max : RunningMax,
}

fn main() {
    let reader = io::BufReader::new(io::stdin());

    let strings = reader.lines().into_iter().filter_map(|l| l.ok());
    let quantities = strings.map(|s| s.parse::<i32>().ok());

    let result = quantities
        .fold(
            Accumulator {
                current: 0i32,
                running_max: std::default::Default::default(),
            },
            |acc, x| {
                x.map_or_else(
                    || Accumulator {
                        current: 0i32,
                        running_max: acc.running_max.update(acc.current),
                    },
                    |y| Accumulator {
                        current: acc.current + y,
                        ..acc
                    },
                )
            },
        )
        .running_max;

    println!("{}", result.2);
    println!("{}", result.sum())
}
