#![allow(dead_code)]
//use core::asserting::Printable;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Read lines from puzzle input file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_input_file() -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line) = line{
                let mut l: String = line;
                vec.push(l);
            }
        }
    }
    return vec;
}

fn find_large_diff(ranges: &Vec<Vec<&str>>) -> (usize, usize) {
    let mut diffs: Vec<i32> = Vec::new();
    let mut small_bound = 0;
    let mut large_bound = 0;
    for range in ranges {
        if range[0] > range[1] {
            small_bound = range[1].parse::<i32>().unwrap();
            large_bound = range[0].parse::<i32>().unwrap();
            diffs.push(large_bound - small_bound);
        }
        else if range[0] < range[1] {
            small_bound = range[0].parse::<i32>().unwrap();
            large_bound = range[1].parse::<i32>().unwrap();
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

fn check_overlap(line: String) -> bool {    
    // Split each line by comma, and then dash
    let mut ranges: Vec<Vec<&str>> = Vec::new();
    let mut split_by_comma: Vec<&str> = line.split(",").collect();
    for range in &split_by_comma {
        let mut split_by_dash: Vec<&str> = range.split("-").collect();
        ranges.push(split_by_dash.clone());
    }
    let mut diffs = find_large_diff(&ranges);
    if ranges[diffs.1][0] >= ranges[diffs.0][0] && ranges[diffs.1][1] <= ranges[diffs.0][1] || ranges[diffs.0][0] >= ranges[diffs.1][0] && ranges[diffs.0][1] >= ranges[diffs.1][1] {
        return true;
    }
    
    //println!("{}\n{}\n----------------------", split_by_comma[0], split_by_comma[1]);
    return false;
}

fn part1() -> i32 {
    let lines = read_input_file();
    let mut overlap_pairs = 0;

    for x in 0..lines.len() {
        if check_overlap(lines[x].clone()){
            overlap_pairs += 1;
        }
    }
    return overlap_pairs;
}

fn main() {
    println!("Part 1 Answer: {}", part1());
    //println!("Part 2 Answer: {}", part2_answer);
}
