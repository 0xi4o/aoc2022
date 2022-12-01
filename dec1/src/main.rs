// use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

fn main() {
    let lines = read_lines();
    let mut all_elf_cal: Vec<u64> = Vec::new();
    let mut elf_total_cal: u64 = 0;

    for line in lines {
        let val = line.unwrap();
        if val != "" {
            let num: u64 = val.parse().unwrap();
            elf_total_cal += num;
        } else {
            all_elf_cal.push(elf_total_cal);
            elf_total_cal = 0;
        }
    }

    let max = all_elf_cal.iter().max().unwrap();
    println!("{:?}", max);

    all_elf_cal.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top3 = all_elf_cal[0] + all_elf_cal[1] + all_elf_cal[2];
    println!("{:?}", top3);
}

fn read_lines() -> Lines<BufReader<File>> {
    let file = File::open("input.txt").expect("error reading file");
    let reader = BufReader::new(file);

    reader.lines()
}
