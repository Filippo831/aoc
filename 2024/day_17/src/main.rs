fn part1(val_reg_a: u64, val_reg_b: u64, val_reg_c: u64, instr: &str) -> String {
    let mut values: [u64; 7] = [0, 1, 2, 3, val_reg_a, val_reg_b, val_reg_c];

    let mut pc: usize = 0;

    let instructions: Vec<usize> = instr
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    let mut result: String = "".to_string();

    while pc < instructions.len() {
        let opcode: usize = instructions[pc];
        let operand: usize = instructions[pc + 1];

        match opcode {
            0 => {
                values[4] = values[4] / u64::pow(2, values[operand] as u32);
                pc += 2;
            }
            1 => {
                values[5] = values[5] ^ operand as u64;
                pc += 2;
            }
            2 => {
                values[5] = values[operand] % 8;
                pc += 2;
            }
            3 => {
                if values[4] != 0 {
                    pc = operand;
                } else {
                    pc += 2;
                }
            }
            4 => {
                values[5] = values[5] ^ values[6];
                pc += 2;
            }
            5 => {
                let value_to_output = values[operand] % 8;
                result.push_str(&value_to_output.to_string());
                result.push(',');
                pc += 2;
            }
            6 => {
                values[5] = values[4] / u64::pow(2, values[operand] as u32);
                pc += 2;
            }
            7 => {
                values[6] = values[4] / u64::pow(2, values[operand] as u32);
                pc += 2;
            }
            _ => {}
        }
    }
    result = result.trim_matches(',').to_string();

    return result;
}

fn part2(val_reg_a: u64, val_reg_b: u64, val_reg_c: u64, instr: &str) -> u64 {
    let mut a: u64 = 0;
    loop {
        if part1(a, val_reg_b, val_reg_c, instr) == instr {
            if a != val_reg_a {
                break;
            }
        }
        a += 1;
    }
    return a;
}

fn main() {
    println!(
        "part1: {}",
        part1(47719761, 0, 0, "2,4,1,5,7,5,0,3,4,1,1,6,5,5,3,0")
    );
    println!(
        "part2: {}",
        part2(47719761, 0, 0, "2,4,1,5,7,5,0,3,4,1,1,6,5,5,3,0")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(10, 0, 0, "5,0,5,1,5,4"), "0,1,2");
    }
    #[test]
    fn test_part1_2() {
        assert_eq!(part1(2024, 0, 0, "0,1,5,4,3,0"), "4,2,5,6,7,7,7,7,3,1,0");
    }
    #[test]
    fn test_part1_3() {
        assert_eq!(part1(729, 0, 0, "0,1,5,4,3,0"), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(2024, 0, 0, "0,3,5,4,3,0"), 117440);
    }
}
