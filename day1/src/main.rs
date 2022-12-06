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

fn main() {
    let mut largest_cals1 = 0;
    let mut largest_cals2 = 0;
    let mut largest_cals3 = 0;
    let mut temp_sum = 0;

    if let Ok(lines) = read_lines("src/input.txt") {
        for line in lines {
            if let Ok(cal) = line {
                if cal != "" {
                    temp_sum += cal.parse::<i32>().unwrap();
                }
                else {
                    if temp_sum > largest_cals1 {
                        largest_cals3 = largest_cals2;
                        largest_cals2 = largest_cals1;
                        largest_cals1 = temp_sum;
                    }
                    else if temp_sum > largest_cals2 {
                        largest_cals3 = largest_cals2;
                        largest_cals2 = temp_sum;
                    }
                    else if temp_sum > largest_cals3 {
                        largest_cals3 = temp_sum;
                    }
                    temp_sum = 0;
                }
            }
        }
    }
    // Part 1
    println!("The most calories carried by an elf is {}", largest_cals1);

    // Part 2
    println!("The total number of carried by the top three elves is {} calories", largest_cals1 + largest_cals2 + largest_cals3);
}
