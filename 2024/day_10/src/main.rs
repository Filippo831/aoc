use std::{collections::HashSet, fs};

fn recursion(
    row_index: usize,
    col_index: usize,
    size: usize,
    prev_value: u32,
    matrix: &Vec<Vec<u32>>,
    found_nine: &mut HashSet<(usize, usize)>,
    result: u32,
) -> u32 {
    let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut temp_result: u32 = result;

    if matrix[row_index][col_index] == 9 {
        // This code works for part 1
        // 
        // if found_nine.insert((row_index, col_index)) {
        //     return temp_result + 1;
        // } else {
        //     return temp_result;
        // }
        //
        //-----------------------------------
        //
        // This code works for part 2
        return temp_result + 1;
    }

    for d in directions {
        let next_row_index = (row_index as i32 + d.0) as usize;
        let next_col_index = (col_index as i32 + d.1) as usize;

        if next_row_index < size && next_col_index < size {
            let next_value = matrix[next_row_index][next_col_index];
            if next_value == prev_value + 1 {
                temp_result = recursion(
                    next_row_index,
                    next_col_index,
                    size,
                    prev_value + 1,
                    matrix,
                    found_nine,
                    temp_result,
                );
            }
        }
    }
    return temp_result;
}

fn part1(input: &str) -> u32 {
    let matrix: Vec<Vec<u32>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let size = matrix.len();
    let mut total_result: u32 = 0;
    for (row_index, row) in matrix.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if *col == 0 {
                let partial_result: u32;
                let mut found_nine: HashSet<(usize, usize)> = HashSet::new();
                partial_result = recursion(row_index, col_index, size, 0, &matrix, &mut found_nine, 0);
                total_result = total_result + partial_result;
            }
        }
    }
    return total_result;
}

fn main() {
    println!("{}", part1("input.txt"));
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 36);
    }
}
