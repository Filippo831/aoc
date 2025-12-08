use std::{collections::HashMap, fs};

fn part1(input: &str) -> u64 {
    let content: Vec<Vec<u32>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.split(",").map(|e| e.parse::<u32>().unwrap()).collect())
        .collect();

    let mut pair_distance: HashMap<u32, (usize, usize)> = HashMap::new();

    // calucalte distances
    for first_index in 0..content.len() {
        let mut min_distance: u32 = u32::max_value();
        let mut pair: (usize, usize) = (0, 0);
        for second_index in 0..content.len() {
            if first_index != second_index {
                let mut distance = 0;
                for axis in 0..3 {
                    distance = distance
                        + (content[first_index][axis] as i32 - content[second_index][axis] as i32)
                            .pow(2) as u32;
                }
                if distance < min_distance && !pair_distance.contains_key(&distance) {
                    min_distance = distance;
                    pair = (first_index, second_index);
                }
            }
        }
        pair_distance.insert(min_distance, pair);
    }

    let mut ordered_distance: Vec<&u32> = pair_distance.keys().collect();
    ordered_distance.sort();
    // for i in 0..20 {
    //     let pair = &pair_distance.get(ordered_distance[i]).unwrap();
    //     dbg!("{} : {}", &ordered_distance[i], (&content[pair.0], &content[pair.1]));
    // }

    let mut groups: [u32; 1000] = [0; 1000];
    let mut pair_numbers = 1;

    for el in 0..10 {
        let couple: &(usize, usize) = pair_distance.get(ordered_distance[el]).unwrap();

        if groups[couple.0] == 0 && groups[couple.1] == 0 {
            groups[couple.0] = pair_numbers;
            groups[couple.1] = pair_numbers;
            pair_numbers = pair_numbers + 1;
        } else if groups[couple.0] != 0 && groups[couple.1] == 0 {
            groups[couple.1] = groups[couple.0];
        } else if groups[couple.1] != 0 && groups[couple.0] == 0 {
            groups[couple.0] = groups[couple.1];
        }
    }

    let mut sizes: Vec<u32> = vec![0; pair_numbers as usize];
    for index in 0..1000 {
        if groups[index] != 0 {
            sizes[groups[index] as usize] += 1;
        }
    }
    sizes.sort();
    sizes.reverse();
    dbg!("{}", &sizes);
    return (sizes[0] * sizes[1] * sizes[2]) as u64;
}

fn part2(input: &str) -> u64 {
    return 0;
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
        assert_eq!(part1("input_test.txt"), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 3263827);
    }
}
