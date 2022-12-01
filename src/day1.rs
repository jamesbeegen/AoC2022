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
    let mut current_elf = 1;
    let mut largest_elf = 1;
    let mut largest_cals = 0;
    let mut temp_sum = 0;

    if let Ok(lines) = read_lines("./puzzle_input/day1-input.txt") {
        for line in lines {
            if let Ok(cal) = line{
                if cal != "" {
                    temp_sum += cal.parse::<i32>().unwrap();
                }
                else {
                    if temp_sum > largest_cals {
                        largest_cals = temp_sum;
                        largest_elf = current_elf;
                    }
                    current_elf += 1;
                    temp_sum = 0;
                }
            }
        }
    }
    println!("Elf #{} is carrying the most calories: {}", largest_elf, largest_cals);
}