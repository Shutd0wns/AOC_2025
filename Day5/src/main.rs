use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let (solution1, solution2) = match solve_puzzle("src/input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Solver failed with error {e}")
    };

    println!("Solution to part one: {solution1}");
    println!("Solution to part two: {solution2}");
}

fn solve_puzzle(file_path: &str) -> Result<(u64, u64), String> {
    let file_result = File::open(file_path);
    let file = match file_result {
        Ok(value) => value,
        Err(e) => return Err(e.to_string())
    };

    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines().into_iter();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut fresh = 0_u64;

    for line_result in lines_iter.by_ref() {
        let line = match line_result {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };
        if line == ""{
            break
        }

        let split_line: Vec<&str> = line.split('-').collect();

        if split_line.len() != 2 {
            return Err("Invalid Format".to_string());
        }

        let mut lower = match split_line[0].parse::<u64>() {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };
        let mut upper = match split_line[1].parse::<u64>() {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };

        for elem in &mut ranges  {
            // Moves range boundaries depending on existing ranges
            if elem.0 <= lower && lower <= elem.1 {
                lower = elem.1 + 1
            }
            if elem.0 <= upper && upper <= elem.1 {
                upper = elem.0 - 1
            }
            // Make it range 1-0 (so that it calculates as 0 ids) if found range is subset of input range
            if lower <= elem.0 && upper >= elem.1 {
                elem.0 = 1;
                elem.1 = 0;
            }
        }
        if lower <= upper {
            ranges.push((lower, upper))
        }
    }

    for line_result in lines_iter {
        let line = match line_result {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };

        let num = match line.parse::<u64>() {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };

        for &(lower, upper) in &ranges {
            if lower <= num && num <= upper {
                fresh += 1;
                break
            }
        }
    }
    
    let fresh_id_count: u64 = ranges.iter().map(|&(lower, upper)| upper + 1 - lower).sum();

    Ok((fresh,fresh_id_count))
}