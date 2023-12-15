# Advent of Code 2023
Each directory contains an answer to the coding challenges for 2023's [Advent of Code](https://adventofcode.com/) event.
I'm using these challenges to learn Rust (just for fun).

## Running
Build each solution with `cargo` and read input from stdin.

```bash
cargo run < input
```

At some point I started using tests to keep my code for part 1 and part 2 both working. To run those ones, use the `--nocapture` flag to make sure the output is written to stdout.
```bash
cargo test -- --nocapture
```

## Goals
- come up with working solutions
- try out Rust features
- run into some borrow checker edge cases that force me to go deeper into Rust

## Comments
- Day 5 part 2 gave me some trouble, so I skipped it. I intend to come back to it at some point.