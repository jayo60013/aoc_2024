use std::{env, fs};

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

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
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let grid_height = (grid.len() - 1) as i32;
    let grid_width = (grid[0].len() - 1) as i32;
    let mut total = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != 'X' {
                continue;
            }
            for delta in DIRECTIONS.iter() {
                let end_x = (x as i32) + delta.0 * 3;
                let end_y = (y as i32) + delta.1 * 3;
                if end_x < 0 || end_x > grid_width || end_y < 0 || end_y > grid_height {
                    continue;
                }
                let mx = ((x as i32) + delta.0 * 1) as usize;
                let my = ((y as i32) + delta.1 * 1) as usize;
                if grid[my][mx] != 'M' {
                    continue;
                }
                let ax = ((x as i32) + delta.0 * 2) as usize;
                let ay = ((y as i32) + delta.1 * 2) as usize;
                if grid[ay][ax] != 'A' {
                    continue;
                }
                let sx = ((x as i32) + delta.0 * 3) as usize;
                let sy = ((y as i32) + delta.1 * 3) as usize;
                if grid[sy][sx] != 'S' {
                    continue;
                }
                total += 1;
            }
        }
    }
    return total;
}

fn part2(contents: &str) -> i32 {
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let grid_height = (grid.len() - 1) as i32;
    let grid_width = (grid[0].len() - 1) as i32;
    let mut total = 0;

    for y in 1..grid_height as usize {
        for x in 1..grid_width as usize {
            if grid[y][x] != 'A' {
                continue;
            }
            let tl = grid[y + 1][x - 1];
            let tr = grid[y + 1][x + 1];
            let bl = grid[y - 1][x - 1];
            let br = grid[y - 1][x + 1];

            let a = tl == 'M' && tr == 'M' && br == 'S' && bl == 'S';
            let b = tl == 'S' && tr == 'M' && br == 'M' && bl == 'S';
            let c = tl == 'S' && tr == 'S' && br == 'M' && bl == 'M';
            let d = tl == 'M' && tr == 'S' && br == 'S' && bl == 'M';

            if a || b || c || d {
                total += 1;
            }
        }
    }
    total
}
