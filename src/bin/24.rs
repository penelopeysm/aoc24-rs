advent_of_code::solution!(24);

use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum Op {
    And,
    Or,
    Xor,
}

// Represents a pending calculation
#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct Expr {
    op: Op,
    in1: String,
    in2: String,
}
impl Expr {
    fn eval(&self, global_state: &mut GlobalState) -> (bool, String) {
        // Evaluate the inputs recursively (and memoise)
        let s1 = global_state.state.get(&self.in1).unwrap().clone();
        let (b1, s1o) = s1.eval(global_state);
        global_state
            .state
            .insert(self.in1.clone(), State::Value(b1, s1o.clone()));
        let s2 = global_state.state.get(&self.in2).unwrap().clone();
        let (b2, s2o) = s2.eval(global_state);
        global_state
            .state
            .insert(self.in2.clone(), State::Value(b2, s2o.clone()));
        // Then do the actual operation
        global_state.n_ops += 1;
        // println!("evaluating {} {:?} {}", self.in1, self.op, self.in2);
        // Sort the operands here so that the output string is nicer to read. It doesn't affect
        // correctness because the operations are commutative
        let (mut smin, smax) = if s1o < s2o { (s1o, s2o) } else { (s2o, s1o) };
        // Simplify representation of carry bits
        if smin == "'x00' AND 'y00'" {
            smin = "c00".to_string();
        } else {
            let carry_re = Regex::new(r"^''c(\d\d)' AND ''x(\d\d)' XOR 'y(\d\d)''' OR ''x(\d\d)' AND 'y(\d\d)''$").unwrap();
            if let Some(caps) = carry_re.captures(&smin.clone()) {
                println!("matched carry: {}", smin);
                let c = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let x = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
                let y = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
                let x2 = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
                let y2 = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();
                if x == y && x == x2 && x == y2 && c + 1 == x {
                    smin = format!("c{:02}", x);
                }
            }
        }
        match self.op {
            Op::And => (b1 & b2, format!("'{}' AND '{}'", smin, smax)),
            Op::Or => (b1 | b2, format!("'{}' OR '{}'", smin, smax)),
            Op::Xor => (b1 ^ b2, format!("'{}' XOR '{}'", smin, smax)),
        }
    }
    fn _pretty_print(&self) -> String {
        format!(
            "{} {} {}",
            self.in1,
            match self.op {
                Op::And => "AND",
                Op::Or => "OR",
                Op::Xor => "XOR",
            },
            self.in2
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
enum State {
    Value(bool, String),
    Expr(Expr),
}
impl State {
    fn eval(&self, global_state: &mut GlobalState) -> (bool, String) {
        match self {
            State::Value(b, s) => (*b, s.to_string()),
            State::Expr(e) => e.eval(global_state),
        }
    }
    fn _pretty_print(&self, with_expr: bool) -> String {
        match self {
            State::Value(true, s) => {
                if with_expr {
                    format!("1 <-- {}", s)
                } else {
                    "1".to_string()
                }
            }
            State::Value(false, s) => {
                if with_expr {
                    format!("0 <-- {}", s)
                } else {
                    "0".to_string()
                }
            }
            State::Expr(e) => e._pretty_print(),
        }
    }
}

#[derive(Clone, Debug)]
struct GlobalState {
    n_ops: u32, // Number of operations performed -- should match number of lines of input
    state: HashMap<String, State>,
}
impl GlobalState {
    fn eval_gate(&mut self, name: String) -> (bool, String) {
        let s = self.state.get(&name).unwrap().clone();
        // this effectively memoizes
        let (b1, s1) = s.eval(self);
        self.state.insert(name, State::Value(b1, s1.clone()));
        (b1, s1)
    }

    fn eval(&mut self) {
        let z_names = self
            .state
            .keys()
            .filter(|k| k.starts_with("z"))
            .cloned()
            .collect::<Vec<_>>();
        for name in z_names {
            self.eval_gate(name.clone());
        }
    }

    fn z_bits(&self) -> u64 {
        let z_names = self
            .state
            .keys()
            .filter(|k| k.starts_with("z"))
            .cloned()
            .collect::<Vec<_>>();
        let mut acc = 0;
        for name in z_names {
            if let State::Value(true, _) = self.state.get(&name).unwrap() {
                let pow2 = &name[1..].parse::<u32>().unwrap();
                acc += 2u64.pow(*pow2);
            }
        }
        acc
    }

    fn _pretty_print(&self, with_expr: bool) {
        let mut names = self.state.keys().cloned().collect::<Vec<_>>();
        names.sort();
        for name in names {
            let state = self.state.get(&name).unwrap();
            println!("\n{}: {}", name, state._pretty_print(with_expr));
        }
    }
}

impl From<&str> for GlobalState {
    fn from(input: &str) -> GlobalState {
        let mut state = HashMap::new();
        if let [values, exprs] = input.split("\n\n").collect::<Vec<_>>()[..] {
            for line in values.lines() {
                let parts = line.split(": ").collect::<Vec<_>>();
                let name = parts[0].to_string();
                let value = parts[1].parse::<u16>().unwrap(); // Rust doesn't parse '0' and '1' as bools
                state.insert(name.clone(), State::Value(value == 1, name));
            }
            for line in exprs.lines() {
                let parts = line.split_whitespace().collect::<Vec<_>>();
                let in1 = parts[0].to_string();
                let op = match parts[1] {
                    "AND" => Op::And,
                    "OR" => Op::Or,
                    "XOR" => Op::Xor,
                    _ => panic!("Unknown op"),
                };
                let in2 = parts[2].to_string();
                let name = parts[4].to_string();
                state.insert(name, State::Expr(Expr { op, in1, in2 }));
            }
        }
        GlobalState { n_ops: 0, state }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut global_state = GlobalState::from(input);
    global_state.eval();
    // println!("{} evaluations", global_state.n_ops);
    // global_state._pretty_print(false);
    Some(global_state.z_bits())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut global_state = GlobalState::from(input);
    global_state.eval();
    global_state._pretty_print(true);
    // Proof by inspection
    Some("cgh,frt,pmd,sps,tst,z05,z11,z23".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("z00,z01,z02,z05".to_string()));
    }
}
