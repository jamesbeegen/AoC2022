#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Read lines from puzzle input file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn get_round_score(round: &str, part: i32) -> i32 {
    let mut my_score = 0;
    let rock_pts = 1;
    let paper_pts = 2;
    let scissors_pts = 3;
    let win_pts = 6;
    let tie_pts = 3;
    let elf_chose = round.chars().nth(0).unwrap();
    let mut user_chose = round.chars().nth(2).unwrap();
    
    if part == 2 {
        if elf_chose == 'A' {
            if user_chose == 'X' {
                user_chose = 'Z';
            }
            else if user_chose == 'Y'{
                user_chose = 'X';
            }
            else if user_chose == 'Z'{
                user_chose = 'Y';
            }
        }
        else if elf_chose == 'C' {
            if user_chose == 'X' {
                user_chose = 'Y';
            }
            else if user_chose == 'Y' {
                user_chose = 'Z';
            }
            else if user_chose == 'Z' {
                user_chose = 'X';
            }
        }
    }
    // Get my score
    if user_chose == 'X' {
        my_score += rock_pts;
        if elf_chose =='A' {
            my_score += tie_pts;
        }
        else if elf_chose == 'C' {
            my_score += win_pts;
        }
    }
    else if user_chose == 'Y' {
        my_score += paper_pts;
        if elf_chose == 'A' {
            my_score += win_pts;
        }
        else if elf_chose == 'B' {
            my_score += tie_pts;
        }
    }
    else if user_chose == 'Z'{
        my_score += scissors_pts;
        if elf_chose == 'C' {
            my_score += tie_pts;
        }
        else if elf_chose == 'B' {
            my_score += win_pts;
        }
    }

    return my_score;
}


fn main() {
    let mut part1_total_score = 0;
    let mut part2_total_score = 0;
    if let Ok(lines) = read_lines("src/input.txt") {
        for line in lines {
            if let Ok(round) = line {
                let mut part = 1;
                part1_total_score += get_round_score(&round, part);
                part = 2;
                part2_total_score += get_round_score(&round, part);
            }
        }
    }
    println!("Part 1 Answer: {}\nPart 2 Answer: {}", part1_total_score, part2_total_score);
}
