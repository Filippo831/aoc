use std::fs;
fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let res: Vec<&str> = line.split("   ").collect();
        left.push(res[0].parse::<i32>().unwrap());
        right.push(res[1].parse::<i32>().unwrap());
    }
    
    left.sort();
    right.sort();
    let mut total_diff: i32 = 0;
    for i in 0..left.len() {
        total_diff = total_diff + (left[i] - right[i]).abs();
    }
    println!("{}", total_diff)
}
