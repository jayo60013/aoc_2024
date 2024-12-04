use regex::{Captures, Regex};
use std::{env, fs};

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

    println!("Part 1: {part1_ans}", part1_ans = part1(&contents));
    println!("Part 2: {part2_ans}", part2_ans = part2(&contents));
}

fn part1(contents: &str) -> i32 {
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    mul_re.captures_iter(contents).map(calc_mul).sum()
}

fn part2(contents: &str) -> i32 {
    // Match either mul(\d,\d), do() or don't()
    // Capture groups for two digits in mul(\d,\d)
    let instruction_re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don\'t\(\)").unwrap();

    let mut is_mul_enabled = true;
    let mut total = 0;

    for capture in instruction_re.captures_iter(contents) {
        let instr = capture.get(0).unwrap().as_str();
        match instr {
            "do()" => {
                is_mul_enabled = true;
            }
            "don't()" => {
                is_mul_enabled = false;
            }
            _ => {
                if is_mul_enabled {
                    total += calc_mul(capture);
                }
            }
        }
    }
    total
}

fn calc_mul(capture: Captures) -> i32 {
    let l: i32 = capture[1].parse().unwrap();
    let r: i32 = capture[2].parse().unwrap();
    l * r
}
