use std::{env, fs};

#[derive(Debug)]
struct File {
    id: u32,
    start: usize,
    end: usize,
}

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
    let disk = parse_disk(&contents);

    println!("Part 1: {part1_ans}", part1_ans = part1(&disk));
    println!("Part 2: {part2_ans}", part2_ans = part2(&disk_map, &disk));
}

fn part1(disk: &Vec<Option<u32>>) -> u64 {
    let mut compressed_disk = disk.clone();

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
        .map(|(i, file_id)| i as u64 * file_id.unwrap() as u64)
        .sum()
}

fn part2(disk_map: &Vec<File>, disk: &Vec<Option<u32>>) -> u64 {
    let mut compressed_disk = disk.clone();

    for file in disk_map.iter().rev() {
        let right_ptr = file.end;
        let window_size = file.end - file.start + 1;

        if let Some(start_idx) = compressed_disk
            .windows(window_size)
            .position(|window| window == vec![None; window_size])
        {
            if start_idx < right_ptr {
                for i in start_idx..(start_idx + window_size) {
                    compressed_disk[i] = Some(file.id);
                }
                for i in file.start..=file.end {
                    compressed_disk[i] = None;
                }
            }
        }
    }

    compressed_disk
        .iter()
        .enumerate()
        .filter(|(_, x)| x.is_some())
        .map(|(i, file_id)| i as u64 * file_id.unwrap() as u64)
        .sum()
}

fn parse_input(input: &str) -> Vec<File> {
    let mut is_filename = true;
    let mut filename: u32 = 0;
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

fn parse_disk(input: &str) -> Vec<Option<u32>> {
    let mut is_filename = true;
    let mut filename: u32 = 0;
    let mut disk: Vec<Option<u32>> = Vec::new();

    for c in input.trim().chars() {
        let length = c.to_digit(10).unwrap() as usize;
        let value = if is_filename { Some(filename) } else { None };
        let mut blocks = vec![value; length];
        disk.append(&mut blocks);
        if is_filename {
            filename += 1;
        }
        is_filename = !is_filename;
    }
    disk
}
