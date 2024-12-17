advent_of_code::solution!(17);

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Adv0,
    Bxl1,
    Bst2,
    Jnz3,
    Bxc4,
    Out5,
    Bdv6,
    Cdv7,
}

impl From<u64> for Opcode {
    fn from(value: u64) -> Self {
        match value {
            0 => Opcode::Adv0,
            1 => Opcode::Bxl1,
            2 => Opcode::Bst2,
            3 => Opcode::Jnz3,
            4 => Opcode::Bxc4,
            5 => Opcode::Out5,
            6 => Opcode::Bdv6,
            7 => Opcode::Cdv7,
            _ => panic!("Invalid opcode"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

// I do miss fromEnum and toEnum
impl From<u64> for Operand {
    fn from(value: u64) -> Self {
        match value {
            0 => Operand::Zero,
            1 => Operand::One,
            2 => Operand::Two,
            3 => Operand::Three,
            4 => Operand::Four,
            5 => Operand::Five,
            6 => Operand::Six,
            7 => Operand::Seven,
            _ => panic!("Invalid operand"),
        }
    }
}

impl From<Operand> for u64 {
    fn from(value: Operand) -> Self {
        match value {
            Operand::Zero => 0,
            Operand::One => 1,
            Operand::Two => 2,
            Operand::Three => 3,
            Operand::Four => 4,
            Operand::Five => 5,
            Operand::Six => 6,
            Operand::Seven => 7,
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: Opcode,
    operand: Operand,
}

#[derive(Debug, Clone)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    instructions: Vec<Instruction>,
    pointer: usize,
    // Store the raw programme for part 2
    programme_numbers: Vec<u64>,
}

enum Stdout {
    Nothing,
    Number(u64),
    Terminated,
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let lines = value.lines().collect::<Vec<&str>>();
        let a = lines[0].split_whitespace().last().unwrap().parse().unwrap();
        let b = lines[1].split_whitespace().last().unwrap().parse().unwrap();
        let c = lines[2].split_whitespace().last().unwrap().parse().unwrap();
        let programme_numbers = lines[4]
            .split_whitespace()
            .last()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u64>>();
        let instructions = programme_numbers
            .clone()
            .chunks(2)
            .map(|x| Instruction {
                opcode: Opcode::from(x[0]),
                operand: Operand::from(x[1]),
            })
            .collect::<Vec<Instruction>>();
        Computer {
            a,
            b,
            c,
            instructions,
            pointer: 0,
            programme_numbers,
        }
    }
}

impl Computer {
    fn combo_operand(&mut self, opd: Operand) -> u64 {
        match opd {
            Operand::Zero => 0,
            Operand::One => 1,
            Operand::Two => 2,
            Operand::Three => 3,
            Operand::Four => self.a,
            Operand::Five => self.b,
            Operand::Six => self.c,
            Operand::Seven => panic!("The instructions said this should not appear"),
        }
    }

    fn step(&mut self) -> Stdout {
        match self.instructions.get(self.pointer) {
            None => Stdout::Terminated,
            Some(i) => {
                // Ack
                match i.opcode {
                    Opcode::Adv0 => {
                        self.a >>= self.combo_operand(i.operand);
                        self.pointer += 1;
                        Stdout::Nothing
                    }
                    Opcode::Bxl1 => {
                        self.b ^= u64::from(i.operand);
                        self.pointer += 1;
                        Stdout::Nothing
                    }
                    Opcode::Bst2 => {
                        self.b = self.combo_operand(i.operand) % 8;
                        self.pointer += 1;
                        Stdout::Nothing
                    }
                    Opcode::Jnz3 => {
                        if self.a == 0 {
                            self.pointer += 1;
                        } else {
                            assert!(i.operand as usize % 2 == 0);
                            self.pointer = i.operand as usize / 2;
                        }
                        Stdout::Nothing
                    }
                    Opcode::Bxc4 => {
                        self.b ^= self.c;
                        self.pointer += 1;
                        Stdout::Nothing
                    }
                    Opcode::Out5 => {
                        self.pointer += 1;
                        Stdout::Number(self.combo_operand(i.operand) % 8)
                    }
                    Opcode::Bdv6 => {
                        self.b = self.a >> self.combo_operand(i.operand);
                        self.pointer += 1;
                        Stdout::Nothing
                    }
                    Opcode::Cdv7 => {
                        self.c = self.a >> self.combo_operand(i.operand);
                        self.pointer += 1;
                        Stdout::Nothing
                    }
                }
            }
        }
    }

    fn step_until_terminated(&mut self) -> Vec<u64> {
        let mut outputs = Vec::new();
        loop {
            match self.step() {
                Stdout::Nothing => {}
                Stdout::Number(x) => {
                    outputs.push(x);
                }
                Stdout::Terminated => break,
            }
        }
        outputs
    }

    fn reset_with_a(&mut self, a: u64) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.pointer = 0;
    }

    fn get_one_output_with_a(&mut self, a: u64) -> u64 {
        self.reset_with_a(a);
        loop {
            match self.step() {
                Stdout::Nothing => {}
                Stdout::Number(x) => {
                    return x;
                }
                Stdout::Terminated => {
                    panic!("This should not happen with the given input, because there isn't a jump instruction until the end of the input");
                }
            }
        }
    }

    fn find_solution(&mut self, remaining_rev_outputs: &[u64], a_so_far: u64) -> Option<u64> {
        // Pluck off the first one
        let rev_output = remaining_rev_outputs[0];
        // Find out what the lowest octal digit is
        for i in 0..8 {
            let output = self.get_one_output_with_a(a_so_far + i);
            // If we found a valid octal digit...
            if output == rev_output {
                // and there are no more outputs to match, it means we matched the entire vector!
                // We can directly return
                if remaining_rev_outputs.len() == 1 {
                    return Some(a_so_far + i);
                }
                // Otherwise, we need to add that to the list of octal digits
                let new_a_so_far = (a_so_far + i) * 8;
                // And recurse with the remaining outputs we need to match
                let new_remaining_rev_outputs = &remaining_rev_outputs[1..];
                let result = self.find_solution(new_remaining_rev_outputs, new_a_so_far);
                if result.is_some() {
                    return result;
                }
            }
        }
        // This only happens if there are no solutions
        None
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::from(input);
    Some(
        computer
            .step_until_terminated()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(","),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut computer = Computer::from(input);
    let mut reverse_outputs = computer.programme_numbers.clone();
    reverse_outputs.reverse();
    computer.find_solution(&reverse_outputs, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
