use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

enum Result {
    W,
    L,
    D
}

fn main() {
    let lines = read_lines();
    let mut total_score: u64 = 0;
    for line in lines {
        let val = line.unwrap();
        let mut parts = val.split_whitespace();
        let (opp, res) = (parts.next().unwrap(), parts.next().unwrap());
        let result = play_round(opp, res);
        let score = cal_score(res, result);
        total_score += score;
    }
    println!("{}", total_score);
}

fn read_lines() -> Lines<BufReader<File>> {
    let file = File::open("input.txt").expect("error reading file");
    let reader = BufReader::new(file);

    reader.lines()
}

fn play_round(opp: &str, res:&str) -> Result {
    if opp == "A" && res == "X" {
        Result::D
    } else if opp == "A" && res == "Y" {
        Result::W
    } else if opp == "A" && res == "Z" {
        Result::L
    } else if opp == "B" && res == "X" {
        Result::L
    } else if opp == "B" && res == "Y" {
        Result::D
    } else if opp == "B" && res == "Z" {
        Result::W
    } else if opp == "C" && res == "X" {
        Result::W
    } else if opp == "C" && res == "Y" {
        Result::L
    } else {
        Result::D
    }

}

fn cal_score(res: &str, result: Result) -> u64 {
    let res_score = match res {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    };

    let result_score = match result {
        Result::L => 0,
        Result::D => 3,
        Result::W => 6
    };

    res_score + result_score
}