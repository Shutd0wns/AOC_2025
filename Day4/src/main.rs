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

fn solve_puzzle(file_path: &str) -> Result<(u32, u32), String> {
    let file_result = File::open(file_path);
    let file = match file_result {
        Ok(value) => value,
        Err(e) => return Err(e.to_string())
    };

    let reader = BufReader::new(file);

    let mut roll_map: Vec<Vec<char>> = Vec::new();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };
        roll_map.push(line.chars().collect())
    }

    let mut rolls_accessible = 0;
    let mut rolls_accessible_recursive = 0;

    let mut rolls_to_remove: Vec<(usize, usize)> = Vec::new();
    while rolls_to_remove.len() > 0 || rolls_accessible == 0 {
        for &(x, y) in rolls_to_remove.iter() {
            roll_map[y][x] = '.';
        }
        rolls_to_remove.clear();

        for y in 0..roll_map.len() {
            for x in 0..roll_map.len() {
                if roll_map[y][x] == '@' && neighbours((x,y), roll_map.len() - 1).iter()
                    .filter(|cord| roll_map[cord.1][cord.0] == '@')
                    .count() < 4 {
                    rolls_accessible_recursive += 1;
                    rolls_to_remove.push((x,y))
                }
            }
        }

        if rolls_accessible == 0 {
            rolls_accessible = rolls_accessible_recursive;
        }
    }
    
    Ok((rolls_accessible,rolls_accessible_recursive))
}

fn neighbours((x, y): (usize, usize), length: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    if x > 0 {
        neighbours.push((x - 1,y));
        if y > 0 {
            neighbours.push((x - 1, y - 1));
        }
        if y < length {
            neighbours.push((x - 1, y + 1));
        }
    }
    if x < length {
        neighbours.push((x + 1, y));

        if y > 0 {
            neighbours.push((x + 1, y - 1));
        }
        if y < length {
            neighbours.push((x + 1, y + 1));
        }
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if y < length {
        neighbours.push((x, y + 1));
    }
    neighbours
}
