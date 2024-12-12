use std::{collections::HashMap, fs};

// first approach is wrong
fn part1_first(input: &str) -> u64 {
    let matrix: Vec<Vec<char>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let matrix_size: usize = matrix.len();

    let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    let mut map_fences: HashMap<char, u32> = HashMap::new();
    let mut map_times: HashMap<char, u32> = HashMap::new();

    for (row_index, row) in matrix.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            let value_times = map_times.get(col);
            match value_times {
                Some(val) => map_times.insert(*col, val + 1),
                None => map_times.insert(*col, 1),
            };

            for d in directions {
                let next_elemnt_row = (row_index as i32 + d.0) as usize;
                let next_elemnt_col = (col_index as i32 + d.1) as usize;

                if next_elemnt_row < matrix_size && next_elemnt_col < matrix_size {
                    if matrix[next_elemnt_row][next_elemnt_col] != *col {
                        let value = map_fences.get(col);
                        match value {
                            Some(val) => map_fences.insert(*col, val + 1),
                            None => map_fences.insert(*col, 1),
                        };
                    }
                } else {
                    let value = map_fences.get(col);
                    match value {
                        Some(val) => map_fences.insert(*col, val + 1),
                        None => map_fences.insert(*col, 1),
                    };
                }
            }
        }
    }

    let mut result: u64 = 0;
    for value in map_times {
        let map_fences_return = map_fences.get(&value.0).unwrap();
        result += (value.1 * map_fences_return) as u64;
    }

    return result;
}

fn recursion(
    matrix: &mut Vec<Vec<char>>,
    row_index: usize,
    col_index: usize,
    element: char,
    matrix_size: usize,
    total_element: &mut u64,
    total_fence: &mut u64,
) {
    let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    matrix[row_index][col_index] = element.to_ascii_lowercase();
    *total_element = *total_element + 1;

    for d in directions {
        let next_elemnt_row = (row_index as i32 + d.0) as usize;
        let next_elemnt_col = (col_index as i32 + d.1) as usize;

        if !(next_elemnt_row < matrix_size && next_elemnt_col < matrix_size) {
            *total_fence = *total_fence + 1;
        } else {
            if matrix[next_elemnt_row][next_elemnt_col] == element {
                recursion(
                    matrix,
                    next_elemnt_row,
                    next_elemnt_col,
                    element,
                    matrix_size,
                    total_element,
                    total_fence,
                );
            } else if matrix[next_elemnt_row][next_elemnt_col] == element.to_ascii_lowercase() {
            } else {
                *total_fence = *total_fence + 1;
            }
        }
    }
    return;
}


// this approach is working
fn part1(input: &str) -> u64 {
    let mut matrix: Vec<Vec<char>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let matrix_size = matrix.len();
    let mut row_index: usize = 0;

    let mut results: Vec<(u64, u64)> = vec![];

    while row_index < matrix_size {
        let mut col_index: usize = 0;
        while col_index < matrix_size {
            let element: char = matrix[row_index][col_index];
            if matrix[row_index][col_index].is_uppercase() {
                let mut total_elements: u64 = 0;
                let mut total_fences: u64 = 0;
                recursion(
                    &mut matrix,
                    row_index,
                    col_index,
                    element,
                    matrix_size,
                    &mut total_elements,
                    &mut total_fences,
                );
                results.push((total_elements, total_fences));
            }
            col_index += 1;
        }
        row_index += 1;
    }

    let mut total_result: u64 = 0;
    for el in results {
        total_result += el.0 * el.1;
    }
    return total_result;
}


fn main() {
    println!{"solution to part1: {}", part1("input.txt")};
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1("input_test1.txt"), 140);
    }
    #[test]
    fn test_part1_2() {
        assert_eq!(part1("input_test2.txt"), 772);
    }
    #[test]
    fn test_part1_3() {
        assert_eq!(part1("input_test3.txt"), 1930);
    }
}
