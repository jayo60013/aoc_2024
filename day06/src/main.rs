use std::{collections::HashSet, env, fs};

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

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let (width, height) = (grid[0].len() as i32, grid.len() as i32);
    let mut pos = get_start_pos(input);
    let mut dir = (0, -1);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    while 0 <= pos.0 && pos.0 < width && 0 <= pos.1 && pos.1 < height {
        visited.insert(pos);
        (pos, dir) = step_guard(&grid, pos, dir);
    }
    visited.len()
}

fn part2(_input: &str) -> i32 {
    return 0;
}

fn step_guard(grid: &Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let nx = pos.0 + dir.0;
    let ny = pos.1 + dir.1;

    if nx < 0 || nx >= grid[0].len() as i32 || ny < 0 || ny >= grid.len() as i32 {
        return ((nx, ny), dir);
    }

    let x = nx as usize;
    let y = ny as usize;
    if grid[y][x] == '.' || grid[y][x] == '^' {
        return ((nx, ny), dir);
    }

    return step_guard(grid, pos, rotate_dir(dir));
}

fn rotate_dir(in_dir: (i32, i32)) -> (i32, i32) {
    return match in_dir {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => panic!("Invalid dir"),
    };
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_start_pos(input: &str) -> (i32, i32) {
    for (i, line) in input.lines().enumerate() {
        if let Some(j) = line.find('^') {
            return (j as i32, i as i32);
        };
    }
    panic!("Could not find start");
}
