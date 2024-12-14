use regex::{self, Regex};
use std::fs;

fn part1(input: &str, x_size: usize, y_size: usize) -> u32 {
    let re = Regex::new(r"-?\d*\.{0,1}\d+").unwrap();

    let mut robots: Vec<Vec<i32>> = vec![];
    let mut last_positions: Vec<(i32, i32)> = vec![];

    for (line_index, line) in fs::read_to_string(input).unwrap().lines().enumerate() {
        robots.push(vec![]);
        for value in re.find_iter(line) {
            robots[line_index].push(value.as_str().parse::<i32>().unwrap());
        }
    }

    for robot in robots {
        let mut final_x;
        let mut final_y;

        if robot[2] > 0 {
            final_x = (robot[0] + robot[2] * 100) % x_size as i32;
        } else {
            final_x = x_size as i32 + ((robot[0] + robot[2] * 100) % x_size as i32);
            if final_x == x_size as i32 {
                final_x = 0;
            }
        }

        if robot[3] > 0 {
            final_y = (robot[1] + robot[3] * 100) % y_size as i32;
        } else {
            final_y = y_size as i32 + ((robot[1] + robot[3] * 100) % y_size as i32);
            if final_y == y_size as i32 {
                final_y = 0;
            }
        }

        last_positions.push((final_x, final_y));
    }
    let mut quadrant_risk: [u32; 4] = [0, 0, 0, 0];

    for position in last_positions {
        let mut quadrant_to_go = 0; // (position.0 < x_size / 2) + (position.0 < y_size / 2) * 2;
        if position.0 > (x_size / 2) as i32 {
            quadrant_to_go += 1;
        }
        if position.1 > (y_size / 2) as i32 {
            quadrant_to_go += 2;
        }
        if position.0 != (x_size / 2) as i32 && position.1 != (y_size / 2) as i32 {
            quadrant_risk[quadrant_to_go] += 1;
        }
    }
    let mut result = 1;

    for q in quadrant_risk {
        result *= q;
    }

    return result;
}

fn main() {
    println!("{}", part1("input.txt", 101, 103));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt", 11, 7), 12)
    }
}
