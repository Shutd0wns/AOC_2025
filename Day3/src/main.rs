use std::cmp::{min};
use std::fs::{File};
use std::io::{BufRead, BufReader};

fn main() {
    let solution_result = solve_puzzle("src/input.txt");
    let (solution1, solution2) = match solution_result {
        Ok(val) => val,
        Err(e) => panic!("Solver thrown error: {e}")
    };
    println!("Solution to part one is {solution1}");
    println!("Solution to part two is {solution2}");
}

fn solve_puzzle(file_path: &str) -> Result<(u64, u64), String> {
    let file_result = File::open(file_path);
    let file = match file_result {
        Ok(value) => value,
        Err(e) => return Err(e.to_string())
    };

    let reader = BufReader::new(file);

    let mut joltage_sum = 0;
    let mut joltage_sum_12 = 0;

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };
        match find_joltage(line.as_str(), 2) {
            Some(joltage ) => joltage_sum += joltage,
            None => return Err(String::from("Invalid Format"))
        }
        match find_joltage(line.as_str(), 12) {
            Some(joltage ) => joltage_sum_12 += joltage,
            None => return Err(String::from("Invalid Format"))
        }
    }

    Ok((joltage_sum, joltage_sum_12))
}

fn find_joltage(line: &str, batteries: usize) -> Option<u64> {
    let mut maximums: Vec<u32> = std::iter::repeat(0).take(batteries).collect();

    if line.len() < batteries {
        return None
    }

    let mut i = 0;
    for char in line.chars() {
        let num = match char.to_digit(10) {
            Some(val) => val,
            None => return None
        };
        let lower_bound = batteries - min(line.len() - i, batteries);
        let mut clear = false;
        for j in lower_bound..batteries {
            if clear {
                maximums[j] = 0;
                continue;
            }
            if num > maximums[j] {
                maximums[j] = num;
                clear = true;
            }
        }
        i += 1;
    }

    Some(number_from_vec(&maximums))
}

fn number_from_vec(vec: &Vec<u32>) -> u64 {
    let mut sum = 0_u64;

    let mut i = 1;
    for &val in vec {
        sum += val as u64 * 10_u64.pow(vec.len() as u32 - i);
        i += 1;
    }

    sum
}
