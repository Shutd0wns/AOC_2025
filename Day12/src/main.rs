use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let solution = match solve_puzzle("src/input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Solver failed with error {e}")
    };

    println!("Solution is {solution}");
}

fn solve_puzzle(file_path: &str) -> Result<u64, String> {
    let file_result = File::open(file_path);
    let file = match file_result {
        Ok(value) => value,
        Err(e) => return Err(e.to_string())
    };

    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines();
    for _ in 0..30 {
        lines_iter.next();
    }

    let mut correct: u64 = 0;

    for line_result in lines_iter {
        let line = match line_result {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };

        let (area, counts) = match line.split(':').collect::<Vec<_>>()[..] {
            [area, counts] => (area, counts),
            _ => return Err("Invalid Format".to_string()),
        };

        let fit_area: u64 = area.split('x').map(|x| x.parse::<u64>().unwrap()).fold(1, |acc, elem| acc * elem);
        let shapes_area: u64 = counts.trim_start().split(' ').map(|x| x.parse::<u64>().unwrap()).sum::<u64>() * 9;

        if fit_area >= shapes_area {
            correct += 1;
        }
    }

    Ok(correct)
}