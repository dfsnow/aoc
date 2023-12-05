use std::collections::HashMap;
use std::env::args;
use std::io::{stdin, Read};
use regex::Regex;

fn get_lines() -> Vec<String> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
    lines.retain(|s| !s.is_empty());
    return lines
}

fn q1(lines: Vec<String>, cubes: HashMap<&str, i32>) {
    let re_id = Regex::new(r"^Game ([0-9]{1,3}):").unwrap();

    let results = lines.iter().map(|l| {
        let Some(id) = re_id.captures(l) else { return 0 };
        let id = id[1].parse().unwrap();
        let mut over = 0;
        
        for color in cubes.clone().into_keys().collect::<Vec<&str>>().iter() {
            let re_color = Regex::new(&format!(r"([0-9]{{1,3}}) {}", color)).unwrap();
            let color_caps = re_color.captures_iter(l).map(|caps| {
                let (_, [val]) = caps.extract();
                val.parse::<i32>().unwrap()
            }).collect::<Vec<i32>>();
            over = over + (color_caps.iter().any(|&x| x > cubes[color])) as i32;
        }
        if over > 0 { return 0 } else { return id };
    });

    println!("{}", results.collect::<Vec<i32>>().iter().sum::<i32>());
}

fn q2(lines: Vec<String>, cubes: HashMap<&str, i32>) {
    let results = lines.iter().map(|l| {
        let mut color_power = 1;

        for color in cubes.clone().into_keys().collect::<Vec<&str>>().iter() {
            let re_color = Regex::new(&format!(r"([0-9]{{1,3}}) {}", color)).unwrap();
            let color_max = re_color.captures_iter(l).map(|caps| {
                let (_, [val]) = caps.extract();
                val.parse::<i32>().unwrap()
            }).collect::<Vec<i32>>();
            color_power = color_power * color_max.iter().max().unwrap();
        }
        return color_power;
    });

    println!("{}", results.collect::<Vec<i32>>().iter().sum::<i32>());
}

fn main() {
    let cubes = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let args: Vec<String> = args().collect();
    if args[1] == "1" {
        q1(get_lines(), cubes);
    } else if args[1] == "2" {
        q2(get_lines(), cubes);
    }
}
