use std::{collections::HashMap, env, fs};

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

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
    let grid = parse_grid(input);
    let start_pos = get_pos(input, 'S');
    let goal_pos = get_pos(input, 'E');
    let mut queue: Vec<((i32, i32), (i32, i32))> = Vec::new();
    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();

    let mut pos = start_pos;
    let mut dir = (1, 0); //Facing east

    queue.push((pos, dir));
    visited.insert(pos, 0);

    while queue.len() > 0 {
        (pos, dir) = queue.pop().unwrap();
        let score_so_far = visited.get(&pos).unwrap();

        for (np, nd) in get_nbours(&grid, pos) {
            let score_inc = get_rotation_distance(nd, dir) * 1000 + 1;
            if visited.contains_key(&np) {
                let old_score = visited.get(&np).unwrap();
                if old_score > &(score_so_far + score_inc) {
                    visited.insert(np, score_so_far + score_inc);
                } else {
                    continue;
                }
            } else {
                visited.insert(np, score_so_far + score_inc);
            }

            queue.push((np, nd));
        }
    }

    *visited.get(&goal_pos).unwrap()
}

fn get_nbours(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> Vec<((i32, i32), (i32, i32))> {
    let mut nbors: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for delta in DIRECTIONS {
        let np = (pos.0 + delta.0, pos.1 + delta.1);
        let npu = (np.0 as usize, np.1 as usize);

        if !check_inbounds(np, grid) {
            continue;
        }

        if grid[npu.1][npu.0] == '#' {
            continue;
        }
        nbors.push((np, delta));
    }
    return nbors;
}

fn check_inbounds(point: (i32, i32), grid: &Vec<Vec<char>>) -> bool {
    0 <= point.0
        && point.0 < grid.get(0).unwrap().len().try_into().unwrap()
        && 0 <= point.1
        && point.1 < grid.len().try_into().unwrap()
}

fn get_rotation_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    if a == b {
        return 0;
    }

    if a.0.abs() == b.0.abs() || a.1.abs() == b.1.abs() {
        return 2;
    }
    return 1;
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_pos(input: &str, ch: char) -> (i32, i32) {
    input
        .lines()
        .enumerate()
        .find_map(|(i, line)| line.find(ch).map(|j| (j as i32, i as i32)))
        .unwrap()
}

fn part2(input: &str) -> i32 {
    0
}
