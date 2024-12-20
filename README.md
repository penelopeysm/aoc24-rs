# aoc24-rs

Advent of Code 2024 in Rust

(except for Day 19, which I golfed in Python)

Repository generated from [the template repository](https://github.com/fspoettel/advent-of-code-rust)

## One-time setup

Install the aoc-cli crate:

```sh
cargo install aoc-cli
```

Then set the session cookie as described [here](https://github.com/fspoettel/advent-of-code-rust?tab=readme-ov-file#configure-aoc-cli-integration).


## Usage

Note that in all of the following examples, the day is left-padded with zeros, so day 1 is `01` not `1`.

Download the input files for a specific day:

```sh
cargo download {day}
cargo scaffold {day}
```

Edit the file to solve the problem.

```sh
nvim src/bin/{day}.rs
```

Test the solution against the example input.
(The example input is part of the puzzle text, so can't be automatically extracted.)

```sh
# first copy the input to data/examples/{day}.txt
cargo test --bin {day}
```

Run the solution (using the release flag if you want to feel good about performance):

```sh
cargo solve {day} [--release]
```

Submit the solution:

```sh
cargo solve {day} [--release] --submit {part}
```

Benchmark:

```sh
cargo time {day}
```
