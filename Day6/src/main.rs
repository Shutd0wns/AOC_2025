use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

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

    let solution1 = match solve_part1(file) {
        Ok(val) => val,
        Err(e) => return Err(e)
    };

    let file_result = File::open(file_path);
    let file = match file_result {
        Ok(value) => value,
        Err(e) => return Err(e.to_string())
    };

    let solution2 = match solve_part2(file) {
        Ok(val) => val,
        Err(e) => return Err(e)
    };

    Ok((solution1,solution2))
}

fn solve_part1(file: File) -> Result<u64, String> {
    let reader = BufReader::new(file);

    let num_reg = Regex::new(r"\d+").unwrap();
    let op_reg = Regex::new(r"[\+\*]").unwrap();

    let mut numbers: Vec<Vec<u64>> = Vec::new();

    let mut sum_of_solutions: u64 = 0;

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };

        if op_reg.is_match(line.as_str()) {
            let ops: Vec<&str> = op_reg.find_iter(line.as_str()).map(|x| x.as_str()).collect();
            for index in 0..ops.len() {
                sum_of_solutions += match ops[index] {
                    "+" => numbers.iter().map(|x| x[index]).sum::<u64>(),
                    _ => numbers.iter().map(|x| x[index]).fold(1, |acc, x| acc * x)
                }
            }
            return Ok(sum_of_solutions);
        }

        let mut row: Vec<u64> = Vec::new();

        for found_match in num_reg.find_iter(line.as_str()).into_iter() {
            match found_match.as_str().parse::<u64>() {
                Ok(val) => row.push(val),
                Err(e) => return Err(e.to_string())
            }
        }

        numbers.push(row);
    }

    Err("No operators found".to_string())
}

fn solve_part2(file: File) -> Result<u64, String> {
    let mut reader = BufReader::new(file);

    let op_reg = Regex::new(r"[\+\*]").unwrap();

    let mut buffer: String = String::new();
    match reader.read_line(&mut buffer) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }
    let mut strings: Vec<String> = buffer.strip_suffix("\r\n").unwrap().chars().map(|x| if x == ' ' {"".to_string()} else {x.to_string()}).collect();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };

        let mut sum_of_solutions: u64 = 0;

        if op_reg.is_match(line.as_str()) {
            let ops: Vec<&str> = op_reg.find_iter(line.as_str()).map(|x| x.as_str()).collect();
            let num_groups: Vec<Vec<_>> = strings.split(|x| x == "")
                .map(|x| x.iter().map(|y| y.parse::<u64>().unwrap()).collect()).collect();

            for index in 0..ops.len() {
                sum_of_solutions += match ops[index] {
                    "+" => num_groups[index].iter().sum::<u64>(),
                    _ => num_groups[index].iter().fold(1, |acc, x| acc * x)
                }
            }
            return Ok(sum_of_solutions);
        }

        let mut index = 0;
        for char in line.chars() {
            if char != ' ' {
                strings[index].push(char);
            }
            index += 1;
        }
    }


    Err("No operators found".to_string())
}