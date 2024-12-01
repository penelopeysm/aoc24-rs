# aoc24-rs

Advent of Code 2024 in Rust

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
(This can't be automatically downloaded because the example input is part of the puzzle text.)

```sh
# first download the input to data/examples/{day}.txt
cargo test --bin {day}
```

Run the solution:

```sh
cargo solve {day}
```

Submit the solution:

```sh
cargo solve {day} --submit {part}
```
