use regex::Regex;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    let mut result: i32 = 0;
    let results: Vec<(i32, i32)> = re.captures_iter(&input).map(|el| {
        (el["first"].parse::<i32>().unwrap(), el["second"].parse::<i32>().unwrap())
    }).collect();

    for r in results {
        result = result + r.0 * r.1;
    }
    println!("{}", result);
}
