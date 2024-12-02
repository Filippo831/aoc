use std::fs;

fn part1() {
    let mut total_good: i32 = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let mut numbers: Vec<i32> = Vec::new();
        line.split(" ")
            .for_each(|c| numbers.push(c.parse::<i32>().unwrap()));

        let greater: bool = numbers[0] > numbers[1];

        let mut good: bool = true;

        for my_slice in numbers.windows(2) {
            if my_slice[0] == my_slice[1] {
                good = false
            }
            if (my_slice[0] > my_slice[1]) != greater {
                good = false
            } else if (my_slice[0] - my_slice[1]).abs() > 3 {
                good = false
            }
        }
        if good {
            // for n in numbers {
            //     print!("{} ", n);
            // }
            total_good = total_good + 1;
        }
        // println!();
    }
    println!("{}", total_good);
}

fn part2() {
    let mut total_good: i32 = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let mut numbers: Vec<i32> = Vec::new();
        line.split(" ")
            .for_each(|c| numbers.push(c.parse::<i32>().unwrap()));

        let greater: bool = numbers[0] > numbers[1];

        let mut good: bool = true;

        for (index, my_slice) in numbers.windows(2).enumerate() {
            if my_slice[0] == my_slice[1] {
                good = false
            }
            if (my_slice[0] > my_slice[1]) != greater {
                good = false
            } else if (my_slice[0] - my_slice[1]).abs() > 3 {
                good = false
            }
            if !good {
                numbers.remove(index);
            }
        }
        if good {
            total_good = total_good + 1;
        }     }
    println!("{}", total_good);
}
fn main() {
    // part1();
    part2();
}
