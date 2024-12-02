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

    println!("Part 1: {part1_ans}", part1_ans = part1(contents.clone()));
    println!("Part 2: {part2_ans}", part2_ans = part2(contents));
}

fn part1(contents: String) -> usize {
    apply_record_safety(|record| is_record_safe(record), contents)
}

fn part2(contents: String) -> usize {
    apply_record_safety(
        |record| is_record_safe(record) || is_record_safe_without_one_element(record),
        contents,
    )
}

fn apply_record_safety<F>(safety_func: F, contents: String) -> usize
where
    F: Fn(&Vec<i32>) -> bool,
{
    contents
        .lines()
        .map(|line| {
            return line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
        })
        .filter(|record| safety_func(record))
        .count()
}

fn is_record_safe(record: &Vec<i32>) -> bool {
    let is_desc = record[0] > record[1];

    for i in 0..(record.len() - 1) {
        let a = record[i];
        let b = record[i + 1];
        if is_desc && a < b || !is_desc && a > b {
            return false;
        }

        let diff = (a - b).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

fn is_record_safe_without_one_element(record: &Vec<i32>) -> bool {
    (0..record.len()).any(|i| {
        let mut tmp = record.clone();
        tmp.remove(i);
        is_record_safe(&tmp)
    })
}
