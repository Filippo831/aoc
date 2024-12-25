use std::fs;

fn part1(input: &str) -> u32 {
    let mut keys: Vec<Vec<u32>> = vec![];
    let mut locks: Vec<Vec<u32>> = vec![];
    for block in fs::read_to_string(input).unwrap().split("\n\n") {
        let mut is_key = false;
        let mut temp: Vec<u32> = vec![0; 5];
        for (line_index, line) in block.lines().enumerate() {
            if line_index == 0 {
                if line == "....." {
                    is_key = true;
                }
            } else if line_index < 6 {
                for (char_index, my_char) in line.chars().enumerate() {
                    if my_char == '#' {
                        temp[char_index] += 1;
                    }
                }
            }
        }
        if is_key {
            keys.push(temp);
        } else {
            locks.push(temp);
        }
    }
    let mut result: u32 = 0;
    for key in &keys {
        for lock in &locks {
            let mut overlap: bool = false;
            for i in 0..5 {
                if key[i] + lock[i] > 5 {
                    overlap = true;
                }
            }
            if !overlap {
                result += 1;
            }
        }
    }
    return result;
}

fn main() {
    println!("part1: {}", part1("input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 3)
    }
}
