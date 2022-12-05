use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

fn read_lines(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).expect("error reading file");
    let reader = BufReader::new(file);

    reader.lines()
}

fn main() {
    part1();
    part2();
}

fn convert_lines_to_stack() -> Vec<Vec<String>> {
    let stack_lines = read_lines("stacks.txt");
    let mut indices: Vec<usize> = Vec::new();
    let mut stacks: Vec<Vec<String>> = Vec::new();
    let mut stack_lines: Vec<_> = stack_lines.map(|line| { line.unwrap() }).collect();
    stack_lines.reverse();
    let stack_numbers: Vec<&str> = stack_lines[0].split("").collect();

    for (i, val) in stack_numbers.iter().enumerate() {
        if !val.trim().is_empty() {
            indices.push(i - 1);
        }
    }

    stack_lines.remove(0);
    for index in indices {
        let mut stack: Vec<String> = Vec::new();
        for (_, line) in stack_lines.iter().enumerate() {
            let chars: Vec<&str> = line.split("").collect();
            let crate_id = String::from(chars[index + 1]);
            if !crate_id.trim().is_empty() {
                stack.push(crate_id);
            }
        }
        stacks.push(stack);
    }

    stacks
}

fn part1() {
    let mut stacks = convert_lines_to_stack();
    let instructions = read_lines("input.txt");

    for line in instructions {
        let val = line.unwrap();
        let instruction: Vec<&str> = val.split_whitespace().collect();
        let amount = instruction[1].parse::<usize>().unwrap();
        let from_stack = instruction[3].parse::<usize>().unwrap();
        let to_stack = instruction[5].parse::<usize>().unwrap();

        let start_stack = &mut stacks[from_stack - 1];
        let mut buffer: Vec<String> = Vec::new();

        for _ in 0..amount {
            let val = start_stack.pop();
            buffer.push(val.unwrap());
        }

        let _ = &mut stacks[to_stack - 1].append(&mut buffer);
    }

    for stack in &stacks {
        print!("{:?}", stack[stack.len() - 1]);
    }
}

fn part2() {
    let mut stacks = convert_lines_to_stack();
    let instructions = read_lines("input.txt");

    for line in instructions {
        let val = line.unwrap();
        let instruction: Vec<&str> = val.split_whitespace().collect();
        let amount = instruction[1].parse::<usize>().unwrap();
        let from_stack = instruction[3].parse::<usize>().unwrap();
        let to_stack = instruction[5].parse::<usize>().unwrap();

        let start_stack = &mut stacks[from_stack - 1];
        let final_amount = start_stack.len().saturating_sub(amount);
        let mut buffer = start_stack.split_off(final_amount);
        let _ = &mut stacks[to_stack - 1].append(&mut buffer);
    }

    println!();

    for stack in &stacks {
        print!("{:?}", stack[stack.len() - 1]);
    }
}