use std::{env, fs};

use regex::Regex;

#[derive(Debug)]
struct Configuration {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print!(
            "Usage: {executable_name} <filename.txt>\n",
            executable_name = args.get(0).unwrap()
        );
        return;
    }
    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();

    let configs = parse_input(&contents);

    println!("Part 1: {}", part1(&configs));
    println!("Part 2: {}", part2(&configs));
}

fn part1(configs: &Vec<Configuration>) -> i64 {
    configs
        .iter()
        .map(|config| {
            let (n, m) = solve_config(config);
            n * 3 + m * 1
        })
        .sum()
}

fn part2(configs: &Vec<Configuration>) -> i64 {
    configs
        .iter()
        .map(|config| {
            let p2_config = Configuration {
                prize: (
                    config.prize.0 + 10000000000000,
                    config.prize.1 + 10000000000000,
                ),
                ..*config
            };
            let (n, m) = solve_config(&p2_config);
            n * 3 + m * 1
        })
        .sum()
}

fn solve_config(config: &Configuration) -> (i64, i64) {
    let det = (config.a.0 * config.b.1) - (config.a.1 * config.b.0);
    if det == 0 {
        panic!("det == 0");
    }
    let n = (config.b.1 * config.prize.0 - config.b.0 * config.prize.1) / det;
    let m = (config.a.0 * config.prize.1 - config.a.1 * config.prize.0) / det;

    let a_correct = n * config.a.0 + m * config.b.0 == config.prize.0;
    let b_correct = n * config.a.1 + m * config.b.1 == config.prize.1;
    return if a_correct && b_correct {
        (n, m)
    } else {
        (0, 0)
    };
}

fn parse_input(input: &str) -> Vec<Configuration> {
    let part_pattern =
        r"Button A: X\+(\d+), Y\+(\d+)\s*Button B: X\+(\d+), Y\+(\d+)\s*Prize: X=(\d+), Y=(\d+)";
    let part_re = Regex::new(&part_pattern).unwrap();

    part_re
        .captures_iter(input)
        .map(|c| {
            let a = (c[1].parse::<i64>().unwrap(), c[2].parse::<i64>().unwrap());
            let b = (c[3].parse::<i64>().unwrap(), c[4].parse::<i64>().unwrap());
            let prize = (c[5].parse::<i64>().unwrap(), c[6].parse::<i64>().unwrap());
            Configuration { a, b, prize }
        })
        .collect()
}
