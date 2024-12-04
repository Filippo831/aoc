use std::fs;

fn main() {
    let word = "XMAS";

    let mut total_found: i32 = 0;

    let directions: Vec<(i32, i32)> = vec![
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let matrix: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    for (row_index, row) in matrix.iter().enumerate() {
        for (el_index, el) in row.iter().enumerate() {
            if el == &'X' {
                for dir in &directions {
                    if (searchWork(word, 0, &matrix, row_index, el_index, *dir)) {
                        total_found += 1;
                    }
                }
            }
        }
    }
    println!("{}", total_found);
}

fn searchWork(
    word: &str,
    word_index: usize,
    matrix: &Vec<Vec<char>>,
    row_index: usize,
    el_index: usize,
    direction: (i32, i32),
) -> bool {
    if row_index >= matrix.len() {
        return false;
    }
    if el_index >= matrix[0].len() {
        return false;
    }
    if matrix[row_index][el_index] == word.chars().nth(word_index).unwrap() {
        if word.chars().nth(word_index).unwrap() == 'S' {
            return true;
        }
        return searchWork(
            word,
            word_index + 1,
            matrix,
            (row_index as i32 + direction.0) as usize,
            (el_index as i32 + direction.1) as usize,
            direction,
        );
    }

    return false;
}
