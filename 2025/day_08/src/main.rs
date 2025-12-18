use std::{collections::HashMap, fs};

fn part1(input: &str, upper_bound: u32) -> u64 {
    let content: Vec<Vec<u32>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.split(",").map(|e| e.parse::<u32>().unwrap()).collect())
        .collect();

    let mut distances: HashMap<u64, Vec<(usize, usize)>> = HashMap::new();

    for first_index in 0..content.len() {
        for second_index in first_index + 1..content.len() {
            let mut distance: u64 = 0;
            for axis in 0..3 {
                distance = distance
                    + (content[first_index][axis] as i64 - content[second_index][axis] as i64)
                        .pow(2) as u64;
            }
            distances
                .entry(distance)
                .or_insert_with(Vec::new)
                .push((first_index, second_index));
        }
    }

    let mut sorted_distances: Vec<&u64> = Vec::from_iter(distances.keys());
    sorted_distances.sort();

    let mut index_to_groups: [u32; 1000] = [0; 1000];
    let mut group_index: u32 = 1;
    let mut index: u32 = 0;

    while index < upper_bound {
        let distance = sorted_distances.get(index as usize).unwrap();

        for el in distances.get(distance).unwrap() {
            if index_to_groups[el.0] == 0 && index_to_groups[el.1] == 0 {
                index_to_groups[el.0] = group_index;
                index_to_groups[el.1] = group_index;
                group_index = group_index + 1;
            } else if index_to_groups[el.0] == 0 {
                index_to_groups[el.0] = index_to_groups[el.1];
            } else if index_to_groups[el.1] == 0 {
                index_to_groups[el.1] = index_to_groups[el.0];
            } else {
                let reference_group = index_to_groups[el.0];
                for value in 0..index_to_groups.len() {
                    if index_to_groups[value] == reference_group {
                        index_to_groups[value] = index_to_groups[el.1];
                    }
                }
            }

            index = index + 1;
        }
    }

    let mut lengths: Vec<u32> = vec![0; 1000];

    for value in index_to_groups {
        if value != 0 {
            lengths[value as usize] += 1;
        }
    }

    lengths.sort();
    lengths.reverse();
    dbg!("{}", &lengths[0..10]);

    return (lengths[0] * lengths[1] * lengths[2]) as u64;
}

fn part2(input: &str) -> u64 {
    let content: Vec<Vec<u64>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.split(",").map(|e| e.parse::<u64>().unwrap()).collect())
        .collect();

    let mut distances: HashMap<u64, Vec<(usize, usize)>> = HashMap::new();

    for first_index in 0..content.len() {
        for second_index in first_index + 1..content.len() {
            let mut distance: u64 = 0;
            for axis in 0..3 {
                distance = distance
                    + (content[first_index][axis] as i64 - content[second_index][axis] as i64)
                        .pow(2) as u64;
            }
            distances
                .entry(distance)
                .or_insert_with(Vec::new)
                .push((first_index, second_index));
        }
    }

    let mut sorted_distances: Vec<&u64> = Vec::from_iter(distances.keys());
    sorted_distances.sort();

    let mut index_to_groups: [u32; 1000] = [0; 1000];
    let mut group_index: u32 = 1;
    let mut index: u32 = 0;
    let mut zero_number = content.len();
    let mut result: u64 = 0;

    loop {
        let distance = sorted_distances.get(index as usize).unwrap();

        for el in distances.get(distance).unwrap() {
            if index_to_groups[el.0] == 0 && index_to_groups[el.1] == 0 {
                index_to_groups[el.0] = group_index;
                index_to_groups[el.1] = group_index;
                group_index = group_index + 1;
                zero_number = zero_number - 2;
            } else if index_to_groups[el.0] == 0 {
                index_to_groups[el.0] = index_to_groups[el.1];
                zero_number = zero_number - 1;
            } else if index_to_groups[el.1] == 0 {
                index_to_groups[el.1] = index_to_groups[el.0];
                zero_number = zero_number - 1;
            } else {
                let reference_group = index_to_groups[el.0];
                for value in 0..index_to_groups.len() {
                    if index_to_groups[value] == reference_group {
                        index_to_groups[value] = index_to_groups[el.1];
                    }
                }
            }
            result = content[el.0][0] * content[el.1][0];

            index = index + 1;
        }
        if zero_number == 0 {
            break
        }
    }

    // let mut lengths: Vec<u32> = vec![0; 1000];
    //
    // for value in index_to_groups {
    //     if value != 0 {
    //         lengths[value as usize] += 1;
    //     }
    // }
    //
    // lengths.sort();
    // lengths.reverse();
    // dbg!("{}", &lengths[0..10]);

    return result as u64;
}

fn main() {
    let result_part1 = part1("input.txt", 1000);
    println!("{}", result_part1);
    let result_part2 = part2("input.txt");
    println!("{}", result_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt", 10), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 25272);
    }
}
