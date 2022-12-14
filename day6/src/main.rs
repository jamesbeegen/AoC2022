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

// Checks vector for unique values
fn has_unique_letters(mut chars: Vec<char>, part2: bool) -> bool{
    chars.sort();   // Sort the vector 
    chars.dedup();  // Remove consecutive duplicates
    if part2 {
        if chars.len() == 14{
            return true
        }
    }
    else {
        if chars.len() == 4 {
            return true
        }
    }
    return false;
}


// Part 1
fn part1(lines: Vec<String>) -> i32 {
    let mut char_count = 0;
    let mut chars_list: Vec<char> = Vec::new();
    let chars = lines[0].chars();
    let part2 = false;

    // Add chars from the input line to an indexed list/vec
    for char in chars {
        chars_list.push(char);
    }

    // Select four letter slice, check it for dupes
    for x in 0..chars_list.len() - 3 {
        let four_chars: Vec<char> = vec!(chars_list[x], chars_list[x+1], chars_list[x+2], chars_list[x+3]);
        if has_unique_letters(four_chars, part2){
            break;
        }
        char_count += 1;
    }

    return char_count + 4
}

// Part 2
fn part2(lines: Vec<String>) -> i32{
    let mut char_count = 0;
    let mut chars_list: Vec<char> = Vec::new();
    let chars = lines[0].chars();

    // Add chars from the input line to an indexed list/vec
    for char in chars {
        chars_list.push(char);
    }

    // Select four letter slice, check it for dupes
    for x in 0..chars_list.len() - 14 {
        let mut fourteen_chars: Vec<char> = Vec::new();
        let part2 = true;
        //vec!(chars_list[x], chars_list[x+1], chars_list[x+2], chars_list[x+3]);
        for i in x..x+14 {
            fourteen_chars.push(chars_list[i]);
        }
        if has_unique_letters(fourteen_chars, part2){
            break;
        }
        char_count += 1;
    }

    return char_count + 14
}

fn main() {
    let lines = read_input_file();
    println!("Part 1 Answer: {}", part1(lines.clone()));
    println!("Part 2 Answer: {}", part2(lines.clone()));
}