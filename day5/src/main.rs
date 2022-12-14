use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Move {
    quantity: i32,
    pickup: i32,
    destination: i32,
}

impl Move {
    fn new(instruction: &str) -> Move {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let captured = PATTERN.captures(instruction).unwrap();
        Move {
            quantity: captured[1].parse::<i32>().unwrap(),
            pickup: captured[2].parse::<i32>().unwrap() - 1,
            destination: captured[3].parse::<i32>().unwrap() - 1,
        }
    }
}

#[derive(Clone, Debug)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn push(&mut self, letter: char) {
        self.crates.insert(0, letter);
    }
    fn pop(&mut self) -> char {
        self.crates.pop().unwrap()
    }
    fn append(&mut self, letter: char) {
        self.crates.push(letter);
    }
    fn get_last(&self) -> char {
        self.crates[self.crates.len() - 1]
    }
    fn slice(&mut self, end: usize) -> Vec<char> {
        self.crates.drain(self.crates.len() - end..).collect()
    }
    fn extend(&mut self, vec: Vec<char>) {
        self.crates.extend(vec);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error opening the file.");
    let (mut stacks, moves) = get_stack_and_instructions(input);
    let crate_mover_9000_stack = follow_instructions(&moves, &stacks);
    let crate_mover_9001_stack = follow_new_instructions(&moves, &mut stacks);
    let result = get_last_crate(&crate_mover_9000_stack);
    let part2 = get_last_crate(&crate_mover_9001_stack);
    println!("{result}");
    println!("Part 2: {part2}");
}

fn get_stack_and_instructions(input: String) -> (Vec<Stack>, Vec<Move>) {
    let mut stack_collector: Vec<&str> = Vec::new();
    let mut instructions: Vec<Move> = Vec::new();
    let mut end_of_stack = false;
    for line in input.lines() {
        if line.trim().is_empty() {
            end_of_stack = true;
        }
        if !end_of_stack {
            stack_collector.push(line);
        }
        if !line.trim().is_empty() && end_of_stack {
            instructions.push(Move::new(&line));
        }
    }
    (build_stack(stack_collector), instructions)
}

fn build_stack(stack_collector: Vec<&str>) -> Vec<Stack> {
    let n_stacks = stack_collector[stack_collector.len() - 1]
        .split_whitespace()
        .count();
    let mut stacks: Vec<Stack> = vec![Stack { crates: vec![] }; n_stacks];
    for line in &stack_collector[0..stack_collector.len() - 1] {
        for i in 0..n_stacks {
            let character = line.chars().nth(4 * i + 1).unwrap();
            if !character.is_whitespace() {
                stacks[i].push(character)
            }
        }
    }
    stacks
}

fn follow_instructions(instructions: &Vec<Move>, stacks: &Vec<Stack>) -> Vec<Stack> {
    let mut stacks = stacks.clone();
    for instruction in instructions {
        for _ in 0..instruction.quantity {
            let picked = stacks[instruction.pickup as usize].pop();
            stacks[instruction.destination as usize].append(picked);
        }
    }
    stacks
}

fn follow_new_instructions(instructions: &Vec<Move>, stacks: &mut Vec<Stack>) -> Vec<Stack> {
    for instruction in instructions {
        let picked = stacks[instruction.pickup as usize].slice(instruction.quantity as usize);
        stacks[instruction.destination as usize].extend(picked);
    }
    stacks.to_vec()
}

fn get_last_crate(stacks: &Vec<Stack>) -> String {
    let mut result = String::new();
    for stack in stacks {
        result.push(stack.get_last());
    }
    result
}
