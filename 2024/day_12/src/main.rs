use std::{collections::HashMap, fs};

// first approach not good
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
        println!("{}={}:{}", value.0, value.1, map_fences_return);
        result += (value.1 * map_fences_return) as u64;
    }

    return result;
}
fn part1(input: &str) {
    let matrix: Vec<Vec<char>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
}


fn main() {
    part1("input.txt");
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
