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
    let obstacles = parse_obstacles(input);
    let (width, height) = get_grid_dimensions(input);

    let mut pos = get_start_pos(input);
    let mut dir = UP;
    let mut visited: HashSet<Point> = HashSet::new();

    while check_if_point_in_grid(&pos, width, height) {
        visited.insert(pos);
        pos = step_guard(&obstacles, pos, &mut dir, width, height);
    }
    visited.len()
}

fn part2(input: &str) -> usize {
    let obstacles = parse_obstacles(input);
    let (width, height) = get_grid_dimensions(input);
    let start_pos = get_start_pos(input);
    let mut pos = start_pos;
    let mut dir = UP;
    let mut visited: HashSet<Point> = HashSet::new();

    while check_if_point_in_grid(&pos, width, height) {
        visited.insert(pos);
        pos = step_guard(&obstacles, pos, &mut dir, width, height);
    }

    visited
        .into_iter()
        .filter(|&visited_point| visited_point != start_pos)
        .filter(|&visited_point| {
            let mut additional_obstacles = obstacles.clone();
            additional_obstacles.insert(visited_point);

            check_for_cycle(start_pos, additional_obstacles, width, height)
        })
        .count()
}

fn check_for_cycle(start_pos: Point, obstacles: HashSet<Point>, width: i32, height: i32) -> bool {
    let mut pos = start_pos;
    let mut dir = UP;
    let mut route: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    while check_if_point_in_grid(&pos, width, height) {
        if !route.insert((pos.x, pos.y, dir.x, dir.y)) {
            return true;
        }
        pos = step_guard(&obstacles, pos, &mut dir, width, height);
    }
    return false;
}

fn step_guard(
    obstacles: &HashSet<Point>,
    pos: Point,
    dir: &mut Point,
    width: i32,
    height: i32,
) -> Point {
    loop {
        let np = Point {
            x: pos.x + dir.x,
            y: pos.y + dir.y,
        };

        // Guard exited grid or step ahead clear
        if !check_if_point_in_grid(&np, width, height) || !obstacles.contains(&np) {
            return np;
        }

        *dir = rotate_dir(*dir);
    }
}

const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };

fn rotate_dir(in_dir: Point) -> Point {
    return match in_dir {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        LEFT => UP,
        _ => unreachable!(),
    };
}

fn check_if_point_in_grid(p: &Point, width: i32, height: i32) -> bool {
    return 0 <= p.x && p.x < width as i32 && 0 <= p.y && p.y < height as i32;
}

fn get_grid_dimensions(input: &str) -> (i32, i32) {
    let width = input.split_once('\n').unwrap().0.len() as i32;
    let height = input.lines().count() as i32;
    (width, height)
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_obstacles(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .fold(HashSet::new(), |mut set, (i, line)| {
            line.char_indices()
                .filter(|&(_, c)| c == '#')
                .for_each(|(j, _)| {
                    set.insert(Point {
                        x: j as i32,
                        y: i as i32,
                    });
                });
            set
        })
}

fn get_start_pos(input: &str) -> Point {
    input
        .lines()
        .enumerate()
        .find_map(|(i, line)| {
            line.find('^').map(|j| Point {
                x: j as i32,
                y: i as i32,
            })
        })
        .unwrap()
}
