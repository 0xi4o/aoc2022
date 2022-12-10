#![feature(exclusive_range_pattern)]
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
enum InstructionType {
    addx(i32),
    noop
}

fn instructions(input: &str) -> Vec<InstructionType> {
    input
        .split("\n")
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let ins_type = InstructionType::from_str(parts[0]).unwrap();
            match ins_type {
                InstructionType::addx(_) => InstructionType::addx(parts[1].parse::<i32>().unwrap()),
                InstructionType::noop => InstructionType::noop
            }
        })
        .collect::<Vec<InstructionType>>()
}

pub fn part1(input: &str) -> i32 {
    let mut x: i32 = 1;
    let instruction_set = instructions(input);
    let breakpoints = [20, 60, 100, 140, 180, 220];
    let mut cycle = 0;
    let mut total_signal_strength = 0;

    for ins in instruction_set.iter() {
        match ins {
            InstructionType::addx(num) => {
                for _ in 1..=2 {
                    cycle += 1;

                    if breakpoints.contains(&cycle) {
                        total_signal_strength += cycle * x;
                    }
                }
                x += *num;
            },
            InstructionType::noop => {
                cycle += 1;

                if breakpoints.contains(&cycle) {
                    total_signal_strength += cycle * x;
                }
            }
        }
    }

    total_signal_strength
}

fn get_coords(cycle: &u32) -> (usize, usize) {
    match *cycle - 1 {
        (0..40) => ((*cycle - 1) as usize, 0 as usize),
        (40..80) => ((*cycle - 41) as usize, 1 as usize),
        (80..120) => ((*cycle - 81) as usize, 2 as usize),
        (120..160) => ((*cycle - 121) as usize, 3 as usize),
        (160..200) => ((*cycle - 161) as usize, 4 as usize),
        (200..240) => ((*cycle - 201) as usize, 5 as usize),
        (240..=u32::MAX) => (240, 240)
    }
}

pub fn part2(input: &str) -> String {
    let mut x: i32 = 1;
    let instruction_set = instructions(input);
    let mut crt = [[""; 40]; 6];
    let mut sprite_pos: i32 = 0;
    let mut cycle: u32 = 0;

    for ins in instruction_set.iter() {
        match ins {
            InstructionType::addx(num) => {
                for _ in 1..=2 {
                    cycle += 1;

                    let coords = get_coords(&cycle);
                    if (sprite_pos..=(sprite_pos + 2)).contains(&(coords.0 as i32)) {
                        crt[coords.1][coords.0] = "#";
                    } else {
                        crt[coords.1][coords.0] = ".";
                    }
                }
                x += *num;
                sprite_pos = x - 1;
            },
            InstructionType::noop => {
                cycle += 1;

                let coords = get_coords(&cycle);
                if (sprite_pos..=(sprite_pos + 2)).contains(&(coords.0 as i32)) {
                    crt[coords.1][coords.0] = "#";
                } else {
                    crt[coords.1][coords.0] = ".";
                }
            }
        }
    }

    crt
        .iter()
        .map(|line| line.join("").to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 13140);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let output = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(result, output);
    }
}
