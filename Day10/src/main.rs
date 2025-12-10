use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Sum;
use std::ops::Mul;
use regex::Regex;
use crate::bfs::{BFS};

use good_lp::{variables, variable, SolverModel, Solution, Expression, microlp, constraint};
mod bfs;

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
    let mut shortest_sum = 0_u64;
    let mut shortest_sum_count = 0_u64;

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };

        let re_indicator = Regex::new(r"\[(.+)\]").unwrap();
        let re_buttons = Regex::new(r"\(([^\)]+)\)").unwrap();
        let re_count = Regex::new(r"\{(.*)\}").unwrap();

        let indicator = match re_indicator.captures(line.as_str()) {
            Some(val) => val[1].to_string(),
            None => return Err("Invalid Format".to_string())
        };

        let buttons_result: Result<Vec<Vec<u64>>, String> = re_buttons.captures_iter(line.as_str()).map(|captures| {
            match captures[1].split(',').map(|x| x.parse::<u64>()).collect() {
                Ok(array) => Ok(array),
                Err(_) => Err("Invalid Format".to_string())
            }
        }).collect();

        let buttons = match buttons_result {
            Ok(val) => val,
            Err(e) => return Err(e)
        };

        let counts: Vec<u64> = match re_count.captures(line.as_str()) {
            Some(val) => match val[1].split(',').map(|x| x.parse::<u64>()).collect() {
                Ok(array) => array,
                Err(_) => return Err("Invalid Format".to_string())
            } ,
            None => return Err("Invalid Format".to_string())
        };

        let bfs = BFS::new((0..indicator.len()).map(|_| '.').collect(), indicator, buttons.clone());

        shortest_sum += match bfs.find_shortest() {
            Ok(val) => val,
            Err(e) => return Err(e)
        };

        // Part 2

        let mut vars = variables! {};

        for _ in &buttons {
            vars.add(variable().min(0).integer());
        }
        let var_clone = vars.clone();
        let expression = Expression::sum(var_clone.iter_variables_with_def().map(|x| x.0).into_iter());

        let mut builder = vars.minimise(expression)
            .using(microlp);

        for index in 0..counts.len() {
            let coefficients: Vec<i32> = buttons.iter().map(|x| if x.contains(&(index as u64)) {1_i32} else {0_i32}).collect();
            let mut members: Vec<Expression> = Vec::new();

            let mut iter = var_clone.iter_variables_with_def().map(|x| x.0);

            for member_index in 0..buttons.len() {
                let member = Expression::from(iter.next().unwrap()).mul(coefficients[member_index]);
                members.push(member);
            }
            builder = builder.with(constraint!(Expression::sum(members.iter()) == counts[index] as i32))
        }

        let solution = builder.solve().unwrap();

        let mut machine_sum = 0_u64;

        for var in var_clone.iter_variables_with_def().map(|x| x.0) {
            machine_sum += solution.value(var).round() as u64
        }
        shortest_sum_count += machine_sum;
    }

    Ok((shortest_sum,shortest_sum_count))
}