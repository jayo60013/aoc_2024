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

    println!("Part 1: {part1_ans}", part1_ans = part1(&contents));
    println!("Part 2: {part2_ans}", part2_ans = part2(&contents));
}

fn part1(input: &str) -> i32 {
    let rules = parse_rules(input);
    let page_numbers = parse_page_numbers(input);

    page_numbers
        .into_iter()
        .map(|page_number| process_page_numbers(&rules, &page_number))
        .sum()
}

fn part2(input: &str) -> i32 {
    let rules = parse_rules(input);
    let page_numbers = parse_page_numbers(input);

    page_numbers
        .into_iter()
        .map(|page_number| process_page_numbers_2(&rules, page_number))
        .sum()
}

fn process_page_numbers_2(rules: &HashMap<&str, Vec<&str>>, page_numbers: Vec<&str>) -> i32 {
    let mut tmp = page_numbers.clone();

    if process_page_numbers(rules, &page_numbers) != 0 {
        return 0;
    }

    while process_page_numbers(rules, &tmp) == 0 {
        'killme: for (i, &page_number) in tmp.iter().enumerate() {
            if !rules.contains_key(page_number) {
                continue;
            }

            let others = rules.get(page_number).unwrap();
            for &other in others {
                if let Some(j) = tmp.iter().position(|&pn| pn == other) {
                    if i > j {
                        tmp.swap(i, j);
                        break 'killme;
                    }
                }
            }
        }
    }
    assert!(tmp.len() % 2 != 0);
    let mid_idx = tmp.len() / 2;
    tmp[mid_idx].parse().unwrap()
}

fn process_page_numbers(rules: &HashMap<&str, Vec<&str>>, page_numbers: &Vec<&str>) -> i32 {
    for (i, &page_number) in page_numbers.iter().enumerate() {
        if !rules.contains_key(page_number) {
            continue;
        }

        let others = rules.get(page_number).unwrap();
        for &other in others {
            if let Some(j) = page_numbers.iter().position(|&pn| pn == other) {
                if i > j {
                    return 0;
                }
            }
        }
    }

    assert!(page_numbers.len() % 2 != 0);
    let mid_idx = page_numbers.len() / 2;
    page_numbers[mid_idx].parse().unwrap()
}

fn parse_rules(input: &str) -> HashMap<&str, Vec<&str>> {
    if let Some((rules, _)) = input.split_once("\n\n") {
        rules.split('\n').fold(HashMap::new(), |mut map, rule| {
            if let Some((l, r)) = rule.split_once("|") {
                map.entry(l).or_insert_with(Vec::new).push(r);
                map
            } else {
                panic!("Parsing error");
            }
        })
    } else {
        panic!("Parsing error");
    }
}

fn parse_page_numbers(input: &str) -> Vec<Vec<&str>> {
    if let Some((_, page_numbers)) = input.split_once("\n\n") {
        page_numbers
            .trim_end()
            .split('\n')
            .map(|line| line.split(',').collect())
            .collect()
    } else {
        panic!("Parsing error");
    }
}
