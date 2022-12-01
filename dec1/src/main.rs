// use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

fn main() {
    // let mut file = File::open("input.txt").expect("Failed to open input.txt");
    // let mut data = String::new();
    //
    // file.read_to_string(&mut data).expect("error reading file");
    //
    // println!("data: {}", data);

    let lines = read_lines();
    let mut elf_cal: Vec<u64> = Vec::new();
    let mut total_cal: u64 = 0;

    for line in lines {
        let val = line.unwrap();
        if val != "" {
            let num: u64 = val.parse().unwrap();
            total_cal += num;
        } else {
            elf_cal.push(total_cal);
            total_cal = 0;
        }
    }

    // let max = elf_cal.iter().max().unwrap();
    // println!("{:?}", max);

    elf_cal.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", elf_cal);
}

fn read_lines() -> Lines<BufReader<File>> {
    let file = File::open("input.txt").expect("error reading file");
    let reader = BufReader::new(file);

    reader.lines()
}
