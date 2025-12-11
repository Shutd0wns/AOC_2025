use std::collections::HashMap;
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

    let mut out_map: HashMap<String, Vec<String>> = HashMap::new();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };

        let key = line.chars().take(3).collect();
        let outputs: String = line.chars().skip(5).collect();
        let output_labels: Vec<String> = outputs.split(' ').map(|x| String::from(x)).collect();
        out_map.insert(key, output_labels);
    }

    let you_path_count = get_num_paths("you".to_string(), &out_map, vec![]);
    let svr_path_without_count = get_num_paths("svr".to_string(), &out_map, vec!["dac".to_string()])
        + get_num_paths("svr".to_string(), &out_map, vec!["fft".to_string()])
        - get_num_paths("svr".to_string(), &out_map, vec!["fft".to_string(), "dac".to_string()]);
    let svr_path_count =  get_num_paths("svr".to_string(), &out_map, vec![]) - svr_path_without_count;

    Ok((you_path_count,svr_path_count))
}

fn get_num_paths(from: String, out_map: &HashMap<String, Vec<String>>, without: Vec<String>) -> u64 {
    let mut path_count: HashMap<String, u64> = HashMap::new();
    path_count.insert("out".to_string(), 1);
    for str in without {
        path_count.insert(str, 0);
    }
    let mut resolved = true;
    while resolved {
        resolved = false;
        for (key, value) in out_map {
            if !path_count.contains_key(key) && value.iter().all(|x| path_count.contains_key(x)) {
                resolved = true;
                path_count.insert(key.clone(), value.iter().map(|x| path_count[x]).sum());
            }
        }
    }

    path_count[&from]
}
