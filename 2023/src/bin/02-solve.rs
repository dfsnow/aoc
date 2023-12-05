use std::collections::HashMap;
use regex::Regex;

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
    let start = std::time::Instant::now();
    let input = std::fs::read_to_string("input/02-input.txt")
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let cubes = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    q1(input.clone(), cubes.clone());
    q2(input, cubes);

    println!("time elapsed: {:?}", start.elapsed());
}
