use std::{collections::HashMap, env, fs};

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

fn part1(contents: String) -> i32 {
    let (mut left, mut right) = parse_input(contents);

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn part2(contents: String) -> i32 {
    let (left, right) = parse_input(contents);

    let left_counts: HashMap<i32, i32> = construct_count_map_from_list(left);
    let right_counts: HashMap<i32, i32> = construct_count_map_from_list(right);

    left_counts
        .iter()
        .map(|(k, v)| {
            let n = right_counts.get(k).unwrap_or(&0);
            return k * n * v;
        })
        .sum()
}

fn construct_count_map_from_list(list: Vec<i32>) -> HashMap<i32, i32> {
    let mut count_map: HashMap<i32, i32> = HashMap::new();
    for v in list {
        *count_map.entry(v).or_insert(0) += 1;
    }
    count_map
}

fn parse_input(contents: String) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let size = contents.lines().count();
    left.reserve(size);
    right.reserve(size);

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(l), Some(r)) = (parts.next(), parts.next()) {
            left.push(l.parse().unwrap());
            right.push(r.parse().unwrap());
        }
    }
    (left, right)
}
