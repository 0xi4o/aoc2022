use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

fn main() {
    part1();
    part2();
}

fn read_lines() -> Lines<BufReader<File>> {
    let file = File::open("input.txt").expect("error reading file");
    let reader = BufReader::new(file);

    reader.lines()
}

fn part1() {
    let lines = read_lines();
    let mut total = 0;
    for line in lines {
        let val = line.unwrap();

        let assignments: Vec<&str> = val.split(',').collect();

        let elf1 = assignments[0];
        let elf2 = assignments[1];

        let e1_assignment: Vec<u64> = elf1.split('-').map(|x| x.parse::<u64>().unwrap()).collect();
        let e2_assignment: Vec<u64> = elf2.split('-').map(|x| x.parse::<u64>().unwrap()).collect();

        let e1_contains_e2 = e1_assignment[0] <= e2_assignment[0] && e1_assignment[1] >= e2_assignment[1];
        let e2_contains_e1 = e2_assignment[0] <= e1_assignment[0] && e2_assignment[1] >= e1_assignment[1];

        if e1_contains_e2 || e2_contains_e1 {
            total += 1;
        }
    }

    println!("{}", total);
}

fn part2() {
    let lines = read_lines();
    let mut total = 0;

    for line in lines {
        let val = line.unwrap();

        let assignments: Vec<&str> = val.split(',').collect();
        let elf1 = assignments[0];
        let elf2 = assignments[1];

        let e1_assignment: Vec<u64> = elf1.split('-').map(|x| x.parse::<u64>().unwrap()).collect();
        let e2_assignment: Vec<u64> = elf2.split('-').map(|x| x.parse::<u64>().unwrap()).collect();

        let overlap = (e2_assignment[0] <= e1_assignment[1] && e2_assignment[0] >= e1_assignment[0]) ||
            (e2_assignment[1] <= e1_assignment[1] && e2_assignment[1] >= e1_assignment[0]) ||
            (e1_assignment[0] <= e2_assignment[1] && e1_assignment[0] >= e2_assignment[0]) ||
            (e1_assignment[1] <= e2_assignment[1] && e1_assignment[1] >= e2_assignment[0]);

        if overlap {
            total += 1
        }
    }

    println!("{}", total);
}