use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn part1(input: &str) -> u64 {
    let mut numbers: Vec<u64> = vec![];
    for line in fs::read_to_string(input).unwrap().lines() {
        numbers.push(line.parse::<u64>().unwrap());
    }
    let mut results: Vec<u64> = vec![];
    for number in numbers {
        let mut result = number;
        for i in 0..2000 {
            let mut prev = result;
            result = result << 6;
            result = prev ^ result;
            result = result & 16777215;

            prev = result;
            result = result >> 5;
            result = prev ^ result;
            result = result & 16777215;

            prev = result;
            result = result << 11;
            result = prev ^ result;
            result = result & 16777215;
        }
        results.push(result);
    }

    let mut output: u64 = 0;

    for n in results {
        output += n;
    }

    return output;
}

fn part2(input: &str) -> u32 {
    let mut numbers: Vec<u64> = vec![];
    for line in fs::read_to_string(input).unwrap().lines() {
        numbers.push(line.parse::<u64>().unwrap());
    }

    let mut total: HashMap<(i8, i8, i8, i8), u32> = HashMap::new();

    for number in numbers {
        println!("{}", number);
        let mut changes: Vec<i8> = vec![];
        let mut prices: Vec<u8> = vec![];
        prices.push((number % 10) as u8);
        let mut result = number;
        for i in 0..2000 {
            let mut prev = result;
            result = result << 6;
            result = prev ^ result;
            result = result & 16777215;

            prev = result;
            result = result >> 5;
            result = prev ^ result;
            result = result & 16777215;

            prev = result;
            result = result << 11;
            result = prev ^ result;
            result = result & 16777215;

            let this_price: u8 = (result % 10) as u8;
            // println!("{} - {} = {}", this_price, prices.last().unwrap(), this_price - prices.last().unwrap());
            changes.push(this_price as i8 - *prices.last().unwrap() as i8);
            prices.push(this_price);
        }
        let mut index = 4;
        let mut seen: HashSet<(i8, i8, i8, i8)> = HashSet::new();
        for i in changes.windows(4) {
            let tuple: (i8, i8, i8, i8) = (i[0], i[1], i[2], i[3]);
            if seen.get(&tuple).is_none() {
                match total.get(&tuple) {
                    Some(value) => {
                        total.insert(tuple, value + *prices.get(index).unwrap() as u32);
                    }
                    None => {
                        total.insert(tuple, *prices.get(index).unwrap() as u32);
                    }
                };
                seen.insert(tuple);
            }
            index += 1;
            // if index == 2000 {
            //     break;
            // }
        }
    }

    let mut output: u32 = 0;

    for t in total {
        if t.1 > output {
            output = t.1;
        }
    }

    return output;
}

fn main() {
    println!("{}", part1("input.txt"));
    println!("{}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test_part1.txt"), 37327623);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test_part2.txt"), 23);
    }
}
