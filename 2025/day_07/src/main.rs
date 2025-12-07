use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn part1(input: &str) -> u64 {
    let binding = fs::read_to_string(input).unwrap();
    let content: Vec<&str> = binding.lines().collect();

    let source: usize = content[0].find('S').unwrap();

    let split: Vec<Vec<usize>> = content[1..]
        .iter()
        .map(|c| c.match_indices("^").map(|(i, _)| i).collect())
        .collect();

    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(source);

    let mut split_number: usize = 0;

    for index in 1..content.len() {
        if !(split[index - 1].is_empty()) {
            for s in split.get(index - 1).unwrap() {
                if beams.contains(s) {
                    split_number = split_number + 1;
                    beams.remove(s);
                    beams.insert(s - 1);
                    beams.insert(s + 1);
                }
            }
        }
    }

    return split_number as u64;
}

fn recursion(split: &Vec<Vec<usize>>, col: usize, mut row: usize) -> usize {
    loop {
        if row == split.len() {
            return 1;
        }
        if !(split.get(row - 1).unwrap().is_empty()) {
            if split.get(row - 1).unwrap().contains(&col) {
                return recursion(split, col + 1, row + 1) + recursion(split, col - 1, row + 1);
            }
        }
        row = row + 1;
    }
}
fn part2(input: &str) -> u64 {
    let binding = fs::read_to_string(input).unwrap();
    let content: Vec<&str> = binding.lines().collect();

    let source: usize = content[0].find('S').unwrap();

    let split: Vec<Vec<usize>> = content[1..]
        .iter()
        .map(|c| c.match_indices("^").map(|(i, _)| i).collect())
        .collect();

    return recursion(&split, source, 1) as u64;
}
fn part2_alt(input: &str) -> u64 {
    let binding = fs::read_to_string(input).unwrap();
    let content: Vec<&str> = binding.lines().collect();

    let source: usize = content[0].find('S').unwrap();

    let split: Vec<Vec<usize>> = content[1..]
        .iter()
        .map(|c| c.match_indices("^").map(|(i, _)| i).collect())
        .collect();

    let mut beams: HashMap<usize, usize> = HashMap::new();
    beams.insert(source, 1);

    let mut split_number: usize = 0;

    for index in 1..content.len() {
        if !(split[index - 1].is_empty()) {
            for s in split.get(index - 1).unwrap() {
                if beams.contains_key(s) {
                    if beams.contains_key(&(s - 1)) {
                        beams.insert(
                            s - 1,
                            *beams.get(&(s - 1)).unwrap() + *beams.get(s).unwrap(),
                        );
                    } else {
                        beams.insert(s - 1, *beams.get(s).unwrap());
                    }
                    if beams.contains_key(&(s + 1)) {
                        beams.insert(
                            s + 1,
                            *beams.get(&(s + 1)).unwrap() + *beams.get(s).unwrap(),
                        );
                    } else {
                        beams.insert(s + 1, *beams.get(s).unwrap());
                    }
                    beams.remove(s);
                }
            }

        }
    }
    for el in beams.values() {
        split_number = split_number + el;
    }

    return split_number as u64;
}

fn main() {
    let result_part1 = part1("input.txt");
    println!("{}", result_part1);
    let result_part2 = part2_alt("input.txt");
    println!("{}", result_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_alt("input_test.txt"), 40);
    }
}
