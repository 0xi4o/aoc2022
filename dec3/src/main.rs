use std::fs::File;
use std::io::{BufReader, BufRead, Lines};
use array_tool::vec::Intersect;

fn main() {
    // part1();
    part2();
}

fn read_lines() -> Lines<BufReader<File>> {
    let file = File::open("input.txt").expect("error reading file");
    let reader = BufReader::new(file);

    reader.lines()
}

fn find_intersection(val: &str) -> char {
    let len = val.len();

    // split the rucksack into two compartments
    let comp1 = &val[0..(len / 2)];
    let comp2 = &val[(len / 2)..len];

    // turn slices into vectors so we can run intersection on the vectors
    let vec1: Vec<char> = comp1.chars().collect();
    let vec2: Vec<char> = comp2.chars().collect();

    let intersection = vec1.intersect(vec2);

    intersection[0]
}

fn part1() {
    let lines = read_lines();
    let mut total = 0;
    for line in lines {
        let val = line.unwrap();
        let item_type = find_intersection(&val);

        if item_type.is_lowercase() {
            // get ascii value of the lowercase letter and subtract 96 from it to get a value between 1 and 26
            total += item_type as u32 - 96;
        } else {
            // get ascii value of the uppercase letter and subtract 38 (65 - 27) from it to get a value between 27 and 52
            total += item_type as u32 - 38;
        }
    }

    println!("{}", total);
}

fn part2() {
    let lines = read_lines();
    let lines_1 = lines.enumerate();

    let mut group: Vec<String> = Vec::new();
    let mut total = 0;

    for (index, line) in lines_1 {
        let val = line.unwrap();
        group.push(val);

        // do this for every 3 lines which equals a group of 3 elves according to the puzzle
        if (index + 1) % 3 == 0 {
            // turn the strings into vectors
            let vec1: Vec<char> = group[0].chars().collect();
            let vec2: Vec<char> = group[1].chars().collect();
            let vec3: Vec<char> = group[2].chars().collect();

            // find the intersection of the three vectors
            let ins = vec1.intersect(vec2.intersect(vec3));
            let item_type = ins[0];

            if item_type.is_lowercase() {
                // get ascii value of the lowercase letter and subtract 96 from it to get a value between 1 and 26
                total += item_type as u32 - 96;
            } else {
                // get ascii value of the uppercase letter and subtract 38 (65 - 27) from it to get a value between 27 and 52
                total += item_type as u32 - 38;
            }

            // empty the group
            group.drain(..);
        }
    }

    println!("{}", total);
}