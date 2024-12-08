use std::{
    collections::{HashMap, HashSet},
    env, fs,
    time::Instant,
};

use itertools::Itertools;

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

    let antennas = parse_input(&contents);
    let (width, height) = get_dimensions(&contents);

    let part1_answer = part1(&antennas, width, height);
    println!("Part 1: {part1_answer}");

    let part2_answer = part2(&antennas, width, height);
    println!("Part 2: {part2_answer}");
}

fn part1(antennas: &HashMap<char, Vec<Point>>, width: i32, height: i32) -> usize {
    antennas
        .iter()
        .fold(
            HashSet::new(),
            |mut antinode_locations: HashSet<Point>, (_, locations)| {
                // get every combination of location, find antinodes & add to the set
                locations.iter().tuple_combinations().for_each(|(&a, &b)| {
                    let dist_vec = subtract_points(a, b);

                    let na = add_points(a, dist_vec);
                    if check_point_inbounds(na, width, height) {
                        antinode_locations.insert(na);
                    }

                    let nb = subtract_points(b, dist_vec);
                    if check_point_inbounds(nb, width, height) {
                        antinode_locations.insert(nb);
                    }
                });

                antinode_locations
            },
        )
        .len()
}

fn part2(antennas: &HashMap<char, Vec<Point>>, width: i32, height: i32) -> usize {
    antennas
        .iter()
        .fold(
            HashSet::new(),
            |mut antinode_locations: HashSet<Point>, (_, locations)| {
                // get every combination of location, find resonant antinodes & add to the set
                locations.iter().tuple_combinations().for_each(|(&a, &b)| {
                    let dist_vec = subtract_points(a, b);

                    let mut na = add_points(a, dist_vec);
                    while check_point_inbounds(na, width, height) {
                        antinode_locations.insert(na);
                        na = add_points(na, dist_vec);
                    }

                    let mut nb = subtract_points(b, dist_vec);
                    while check_point_inbounds(nb, width, height) {
                        antinode_locations.insert(nb);
                        nb = subtract_points(nb, dist_vec);
                    }
                });
                // If more than one antenna on a frequency exists, antinode includes antenna
                // location
                if locations.len() > 1 {
                    antinode_locations.extend(locations.iter());
                }

                antinode_locations
            },
        )
        .len()
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

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn get_dimensions(input: &str) -> (i32, i32) {
    let w = input.split_once('\n').unwrap().0.len();
    let h = input.lines().count();
    (w as i32, h as i32)
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
