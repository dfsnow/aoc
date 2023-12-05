use std::collections::HashMap;
use std::env::args;
use std::io::{stdin, Read};
use regex::Regex;

const NUM_MAP: [(&'static str, &'static str); 9] = [
    ("one", "1"), 
    ("two", "2"), 
    ("three", "3"), 
    ("four", "4"), 
    ("five", "5"), 
    ("six", "6"), 
    ("seven", "7"), 
    ("eight", "8"), 
    ("nine", "9")
];

fn get_lines() -> Vec<String> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
    lines.retain(|s| !s.is_empty());
    return lines
}

fn reverse_string(x: &String) -> String {
    let mut x_chars: Vec<char> = x.chars().collect();
    x_chars.reverse();
    let x_rev: String = x_chars.into_iter().collect();
    return x_rev
}

fn q1(lines: Vec<String>) {
    let re_fd = Regex::new(r"([0-9]{1})").unwrap();
    let re_ld = Regex::new(r"(\d)[^\d]*$").unwrap();

    let results = lines.iter().map(|l| {
        let Some(first_digit) = re_fd.captures(l) else { return 0 };
        let Some(last_digit) = re_ld.captures(l) else { return 0 };
        let num = first_digit[1].to_owned() + &last_digit[1];
        let num: i32 = num.parse().unwrap();
        return num
    });

    println!("{}", results.collect::<Vec<i32>>().iter().sum::<i32>());
}

fn q2(lines: Vec<String>, num_to_words: HashMap<&str, &str>) {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_rev = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

    let results = lines.iter().map(|l| {
        let l_rev = reverse_string(l);
        let Some(first_digit) = re.captures(l) else { return 0 };
        let Some(last_digit) = re_rev.captures(l_rev.as_str()) else { return 0 };
        let ld_rev = reverse_string(&last_digit[1].to_string());

        let fdn: String = match num_to_words.get(&first_digit[1]) {
            Some(num) => num.to_string(),
            None => first_digit[1].to_string()
        };
        let ldn: &str = match num_to_words.get(ld_rev.as_str()) {
            Some(num) => num,
            None => &ld_rev.as_str()
        };

        let num = fdn + ldn;
        let num: i32 = num.parse().unwrap();
        return num;
    });

    println!("{}", results.collect::<Vec<i32>>().iter().sum::<i32>());
}

fn main() {
    let num_to_words: HashMap<_, _> = NUM_MAP.into_iter().collect();
    let args: Vec<String> = args().collect();
    if args[1] == "1" {
        q1(get_lines());
    } else if args[1] == "2" {
        q2(get_lines(), num_to_words);
    }
}
