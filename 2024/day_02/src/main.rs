use std::fs;

fn part1(input: &str) -> u32 {
    let mut total_good: u32 = 0;

    for line in fs::read_to_string(input).unwrap().lines() {
        let mut numbers: Vec<i32> = Vec::new();
        line.split(" ")
            .for_each(|c| numbers.push(c.parse::<i32>().unwrap()));

        let greater: bool = numbers[0] > numbers[1];

        let mut good: bool = true;

        for my_slice in numbers.windows(2) {
            if my_slice[0] == my_slice[1] {
                good = false
            }
            if (my_slice[0] > my_slice[1]) != greater {
                good = false
            } else if (my_slice[0] - my_slice[1]).abs() > 3 {
                good = false
            }
        }
        if good {
            total_good = total_good + 1;
        }
    }
    return total_good;
}

fn part2(input: &str) -> u32 {
    let mut total_good: u32 = 0;

    for line in fs::read_to_string(input).unwrap().lines() {
        let mut numbers: Vec<i32> = Vec::new();
        line.split(" ")
            .for_each(|c| numbers.push(c.parse::<i32>().unwrap()));

        let mut good_true: bool = false;

        let mut greater: bool = true;
        for i in 0..numbers.len() {
            let mut here_good = true;
            let mut numbers_copy = numbers.clone();
            numbers_copy.remove(i);
            for my_slice in numbers_copy.windows(2) {
                if my_slice[0] == my_slice[1]
                    || (my_slice[0] > my_slice[1]) != greater
                    || (my_slice[0] - my_slice[1]).abs() > 3
                {
                    here_good = false;
                    break;
                }
            }
            good_true = good_true | here_good;
        }

        let mut good_false: bool = false;

        greater = false;
        for i in 0..numbers.len() {
            let mut here_good = true;
            let mut numbers_copy = numbers.clone();
            numbers_copy.remove(i);
            for my_slice in numbers_copy.windows(2) {
                if my_slice[0] == my_slice[1]
                    || (my_slice[0] > my_slice[1]) != greater
                    || (my_slice[0] - my_slice[1]).abs() > 3
                {
                    here_good = false;
                    break;
                }
            }
            good_false = good_false | here_good;
        }

        if good_false || good_true {
            total_good += 1;
        }
    }

    return total_good;
}
fn main() {
    println!("part1: {}", part1("input.txt"));
    println!("part2: {}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 4);
    }
}
