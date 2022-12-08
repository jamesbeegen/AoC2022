#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Read lines from puzzle input file. 
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Creates a Vec from the lines from puzzle input file
fn read_input_file() -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("src/input.txt") {
        for line in lines {
            if let Ok(line) = line{
                let l: String = line;
                vec.push(l);
            }
        }
    }
    return vec;
}

// Returns the index of the assignment with the (smallest, largest) range
fn find_large_diff(ranges: &Vec<Vec<i32>>) -> (usize, usize) {
    let mut diffs: Vec<i32> = Vec::new();
    for range in ranges {
        if range[0] > range[1] {
            let small_bound = range[1];
            let large_bound = range[0];
            diffs.push(large_bound - small_bound);
        }
        else if range[0] < range[1] {
            let small_bound = range[0];
            let large_bound = range[1];
            diffs.push(large_bound - small_bound);
        }
        else{
            diffs.push(0);
        }
    }
    if diffs[0] > diffs[1]{
        return (0, 1);
    }
    return (1, 0);
}

// Part 1
fn check_overlap(line: String, part2: bool) -> bool {    
    // Split each line by comma, and then dash
    let mut ranges: Vec<Vec<i32>> = Vec::new();
    let split_by_comma: Vec<&str> = line.split(",").collect();
    for range in &split_by_comma {
        let temp_vec: Vec<&str> = range.split("-").collect();
        let mut split_by_dash: Vec<i32> = Vec::new();
        for boundary in temp_vec {
            split_by_dash.push(boundary.parse::<i32>().unwrap());
        }
        ranges.push(split_by_dash.clone());
    }

    // Actual check for overlaps
    let diffs = find_large_diff(&ranges);
    let mut overlaps = 0;
    for check_num in ranges[0][0]..ranges[0][1] + 1 {
        if check_num >= ranges[1][0] && check_num <= ranges[1][1] {
            overlaps += 1
        }
    }
    if part2 {
        if overlaps >= 1 {
            return true;
        }
    }
    else if overlaps > (ranges[diffs.1][1] - ranges[diffs.1][0]) {
        return true;
    }

    overlaps = 0;
    for check_num in ranges[1][0]..ranges[1][1] + 1{
        if check_num >= ranges[0][0] && check_num <= ranges[0][1] {
            overlaps += 1
        }
    }
    if part2 {
        if overlaps >= 1 {
            return true;
        }
    }
    if overlaps > (ranges[diffs.1][1] - ranges[diffs.1][0]) {
        return true;
    }
    return false;
}

// Part 1
fn part1(lines: Vec<String>) -> i32 {
    let mut part1_overlap_pairs = 0;

    for x in 0..lines.len() {
        if check_overlap(lines[x].clone(), false){
            part1_overlap_pairs += 1;
        }
    }
    return part1_overlap_pairs;
}
    
fn part2(lines: Vec<String>) -> i32 {
    let mut part2_overlap_pairs = 0;
    for x in 0..lines.len() {
        if check_overlap(lines[x].clone(), true){
            part2_overlap_pairs += 1;
        }
    }
    return part2_overlap_pairs;
}

fn main() {
    let lines = read_input_file();
    println!("Part 1 Answer: {}", part1(lines.clone()));
    println!("Part 2 Answer: {}", part2(lines.clone()));
}
