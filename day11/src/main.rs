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

    let nums: Vec<u64> = contents
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    println!("Part 1: {}", solve(&nums, 25));
    println!("Part 2: {}", solve(&nums, 75));
}

fn solve(nums: &Vec<u64>, blinks: u8) -> u64 {
    let mut lookup: HashMap<(u64, u8), u64> = HashMap::new();

    nums.iter()
        .map(|num| step_stones(*num, blinks, &mut lookup))
        .sum()
}

fn step_stones(num: u64, blinks: u8, lookup_table: &mut HashMap<(u64, u8), u64>) -> u64 {
    let value: u64;
    let len = ((num as f64).log10() + 1.0) as u32;

    if blinks == 0 {
        value = 1;
    } else if lookup_table.contains_key(&(num, blinks)) {
        value = *lookup_table.get(&(num, blinks)).unwrap();
    } else if num == 0 {
        value = step_stones(1, blinks - 1, lookup_table);
    } else if len % 2 == 0 {
        let l = num / (10 as u64).pow(len / 2);
        let r = num % (10 as u64).pow(len / 2);
        value = step_stones(l, blinks - 1, lookup_table) + step_stones(r, blinks - 1, lookup_table);
    } else {
        value = step_stones(num * 2024, blinks - 1, lookup_table);
    }
    lookup_table.insert((num, blinks), value);
    return value;
}
