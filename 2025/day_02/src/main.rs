use std::fs;

fn part1(input: &str) -> u64 {
    // split over "," to get every range, split again by "-" to get the lower and upper bound and convert in u64
    let series: Vec<Vec<u64>> = fs::read_to_string(input)
        .unwrap()
        .split(",")
        .map(|e| {
            e.split('\n')
                .nth(0)
                .unwrap()
                .split("-")
                .map(|n| n.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let mut result: u64 = 0;

    for serie in series {
        for number in serie[0]..serie[1] + 1 {
            let length: u32 = (number.ilog10() + 1) as u32;
            if length % 2 == 0 {
                if number / (10_u64.pow(length / 2)) == number % (10_u64.pow(length / 2)) {
                    result = result + number;
                }
            }
        }
    }

    return result;
}

fn part2(input: &str) -> u64 {
    let series: Vec<Vec<u64>> = fs::read_to_string(input)
        .unwrap()
        .split(",")
        .map(|e| {
            e.split('\n')
                .nth(0)
                .unwrap()
                .split("-")
                .map(|n| n.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let mut result: u64 = 0;

    for serie in series {
        for number in serie[0]..serie[1] + 1 {
            let length: u32 = (number.ilog10() + 1) as u32;
            let number_string: Vec<char> = number.to_string().chars().collect();
            for size in 1..length / 2 + 1 {
                let mut correct = true;
                if length % size == 0 {
                    for index in 1..length / size {
                        for el in 0..size {
                            if number_string[el as usize] != number_string[(size * index + el) as usize] {
                                correct = false;
                            }
                        }

                    }
                    if correct {
                        result += number;
                        break;
                    }
                }
            }
        }
    }

    return result;
}

fn main() {
    let result_part1 = part1("input.txt");
    println!("{}", result_part1);
    let result_part2 = part2("input.txt");
    println!("{}", result_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 4174379265);
    }
}
