use std::{collections::{HashMap, HashSet}, fs};

fn part1(input: &str) -> u32 {
    let mut connection: HashMap<&str, HashSet<&str>> = HashMap::new();

    let binding = fs::read_to_string(input).unwrap();
    for line in binding.lines() {
        let parts = line.split("-").collect::<Vec<&str>>();
        match connection.get(&parts[0]) {
            Some(value) => {
                let mut new_value = value.clone();
                new_value.insert(parts[1]);
                connection.insert(parts[0], new_value);
            }
            None => {connection.insert(parts[0], HashSet::from([parts[1]]));}
        }

        match connection.get(&parts[1]) {
            Some(value) => {
                let mut new_value = value.clone();
                new_value.insert(parts[0]);

                connection.insert(parts[1], new_value);
            }
            None => {connection.insert(parts[1], HashSet::from([parts[0]]));}
        }
    }

    let mut triple_connection: HashSet<(&str, &str, &str)> = HashSet::new();

    for conn in &connection {
        for to in conn.1 {
            let other = connection.get(to).unwrap();
            let conn_intersection = conn.1.intersection(other);
            for el in conn_intersection {
                let mut result: Vec<&str> = vec![conn.0, to, el];
                result.sort();
                triple_connection.insert((result[0], result[1], result[2]));
            }
        }
    }

    let mut good_connection: u32 = 0;

    for conn in triple_connection {
        if conn.0.starts_with("t") ||conn.1.starts_with("t") || conn.2.starts_with("t") {
            good_connection += 1;

        }
    }


    return good_connection;
}

fn main() {
    println!("part1: {}", part1("input.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 7)
    }
}
