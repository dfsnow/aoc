use std::str;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let start = std::time::Instant::now();
    let input = std::fs::read_to_string("input/03-input.txt")
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut nums: HashMap<_, i32> = HashMap::new();
    let mut stars: HashMap<_, Vec<_>> = HashMap::new();
    let re_num = Regex::new(r"[0-9]{1,3}").unwrap();
    let arr: [i32; 3] = [-1, 0, 1];

    for (i, l) in input.iter().enumerate() {
        // Find all numbers in the line
        re_num.find_iter(&l).for_each(|f| {
            let parsed = f.as_str().parse::<i32>().unwrap();
            for w in [f.start(), f.end() - 1] {
                for y in arr {
                    for x in arr {
                        // Bound the area to explore, starting from each number
                        let yi = if i as i32 + y < 0 || i as i32 + y >= input.len() as i32 { 0 } else { y };
                        let xi = if f.start() as i32 + x < 0 || f.end() as i32 + x > l.len() as i32 { 0 } else { x };
                        let idx = &[input[i + yi as usize].as_bytes()[w + xi as usize]];
                        let near_char = str::from_utf8(idx).unwrap();
                        
                        // Send number matches to hashmap
                        if !(near_char.contains(".") || near_char.chars().all(|c| c.is_ascii_digit())) {
                            nums.entry((f.start(), f.end() - 1, i))
                                .or_insert(parsed);
                        }
                        // Send numbers matching the same star to hashmap
                        if let Some(star) = stars.get(&(i + yi as usize, w + xi as usize)) {
                            if !star.contains(&parsed) {
                                stars.entry((i + yi as usize, w + xi as usize))
                                    .or_default()
                                    .push(parsed);
                            }
                        } else {
                            if near_char.contains("*") {
                                stars.entry((i + yi as usize, w + xi as usize))
                                    .or_default()
                                    .push(parsed);
                            }
                        }
                    }
                }
            }
        });
    };

    let part2: i32 = stars
        .iter()
        .filter(|&(_, values)| values.len() > 1)
        .map(|(_, value)| value[0] * value[1])
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>();

    println!("part1: {}", nums.values().cloned().sum::<i32>());
    println!("part2: {}", part2);
    println!("time elapsed: {:?}", start.elapsed());
}
