# My (late) attempt at [AdventOfCode2022](https://adventofcode.com/2022)

The task are solved in Rust.
So far, for tasks that have lengthy inputs, the goal is to compute everything in one go, i.e. write a pipeline that reads likes from `stdin` and produces an output. No `.collect()`. External dependencies are also minimized (and mostly avoided).

How to run? Get your input puzzle into, e.g. a file named `input.txt`, and send it to the `stdin` of the binary. When using `pwsh`, it can be done like this

```pwsh
cat input.txt | cargo run
```

- [Task 1](./01)
- [Task 2](./02)
