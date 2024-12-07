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

    let equations = parse_input(&contents);

    println!(
        "Part 1: {part1_ans}",
        part1_ans = compute_all_equations(&equations, 2)
    );
    println!(
        "Part 2: {part2_ans}",
        part2_ans = compute_all_equations(&equations, 3)
    );
}

#[derive(Debug)]
struct Equation {
    result: i64,
    values: Vec<i32>,
}

fn compute_all_equations(equations: &Vec<Equation>, base: i32) -> i64 {
    equations
        .into_iter()
        .map(|equation| {
            let num_ops = equation.values.len() - 1;
            let num_possibilities = base.pow(num_ops as u32);

            let mut operations = vec![0u8; num_ops];

            for i in 0..num_possibilities {
                let mut value = i;
                for op_idx in 0..num_ops {
                    operations[op_idx] = (value % base) as u8;
                    value /= base;
                }
                if let Some(result) = compute_equation(equation, &operations) {
                    return result;
                }
            }
            0
        })
        .sum()
}

fn compute_equation(equation: &Equation, operations: &Vec<u8>) -> Option<i64> {
    let mut values: Vec<i64> = equation.values.iter().map(|&v| v as i64).collect();
    let mut op_idx: usize = 0;

    while values.len() > 1 {
        let lhs = values.pop().unwrap();
        let rhs = values.pop().unwrap();

        let result = match operations[op_idx] {
            0 => lhs + rhs,
            1 => lhs * rhs,
            2 => {
                let concat = format!("{}{}", lhs, rhs);
                concat.parse::<i64>().unwrap()
            }
            _ => unreachable!("Invalid operation"),
        };
        op_idx += 1;
        if result > equation.result {
            return None;
        }
        values.push(result);
    }

    let result = values.pop().unwrap();
    return if result == equation.result {
        Some(result)
    } else {
        None
    };
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let result: i64 = left.parse().unwrap();
            let values: Vec<i32> = right
                .split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .rev()
                .collect();
            Equation { result, values }
        })
        .collect()
}
