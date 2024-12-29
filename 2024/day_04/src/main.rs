use std::fs;

fn part1(input: &str) -> i32 {
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

    let matrix: Vec<Vec<char>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    for (row_index, row) in matrix.iter().enumerate() {
        for (el_index, el) in row.iter().enumerate() {
            if el == &'X' {
                for dir in &directions {
                    if (search_word(word, 0, &matrix, row_index, el_index, *dir)) {
                        total_found += 1;
                    }
                }
            }
        }
    }
    return total_found;
}

fn search_word(
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
        return search_word(
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

fn part2(input: &str) -> u64 {
    let matrix: Vec<Vec<char>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut total_found: u64 = 0;

    let directions: [[i32; 2]; 4] = [[1, 1], [-1, 1], [-1, -1], [1, -1]];

    let size = matrix.len();

    for (row_index, row) in matrix.iter().enumerate() {
        for (el_index, el) in row.iter().enumerate() {
            if el == &'A' {
                let mut partial_result: [char; 4] = ['.', '.', '.', '.'];
                for (index_dir, dir) in directions.iter().enumerate() {
                    match calc_position(el_index, row_index, &dir, size) {
                        Ok(value) => {
                            partial_result[index_dir] = matrix[value.1][value.0];
                        }
                        Err(error) => {}
                    }
                }
                if ((partial_result[0] == 'M' && partial_result[2] == 'S')
                    || (partial_result[0] == 'S' && partial_result[2] == 'M'))
                    && ((partial_result[1] == 'M' && partial_result[3] == 'S')
                        || (partial_result[1] == 'S' && partial_result[3] == 'M'))
                {
                    total_found += 1;
                }
            }
        }
    }

    return total_found;
}

fn calc_position(
    x: usize,
    y: usize,
    dir: &[i32; 2],
    size: usize,
) -> Result<(usize, usize), String> {
    let final_x = (x as i32 + dir[0]) as usize;
    let final_y = (y as i32 + dir[1]) as usize;

    if final_x < size && final_y < size {
        return Ok((final_x, final_y));
    } else {
        return Err("errore".to_string());
    }
}

fn main() {
    println!("part1: {}", part1("input.txt"));
    println!("part2: {}", part2("input.txt"));

}
