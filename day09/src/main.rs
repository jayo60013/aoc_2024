use std::{env, fs};

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

    let disk_map = parse_input(&contents);

    println!("Part 1: {part1_ans}", part1_ans = part1(&disk_map));
    println!("Part 2: {part2_ans}", part2_ans = part2(&disk_map));
}

fn part1(disk_map: &Vec<File>) -> i64 {
    let mut compressed_disk: Vec<Option<i32>> = Vec::new();
    let length = disk_map.last().unwrap().end;
    let mut disk_map_idx: usize = 0;

    for ptr in 0..=length {
        let file = disk_map.get(disk_map_idx).unwrap();

        if file.start <= ptr && ptr <= file.end {
            compressed_disk.push(Some(file.id));
        } else {
            compressed_disk.push(None);
        }
        if ptr >= file.end {
            disk_map_idx += 1;
        }
    }

    while compressed_disk.contains(&None) {
        let last = compressed_disk.pop().unwrap();
        if last.is_none() {
            continue;
        }

        if let Some(first_none) = compressed_disk.iter_mut().find(|x| x.is_none()) {
            *first_none = last;
        }
    }

    compressed_disk
        .iter()
        .enumerate()
        .map(|(i, file_id)| i as i64 * file_id.unwrap() as i64)
        .sum()
}

fn part2(disk_map: &Vec<File>) -> i64 {
    let mut disk: Vec<Option<i32>> = Vec::new();
    let length = disk_map.last().unwrap().end;
    let mut disk_map_idx: usize = 0;

    for ptr in 0..=length {
        let file = disk_map.get(disk_map_idx).unwrap();

        if file.start <= ptr && ptr <= file.end {
            disk.push(Some(file.id));
        } else {
            disk.push(None);
        }
        if ptr >= file.end {
            disk_map_idx += 1;
        }
    }

    for file in disk_map.iter().rev() {
        let right_ptr = file.end;
        let window_size = file.end - file.start + 1;

        if let Some(start_idx) = disk
            .windows(window_size)
            .position(|window| window == vec![None; window_size])
        {
            if start_idx < right_ptr {
                for i in start_idx..(start_idx + window_size) {
                    disk[i] = Some(file.id);
                }
                for i in file.start..=file.end {
                    disk[i] = None;
                }
            }
        }
    }

    disk.iter()
        .enumerate()
        .filter(|(_, x)| x.is_some())
        .map(|(i, file_id)| i as i64 * file_id.unwrap() as i64)
        .sum()
}

#[derive(Debug)]
struct File {
    id: i32,
    start: usize,
    end: usize,
}

fn parse_input(input: &str) -> Vec<File> {
    let mut is_filename = true;
    let mut filename: i32 = 0;
    let mut ptr: usize = 0;
    let mut files: Vec<File> = Vec::new();

    for c in input.trim().chars() {
        let length = c.to_digit(10).unwrap() as usize;
        if is_filename {
            files.push(File {
                id: filename,
                start: ptr,
                end: ptr + length - 1,
            });

            ptr += length;
            filename += 1;
            is_filename = false;
        } else {
            ptr += length;
            is_filename = true;
        }
    }
    files
}
