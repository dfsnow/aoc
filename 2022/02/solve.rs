use std::env::args;
use std::io::{stdin, Read};

fn conv_to_scores(c: char) -> i64 {
    match c {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => todo!(),
    }
}

fn get_outcome(them: i64, me: i64) -> i64 {
    if ((me - them) == 1) || ((me - them) == -2) {
        6
    } else if me == them {
        3
    } else {
        0
    }
}

fn rev_outcome(them: i64, outcome: i64) -> i64 {
    let score = match outcome {
        1 => 0,
        2 => 3,
        3 => 6,
        _ => todo!(),
    };

    let play = match score {
        0 => { if them - 1 < 1 { 3 } else { them - 1 } }
        3 => them,
        6 => { if them + 1 > 3 { 1 } else { them + 1 } }
        _ => todo!(),
    };
    score + play
}

fn read_inputs() -> Vec<(i64, i64)> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    input
        .lines()
        .map(|round| {
            let them = conv_to_scores(round.chars().nth(0).unwrap());
            let me = conv_to_scores(round.chars().nth(2).unwrap());
            (them, me)
        })
        .collect()
}

fn q1() {
    let total: i64 = read_inputs()
        .iter()
        .map(|(them, me)| me + get_outcome(*them, *me))
        .sum();
    println!("{}", total);
}

fn q2() {
    let total: i64 = read_inputs()
        .iter()
        .map(|(them, outcome)| rev_outcome(*them, *outcome))
        .sum();
    println!("{}", total);
}

fn main() {
    let args: Vec<String> = args().collect();
    if args[1] == "1" {
        q1();
    } else if args[1] == "2" {
        q2();
    }
}
