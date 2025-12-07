use std::collections::{BTreeMap, BTreeSet};
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

fn solve_puzzle(file_path: &str) -> Result<(u64,u64), String> {
    let file_result = File::open(file_path);
    let file = match file_result {
        Ok(value) => value,
        Err(e) => return Err(e.to_string())
    };

    let reader = BufReader::new(file);
    let mut line_iter = reader.lines();

    let mut beams: BTreeMap<u64, u64> = BTreeMap::new();
    let mut splits = 0_u64;

    let first_line = match line_iter.next() {
        Some(val) => match val {
            Ok(value) => value,
            Err(e) => return Err(e.to_string())
        },
        None => return Err("Input file has no lines".to_string())
    };

    match first_line.find("S") {
        Some(val) => _ = beams.insert(val as u64, 1),
        None => return Err("No starting point".to_string())
    }

    for line_result in line_iter {
        let line = match line_result {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };

        let mut index: u64 = 0;
        for char in line.chars() {
            if !beams.contains_key(&index) {
                index += 1;
                continue
            }
            match char {
                '^' => {
                    let prev = beams.remove(&index).unwrap();
                    *beams.entry(index - 1).or_insert(0) += prev;
                    *beams.entry(index + 1).or_insert(0) += prev;
                    splits += 1;
                }
                _ => {}
            }
            index += 1;
        }
    }

    Ok((splits,beams.iter().fold(0, |acc, elem| acc + elem.1)))
}
