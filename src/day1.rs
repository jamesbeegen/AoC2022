fn find_largest(cal_list: Vec<&str>) -> (i32, i32) {
    let mut current_elf = 1;
    let mut largest_elf = 1;
    let mut largest_cals = 0;
    let mut temp_sum = 0;

    for cal in cal_list {
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
    return (largest_elf, largest_cals);
}

fn main() {
    let cal_list = vec![
        "1000",
        "2000",
        "3000",
        "",
        "4000",
        "",
        "5000",
        "6000",
        "",
        "7000",
        "8000",
        "9000",
        "",
        "10000"];
    
    let largest = find_largest(cal_list);
    println!("Elf #{} is carrying the most calories: {}", largest.0, largest.1);
}