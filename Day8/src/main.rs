use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;
use crate::disjoint_set::DisjointSets;

mod disjoint_set;

fn main() {
    let (solution1, solution2) = match solve_puzzle("src/input.txt", 1000) {
        Ok(val) => val,
        Err(e) => panic!("Solver failed with error {e}")
    };

    println!("Solution to part one: {solution1}");
    println!("Solution to part two: {solution2}");
}

fn solve_puzzle(file_path: &str, num_of_closest: usize) -> Result<(usize,usize), String> {
    let file_result = File::open(file_path);
    let file = match file_result {
        Ok(value) => value,
        Err(e) => return Err(e.to_string())
    };

    let reader = BufReader::new(file);

    let mut points: Vec<(usize, usize, usize)> = Vec::new();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };

        let split_values: Result<Vec<_>, String> = line.split(',').map(|x| {
            match x.parse::<usize>() {
                Ok(val) => Ok(val),
                Err(e) => Err(e.to_string())
            }
        }).collect();

        match split_values {
            Ok(val) => {
                match val[..] {
                    [x, y, z] => points.push((x,y,z)),
                    _ => return Err("Expected 3D point".to_string())
                }
            }
            Err(e) => return Err(e.to_string())
        }
    }

    let mut connections: Vec<(usize, usize)> = Vec::new();

    for point_a in 0..points.len() {
        for point_b in point_a + 1..points.len() {
            connections.push((point_a, point_b))
        }
    }

    connections.sort_by(|a , b| {
        calculate_distance(a, &points).cmp(&calculate_distance(b, &points))
    });

    let mut disjoint_sets = DisjointSets::new(points.len());

    for index in 0..num_of_closest {
        let connection = connections[index];
        disjoint_sets.union(connection.0, connection.1);
    }

    let mut index: usize = num_of_closest;
    while index < connections.len() {
        let connection = connections[index];
        disjoint_sets.union(connection.0, connection.1);
        if disjoint_sets.get_number_of_sets() == 1 {
            let coordinate_multiple = points[connection.0].0 * points[connection.1].0;
            return Ok((disjoint_sets.get_sizes().iter().take(3).fold(1_usize, |acc, &elem| acc * elem), coordinate_multiple))
        }
        index += 1;
    }

    Err("Cant create single circuit".to_string())
}

fn calculate_distance(connection: &(usize, usize), points: &Vec<(usize, usize, usize)>) -> usize {
    ((points[connection.0].0 as i64 - points[connection.1].0 as i64).pow(2) as usize
        + (points[connection.0].1 as i64 - points[connection.1].1 as i64).pow(2) as usize
        + (points[connection.0].2 as i64 - points[connection.1].2 as i64).pow(2) as usize ).isqrt()
}