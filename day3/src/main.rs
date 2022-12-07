#![allow(dead_code, unused_mut)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

// Read lines from puzzle input file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Creates character to value mapping
fn setup_value_table(hm: &mut HashMap<char, String>) {
    let letters = vec!('a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z');
    for x in 1..53{
        let s: String = x.to_string();
        hm.insert(letters[x-1], s);
    }
}

// For part 1 - gets the item found in both halfs of the rucksack
fn get_priority_item(line: String) -> char {
    let mut first_half: Vec<char> = Vec::new();
    let mut second_half: Vec<char> = Vec::new();
    let half: usize = line.len() / 2;
    let mut i = 0;
    let mut matching_char = 'n';

    for letter in line.chars() {
        if i >= half {
            second_half.push(letter);
        }
        else{
            first_half.push(letter);
        }
        i += 1;
    }

    for x in 0..first_half.len() {
        for j in 0..second_half.len(){
            if first_half[x] == second_half[j]{
                matching_char = first_half[x];
            }
        }
        
    }
    return matching_char;
}

// Does all of part 2
fn part2(sacks: &mut Vec<String>, value_table: &mut HashMap<char, String>) -> i32 {
    let mut current_sacks: Vec<String> = Vec::new();
    let mut sack1_letters: Vec<char> = Vec::new();
    let mut sack2_letters: Vec<char> = Vec::new();
    let mut sack3_letters: Vec<char> = Vec::new();
    let mut total = 0;
    let mut group = 1;
    let mut prev = 0;

    for x in 0..sacks.len() + 1 {
        if x + 3 == prev + 6 || x == 3{
            prev = x;

            for char in current_sacks[0].chars() {
                sack1_letters.push(char);
            }

            for char in current_sacks[1].chars() {
                sack2_letters.push(char);
            }

            for char in current_sacks[2].chars() {
                sack3_letters.push(char);
            }
            
            for char in &sack1_letters {
                if sack2_letters.contains(&char) && sack3_letters.contains(&char) {
                    total += value_table.get(& char).unwrap().parse::<i32>().unwrap();
                    println!("Elf group {}:\nMatch: {}\nValue: {}", group, char, value_table.get(& char).unwrap().parse::<i32>().unwrap());
                    break;
                }
            }

            current_sacks.clear();
            sack1_letters.clear();
            sack2_letters.clear();
            sack3_letters.clear();
        }

        if x == sacks.len() {
            break;
        }
        else {
            current_sacks.push(sacks[x].clone());
        }
    }
    return total;
}

fn main() {
  // Creates the character to value mapping
    let value_table: &mut HashMap<char, String> = &mut HashMap::new();
    setup_value_table(value_table);
    
    let mut total = 0;
    let sacks: &mut Vec<String> = &mut Vec::new();
    
    if let Ok(lines) = read_lines("src/input.txt") {
        for line in lines {
            if let Ok(line) = line{
                let l: String = line;
                sacks.push(l.clone());
                let item = get_priority_item(l.clone());
                total += value_table.get(& item).unwrap().parse::<i32>().unwrap();
            }
        }
    }
    println!("Part 1 Answer: {}", total);
    println!("Part 2 Answer: {}", part2(sacks, value_table));
}