use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

fn main() {
    let lines = read_lines();
    let mut total_score: u64 = 0;
    for line in lines {
        let val = line.unwrap();
        let mut parts = val.split_whitespace();
        let (opp, round_end) = (parts.next().unwrap(), parts.next().unwrap());
        let result = match round_end {
            "X" => "L",
            "Y" => "D",
            "Z" => "W",
            _ => ""
        };
        let res = play_round(opp, &result);
        let score = cal_score(res, &result);
        total_score += score;
    }
    println!("{}", total_score);
}

fn read_lines() -> Lines<BufReader<File>> {
    let file = File::open("input.txt").expect("error reading file");
    let reader = BufReader::new(file);

    reader.lines()
}

fn play_round<'a>(opp: &'a str, result: &'a str) -> &'a str {
    if opp == "A" && result == "W" {
        "Y"
    } else if opp == "A" && result == "L" {
        "Z"
    } else if opp == "A" && result == "D" {
        "X"
    } else if opp == "B" && result == "W" {
        "Z"
    } else if opp == "B" && result == "L" {
        "X"
    } else if opp == "B" && result == "D" {
        "Y"
    } else if opp == "C" && result == "W" {
        "X"
    } else if opp == "C" && result == "L" {
        "Y"
    } else {
        "Z"
    }

}

fn cal_score(res: &str, result: &str) -> u64 {
    let res_score = match res {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    };

    let result_score = match result {
        "L" => 0,
        "D" => 3,
        "W" => 6,
        _ => 0
    };

    res_score + result_score
}