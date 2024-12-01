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

Download the input files for a specific day:

```sh
cargo download {day}
cargo scaffold {day}
```

Edit the file to solve the problem.
Note that the filepath is padded with zeros, so the file for day 1 is `src/bin/01.rs`.

```sh
nvim src/bin/{day}.rs
```

Run the solution:

```sh
cargo solve {day}
```

Submit the solution:

```sh
cargo solve {day} --submit
```
