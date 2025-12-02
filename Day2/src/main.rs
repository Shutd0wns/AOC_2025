use std::fs;
use fancy_regex::Regex;

fn main() {
    let (solution1, solution2) = solve_puzzle("src/input.txt");
    println!("Solution to part one: {solution1}");
    println!("Solution to part two: {solution2}");
}

fn solve_puzzle(file_path: &str) -> (i64, i64) {
    let file_content = fs::read_to_string(file_path).unwrap();

    let ranges = file_content.split(',');

    let re = Regex::new(r"^(\d*)\1$").unwrap();
    let re2 = Regex::new(r"^(\d*)\1+$").unwrap();
    let mut invalid_sum = 0;
    let mut invalid_sum2 = 0;

    for range in ranges {
        let range_values: Vec<i64> = range.split('-').map(|x| x.parse::<i64>().expect("Invalid Format")).collect();
        for num in range_values[0]..range_values[1] {
            if re.is_match(num.to_string().as_str()).unwrap() {
                invalid_sum += num;
            }
            if re2.is_match(num.to_string().as_str()).unwrap() {
                invalid_sum2 += num;
            }
        }
    }

    (invalid_sum, invalid_sum2)
}