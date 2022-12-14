#![allow(dead_code, unused_labels)]
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

// Creats vectors for each stack, which themselves are stored in a Vector (a list of lists)
fn get_initial_stacks(lines: Vec<String>) -> Vec<Vec<char>> {
    let num_stacks = lines[0].len() / 4 + 1;
    let mut stacks: Vec<Vec<char>> = Vec::new();

    // Creates a vector/list for each stack
    for _x in 0..num_stacks {
        let stack: Vec<char> = Vec::new();
        stacks.push(stack);
    }

    // Goes through the lines of input file, and saves the items to their appropriate list/stack
    'outer: for line in &lines {
        let mut current_stack = 0;
        let mut count = 0;
        '_inner: for char in line.chars() {
            if char == '1' {
                break 'outer
            }
            if count > 3 {
                current_stack += 1;
                count = 0;
            }
            if char != '[' && char != ']' && char != ' ' {
                stacks[current_stack].insert(0, char);
            }
            count += 1;
        }
    }
    return stacks;
}

// Parses input file for instructions on moving items, then performs those instructions
fn move_stacks(mut stacks: Vec<Vec<char>>, lines: Vec<String>, part2: bool) -> Vec<Vec<char>> {
    let mut instructions: Vec<Vec<&str>> = Vec::new();
    '_outer: for line in &lines {
        let chars = line.chars();
        'inner: for char in chars {
            if char == 'm' {
                instructions.push(line.split(" ").collect());
                break 'inner;
            }
        } 
    }
    for instruction in instructions {
        let num_to_move = instruction[1].parse::<usize>().unwrap();
        let move_from = instruction[3].parse::<usize>().unwrap() - 1;
        let move_to = instruction[5].parse::<usize>().unwrap() - 1;

        '_outer: for _x in 0..num_to_move {
            let stack_length = stacks[move_from].len();
            if part2 {
                let lower_bound = stack_length - num_to_move;
                'inner: for x in lower_bound..stack_length {
                    let item_to_move = stacks[move_from][x];
                    stacks[move_to].push(item_to_move);
                }
                for _x in lower_bound..stack_length {
                    stacks[move_from].remove(lower_bound);
                }
                break '_outer;
            }
            else {
                let item_to_move = stacks[move_from][stack_length-1];
                stacks[move_to].push(item_to_move);
                stacks[move_from].pop();
            }
        }
    }
    return stacks;
}

// Part 1
fn part1(lines: Vec<String>) -> String {
    let part2 = false;
    let stacks = get_initial_stacks(lines.clone());
    let mut answer = String::from("");
    let moved_stacks = move_stacks(stacks.clone(), lines.clone(), part2);
    for stack in moved_stacks {
        answer.push(stack[stack.len() - 1]);
    }
    return answer
}

// Part 2
fn part2(lines: Vec<String>) -> String {
    let part2 = true;
    let stacks = get_initial_stacks(lines.clone());
    let mut answer = String::from("");
    let moved_stacks = move_stacks(stacks.clone(), lines.clone(), part2);
    for stack in moved_stacks {
        answer.push(stack[stack.len() - 1]);
    }
    return answer
}

fn main() {
    let lines = read_input_file();
    println!("Part 1 Answer: {}", part1(lines.clone()));
    println!("Part 2 Answer: {}", part2(lines.clone()));
}