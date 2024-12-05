use std::fs;

fn part1(input_path: &str) -> i32 {
    let binding = fs::read_to_string(&input_path).unwrap();
    let first_read: Vec<&str> = binding.split("\n\n").collect();

    let mut rules: Vec<(i32, i32)> = vec![];
    for line in first_read[0].split("\n").into_iter() {
        let result: Vec<i32> = line.split("|").map(|e| e.parse::<i32>().unwrap()).collect();
        rules.push((result[0], result[1]));
    }

    let mut orders: Vec<Vec<i32>> = first_read[1].trim()
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|number| {
                    number.parse::<i32>().unwrap()
                })
                .collect()
        })
        .collect();

    let mut queue: Vec<i32> = vec![rules[0].0, rules[0].1];

    while true {
        for index in 0..rules.len() {
            if index == rules.len() {
                break
            }
            let position = queue.iter().position(|&e| rules[index].1 == e);
            match position {
                Some(position) => {
                    queue.insert(position, rules[index].0);
                    rules.remove(index);
                    println!("{}", position);
                }
                None => {}
            };
        }
        if rules.len() == 0 {
            break
        }
    }

    for el in queue {
        println!("{}", el);
    }

    return 0;
}

fn main() {
    part1("input_test.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result: i32 = part1("input_test.txt");
        assert_eq!(result, 143)
    }
}
