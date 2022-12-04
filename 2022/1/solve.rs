use std::env::args;
use std::io::{stdin, Read};

fn get_totals() -> Vec<i32> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let grps = input.split("\n\n");
    let totals: Vec<i32> = grps
        .map(|grp| {
            let mut total = 0;
            for elf in grp.lines() {
                total += elf.parse::<i32>().unwrap();
            }
            total
        })
        .collect::<Vec<i32>>();
    totals
}

fn sum_top_n(mut x: Vec<i32>, n: usize) -> i32 {
    x.sort_by(|a, b| a.cmp(b).reverse());
    x.truncate(n);
    x.iter().sum()
}

fn q1() {
    println!("{}", get_totals().iter().max().unwrap());
}

fn q2() {
    println!("{}", sum_top_n(get_totals(), 3));
}

fn main() {
    let args: Vec<String> = args().collect();
    if args[1] == "1" {
        q1();
    } else if args[1] == "2" {
        q2();
    }
}
