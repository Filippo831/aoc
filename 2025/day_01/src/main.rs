use std::fs;

fn part1(input: &str) -> u32 {
    let mut position: i32 = 50;
    let mut zero_pass: u32 = 0;
    for line in fs::read_to_string(input).unwrap().lines() {
        if line.chars().nth(0).unwrap() == 'L' {
            position = (position - line[1..].parse::<i32>().unwrap()) % 100;
        } else {
            position = (position + line[1..].parse::<i32>().unwrap()) % 100;
        }
        if position == 0 {
            zero_pass = zero_pass + 1;
        }
    }

    return zero_pass;
}

fn part2(input: &str) -> u32 {
    let mut position: i32 = 50;
    let mut zero_pass: u32 = 0;
    for line in fs::read_to_string(input).unwrap().lines() {
        let line_value = line[1..].parse::<i32>().unwrap();
        let direction = line.chars().nth(0).unwrap() == 'R';
        
        // if line.chars().nth(0).unwrap() == 'R' {
        //     position = position + line_value;
        //     zero_pass = zero_pass + (position / 100) as u32;
        //     position = position % 100;
        //
        // } else {
        //     if position == 0 {
        //         zero_pass = zero_pass + (line_value / 100).abs() as u32;
        //     } else {
        //         if line_value >= position {
        //             zero_pass = zero_pass + ((line_value - position) / 100).abs() as u32 + 1;
        //         }
        //     }
        //     position = (position - line_value) % 100;
        // }
        for n in 0..line_value {
            if (direction) {
                position = (position + 1) % 100;
                if position == 0 {
                    zero_pass = zero_pass + 1;
                }
            } else {
                position = (position - 1) % 100;
                if position == 0 {
                    zero_pass = zero_pass + 1;
                }

            }
        }
    }

    return zero_pass;
}

fn main() {
    let result_part1: u32 = part1("input.txt");
    println!("part1: {}", result_part1);

    let result_part2: u32 = part2("input.txt");
    println!("part2: {}", result_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 6);
    }
}
