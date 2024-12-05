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

    let (rules, page_numbers) = parse_input(&contents);

    println!("Part 1: {}", part1(&rules, &page_numbers));
    println!("Part 2: {}", part2(&rules, &page_numbers));
}

fn part1(rules: &HashMap<&str, Vec<&str>>, page_numbers: &Vec<Vec<&str>>) -> i32 {
    page_numbers
        .into_iter()
        .map(|page_number| process_page_numbers(rules, page_number))
        .sum()
}

fn part2(rules: &HashMap<&str, Vec<&str>>, page_numbers: &Vec<Vec<&str>>) -> i32 {
    page_numbers
        .into_iter()
        .map(|page_number| process_page_numbers_2(rules, page_number))
        .sum()
}

fn process_page_numbers_2(rules: &HashMap<&str, Vec<&str>>, page_numbers: &Vec<&str>) -> i32 {
    let mut tmp = page_numbers.to_vec();

    if process_page_numbers(rules, &page_numbers) != 0 {
        return 0;
    }

    while process_page_numbers(rules, &tmp) == 0 {
        'outer: for (i, &page_number) in tmp.iter().enumerate() {
            if let Some(others) = rules.get(page_number) {
                for &other in others {
                    if let Some(j) = tmp.iter().position(|&pn| pn == other) {
                        if i > j {
                            tmp.swap(i, j);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    let mid_idx = tmp.len() / 2;
    tmp[mid_idx].parse().unwrap()
}

fn process_page_numbers(rules: &HashMap<&str, Vec<&str>>, page_numbers: &Vec<&str>) -> i32 {
    for (i, &page_number) in page_numbers.iter().enumerate() {
        if let Some(others) = rules.get(page_number) {
            for &other in others {
                if let Some(j) = page_numbers.iter().position(|&pn| pn == other) {
                    if i > j {
                        return 0;
                    }
                }
            }
        }
    }
    let mid_idx = page_numbers.len() / 2;
    page_numbers[mid_idx].parse().expect("uh oh")
}

fn parse_input(input: &str) -> (HashMap<&str, Vec<&str>>, Vec<Vec<&str>>) {
    input
        .split_once("\n\n")
        .map(|(rules, page_numbers)| {
            let rules_map = rules.lines().fold(HashMap::new(), |mut map, rule| {
                if let Some((l, r)) = rule.split_once("|") {
                    map.entry(l).or_insert_with(Vec::new).push(r);
                }
                map
            });

            let page_nums = page_numbers
                .lines()
                .map(|line| line.split(',').collect())
                .collect();

            (rules_map, page_nums)
        })
        .unwrap()
}
