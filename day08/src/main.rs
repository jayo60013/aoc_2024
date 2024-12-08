use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

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

fn part1(input: &str) -> usize {
    let antennas = parse_input(input);
    let (width, height) = get_dimensions(input);
    let mut antinode_locations: HashSet<Point> = HashSet::new();

    for (_, locations) in antennas {
        for i in 0..(locations.len() - 1) {
            for j in i..locations.len() {
                if i == j {
                    continue;
                }
                let dist_vec = get_dist(locations[i], locations[j]);
                let na = add_points(locations[i], dist_vec);
                let nb = subtract_points(locations[j], dist_vec);

                if check_point_inbounds(na, width, height) {
                    antinode_locations.insert(na);
                }
                if check_point_inbounds(nb, width, height) {
                    antinode_locations.insert(nb);
                }
            }
        }
    }
    antinode_locations.len()
}

fn check_point_inbounds(p: Point, width: i32, height: i32) -> bool {
    0 <= p.x && p.x < width && 0 <= p.y && p.y < height
}

fn add_points(a: Point, b: Point) -> Point {
    Point {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

fn subtract_points(a: Point, b: Point) -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}

fn get_dist(a: Point, b: Point) -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}

fn get_dimensions(input: &str) -> (i32, i32) {
    let w = input.split_once('\n').unwrap().0.len();
    let h = input.lines().count();
    (w as i32, h as i32)
}

fn part2(input: &str) -> usize {
    let antennas = parse_input(input);
    let (width, height) = get_dimensions(input);
    let mut antinode_locations: HashSet<Point> = HashSet::new();

    for (_, locations) in antennas {
        for i in 0..(locations.len() - 1) {
            for j in i..locations.len() {
                if i == j {
                    continue;
                }
                let dist_vec = get_dist(locations[i], locations[j]);

                let mut na = add_points(locations[i], dist_vec);
                while check_point_inbounds(na, width, height) {
                    antinode_locations.insert(na);
                    na = add_points(na, dist_vec);
                }

                let mut nb = subtract_points(locations[j], dist_vec);
                while check_point_inbounds(nb, width, height) {
                    antinode_locations.insert(nb);
                    nb = subtract_points(nb, dist_vec);
                }
            }
        }
        if locations.len() == 0 {
            continue;
        }
        for location in locations {
            antinode_locations.insert(location);
        }
    }
    antinode_locations.len()
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> HashMap<char, Vec<Point>> {
    input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, line)| {
            line.char_indices()
                .filter(|&(_, c)| c != '.')
                .for_each(|(j, c)| {
                    map.entry(c).or_insert_with(Vec::new).push(Point {
                        x: j as i32,
                        y: i as i32,
                    })
                });
            map
        })
}
