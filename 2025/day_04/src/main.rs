use std::fs;

fn part1(input: &str) -> u64 {
    let adjacent_positions: [(i32, i32); 8] = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    // create a map with true where there is a piece of paper and a false where there isn't
    let mut map: Vec<Vec<bool>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == '@' { true } else { false })
                .collect()
        })
        .collect();

    let mut moved_paper: u64 = 0;
    for _loops in 0..1 {
        let prev_value = moved_paper;
        for row_index in 0..map.len() {
            for col_index in 0..map[0].len() {
                if map[row_index][col_index] {
                    let mut total_adjacent_paper: u8 = 0;
                    for position in 0..8 {
                        let row_position: usize =
                            (row_index as i32 + adjacent_positions[position].0) as usize;
                        let col_position: usize =
                            (col_index as i32 + adjacent_positions[position].1) as usize;

                        if (row_position < map.len()) && (col_position < map[0].len()) {
                            if map[row_position][col_position] {
                                total_adjacent_paper = total_adjacent_paper + 1;
                            }
                        }
                    }
                    if total_adjacent_paper < 4 {
                        moved_paper = moved_paper + 1;
                        // map[row_index][col_index] = false;
                    }
                }
            }
        }
        if prev_value == moved_paper {
            break;
        }
    }
    return moved_paper;
}

fn part2(input: &str) -> u64 {
    let adjacent_positions: [(i32, i32); 8] = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    // create a map with true where there is a piece of paper and a false where there isn't
    let mut map: Vec<Vec<bool>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == '@' { true } else { false })
                .collect()
        })
        .collect();

    let mut moved_paper: u64 = 0;
    for _loops in 0..500 {
        let prev_value = moved_paper;
        for row_index in 0..map.len() {
            for col_index in 0..map[0].len() {
                if map[row_index][col_index] {
                    let mut total_adjacent_paper: u8 = 0;
                    for position in 0..8 {
                        let row_position: usize =
                            (row_index as i32 + adjacent_positions[position].0) as usize;
                        let col_position: usize =
                            (col_index as i32 + adjacent_positions[position].1) as usize;

                        if (row_position < map.len()) && (col_position < map[0].len()) {
                            if map[row_position][col_position] {
                                total_adjacent_paper = total_adjacent_paper + 1;
                            }
                        }
                    }
                    if total_adjacent_paper < 4 {
                        moved_paper = moved_paper + 1;
                        map[row_index][col_index] = false;
                    }
                }
            }
        }
        if prev_value == moved_paper {
            println!("sono passato qui");
            break;
        }
    }
    return moved_paper;
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
        assert_eq!(part1("input_test.txt"), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 43);
    }
}
