use std::{collections::HashSet, env, fs};

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

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

    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let width = grid.get(0).unwrap().len();
    let height = grid.len();

    println!("Part 1: {}", part1(&grid, width as i32, height as i32));
    println!("Part 2: {}", part2(&grid));
}

fn part1(grid: &Vec<Vec<char>>, width: i32, height: i32) -> i32 {
    let claimed: HashSet<(usize, usize)> = HashSet::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if claimed.contains(&(x, y)) {
                continue;
            }

            let mut perimeter = 0;
            let mut area = 0;
            let mut queue: Vec<(usize, usize)> = Vec::new();

            for delta in DIRECTIONS {
                let np = (x as i32 + delta.0, y as i32 + delta.1);
                if !check_point_inbounds(np, width, height) {
                    continue;
                }
                if &grid[np.1 as usize][np.0 as usize] != ch {
                    perimeter += 1;
                }
            }
        }
    }
    0
}

fn check_point_inbounds(point: (i32, i32), width: i32, height: i32) -> bool {
    return 0 <= point.0 && point.0 < width && 0 <= point.1 && point.1 < height;
}

fn part2(input: &Vec<Vec<char>>) -> i32 {
    0
}
