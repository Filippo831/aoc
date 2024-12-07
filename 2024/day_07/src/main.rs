use std::fs;

fn recursion(
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
        boolean_output | recursion(result, operands, index + 1, temp_internal_value, true);
    boolean_output =
        boolean_output | recursion(result, operands, index + 1, temp_internal_value, false);

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
        if recursion(el.0, &el.1, 0, 0, true) {
            total_sum += el.0;
        }
    }
    return total_sum;
}

fn main() {
    let result = part1("input.txt");
    println!("part1 result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 3749);
    }
}
