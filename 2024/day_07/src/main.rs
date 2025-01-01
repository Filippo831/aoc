use std::fs;

fn recursion_part1(
    result: i64,
    operands: &Vec<i64>,
    index: usize,
    internal_value: i64,
    this_operand: bool,
) -> bool {
    let mut boolean_output: bool = false;
    let mut temp_internal_value = internal_value;

    if this_operand {
        temp_internal_value = temp_internal_value + operands[index];
    } else {
        temp_internal_value = temp_internal_value * operands[index];
    }

    if internal_value > result {
        return false;
    }

    if index == operands.len() - 1 {
        if result == temp_internal_value {
            return true;
        } else {
            return false;
        }
    }
    boolean_output =
        boolean_output | recursion_part1(result, operands, index + 1, temp_internal_value, true);
    boolean_output =
        boolean_output | recursion_part1(result, operands, index + 1, temp_internal_value, false);

    return boolean_output;
}

fn part1(input: &str) -> i64 {
    let mut parameters: Vec<(i64, Vec<i64>)> = vec![];

    for line in fs::read_to_string(input).unwrap().lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let result_sum = parts[0].parse::<i64>().unwrap();
        let operands = parts[1]
            .split(" ")
            .map(|e| e.parse::<i64>().unwrap())
            .collect();

        parameters.push((result_sum, operands));
    }

    let mut total_sum: i64 = 0;

    for el in parameters {
        if recursion_part1(el.0, &el.1, 0, 0, true) {
            total_sum += el.0;
        }
    }
    return total_sum;
}

fn recursion_part2(
    result: u64,
    operands: &Vec<u64>,
    index: usize,
    internal_value: u64,
    this_operand: u8,
) -> bool {
    let mut boolean_output: bool = false;
    let mut temp_internal_value = internal_value;

    if this_operand == 0 {
        temp_internal_value = temp_internal_value + operands[index];
    } else if this_operand == 1{
        temp_internal_value = temp_internal_value * operands[index];
    } else {
        temp_internal_value = format!("{}{}", temp_internal_value, operands[index]).parse::<u64>().unwrap();
    }

    if internal_value > result {
        return false;
    }

    if index == operands.len() - 1 {
        if result == temp_internal_value {
            return true;
        } else {
            return false;
        }
    }
    boolean_output =
        boolean_output | recursion_part2(result, operands, index + 1, temp_internal_value, 0);
    boolean_output =
        boolean_output | recursion_part2(result, operands, index + 1, temp_internal_value, 1);
    boolean_output =
        boolean_output | recursion_part2(result, operands, index + 1, temp_internal_value, 2);

    return boolean_output;
}

fn part2(input: &str) -> u64 {
    let mut parameters: Vec<(u64, Vec<u64>)> = vec![];

    for line in fs::read_to_string(input).unwrap().lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let result_sum = parts[0].parse::<u64>().unwrap();
        let operands = parts[1]
            .split(" ")
            .map(|e| e.parse::<u64>().unwrap())
            .collect();

        parameters.push((result_sum, operands));
    }

    let mut total_sum: u64 = 0;

    for el in parameters {
        if recursion_part2(el.0, &el.1, 0, 0, 0) {
            total_sum += el.0;
        }
    }
    return total_sum;
}
fn main() {
    println!("part1 result: {}", part1("input.txt"));
    println!("part2 result: {}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 3749);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 11387);
    }
}
