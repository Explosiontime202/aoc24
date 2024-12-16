use std::{collections::HashMap, time::Instant};

#[derive(Clone, Copy, Debug)]
struct Space {
    pos: usize,
    len: usize,
    id: Option<usize>,
}

fn solve_a(mut files: Vec<Space>, mut free_list: Vec<Space>, total_size: usize) -> usize {
    let mut sum = 0;

    let mut disk_idx = 0;
    while disk_idx < total_size {
        if files.is_empty() {
            break;
        }
        if files[0].pos <= disk_idx {
            assert!(files[0].pos == disk_idx);
            let f = files.remove(0);
            for f_pos in f.pos..(f.pos + f.len) {
                // print!("{}", f.id.unwrap());
                sum += f.id.unwrap() * f_pos;
            }

            disk_idx += f.len;
            continue;
        }

        let mut free = free_list.remove(0);
        if free.pos != disk_idx {
            println!("{:?}", files[0]);
            println!("{free:?}");
        }
        assert_eq!(free.pos, disk_idx);

        while free.len > 0 {
            if files.is_empty() {
                break;
            }
            let mut f = files.pop().unwrap();

            let moved_len = free.len.min(f.len);

            // println!("\nmoved_len: {moved_len} (id = {})", f.id.unwrap());

            for pos in disk_idx..(disk_idx + moved_len) {
                // print!("{}", f.id.unwrap());
                sum += pos * f.id.unwrap();
            }

            disk_idx += moved_len;
            free.len -= moved_len;
            f.len -= moved_len;

            if f.len > 0 {
                files.push(f);
                break;
            }
        }

        if !files.is_empty() {
            assert_eq!(free.len, 0);
        }
    }

    // println!("");

    sum
}

fn solve_b(mut files: Vec<Space>, mut free_list: Vec<Space>, total_size: usize) -> usize {
    let mut processed_files = Vec::new();

    for mut file in files.into_iter().rev() {
        for free_idx in 0..free_list.len() {
            if free_list[free_idx].pos < file.pos && free_list[free_idx].len >= file.len {
                file.pos = free_list[free_idx].pos;

                free_list[free_idx].pos += file.len;
                free_list[free_idx].len -= file.len;

                if free_list[free_idx].len == 0 {
                    free_list.remove(free_idx);
                }
                break;
            }
        }
        processed_files.push(file);
    }

    let mut sum = 0;

    for file in processed_files {
        for p in file.pos..(file.pos + file.len) {
            sum += p * file.id.unwrap();
        }
    }

    sum
}

fn main() {
    let start_loading = Instant::now();
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let loading_time = start_loading.elapsed();
    // let input = std::fs::read_to_string("../example.txt").unwrap();

    let parsing_start = Instant::now();
    let char_map: HashMap<char, usize> = std::iter::zip('0'..='9', 0..=9).collect();

    let input_nums: Vec<usize> = input.chars().map(|c: char| char_map[&c]).collect();

    let mut free_list: Vec<Space> = Vec::new();
    let mut files: Vec<Space> = Vec::new();
    let mut total_size = 0;
    let mut next_file_id = 0;

    for (idx, num) in input_nums.into_iter().enumerate() {
        if num == 0 {
            continue;
        }
        match idx % 2 {
            1 => free_list.push(Space {
                pos: total_size,
                len: num,
                id: None,
            }),
            0 => {
                files.push(Space {
                    pos: total_size,
                    len: num,
                    id: Some(next_file_id),
                });
                next_file_id += 1;
            }
            _ => unreachable!(),
        }

        total_size += num;
    }

    let parsing_time = parsing_start.elapsed();

    println!("free = {:?}", &free_list[..8]);
    println!("files: {:?}", &files[..8]);

    let start_a = Instant::now();
    let output_a = solve_a(files.clone(), free_list.clone(), total_size);
    let a_time = start_a.elapsed();
    let start_b = Instant::now();
    let output_b = solve_b(files, free_list, total_size);
    let b_time = start_b.elapsed();

    println!("Task1: {output_a}");
    println!("Task1: {output_b}");

    println!("Input loading took: {}µs", loading_time.as_micros());
    println!("Parsing took: {}µs", parsing_time.as_micros());
    println!("Task 1 took: {}µs", a_time.as_micros());
    println!("Task 2 took: {}µs", b_time.as_micros());
}
