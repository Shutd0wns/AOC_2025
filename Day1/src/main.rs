use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use extend::ext;

fn main() {
    let (solution1, solution2) = solve_input("C:\\Users\\Radek\\RustroverProjects\\AOC_2025\\Day1\\src\\input.txt");
    //let (solution1, solution2) = solve_input("C:\\Users\\Radek\\RustroverProjects\\AOC_2025\\Day1\\src\\restInput.txt");
    println!("Solution 1 is {solution1}");
    println!("Solution 2 is {solution2}");
}

fn solve_input(file_path: &str) -> (i32, i32) {
    let file = File::open(file_path).expect("File not found or not readable");
    let reader = BufReader::new(file);

    let mut dial = 50;
    let mut on_zero = 0;
    let mut through_zero = 0;

    for line in reader.lines() {
        let read_line = line.unwrap();

        let re = Regex::new(r"([LR])(\d+)").unwrap();
        if !re.is_match(read_line.as_str()) {
            panic!("Invalid Format");
        }
        let caps = re.captures(read_line.as_str()).unwrap().extract().1;

        match caps {
            ["L", num] => {
                let full_turns = num.parse::<i32>().expect("Invalid number") / 100;
                let after_move = (dial - num.parse::<i32>().expect("Invalid number")).modulo(100);
                through_zero += (if dial < after_move && dial != 0 {1} else {0}) + if after_move == 0 {1} else {0} + full_turns;
                dial = after_move;
            },
            ["R", num] => {
                let full_turns = num.parse::<i32>().expect("Invalid number") / 100;
                let after_move = (dial + num.parse::<i32>().expect("Invalid number")).modulo(100);
                through_zero += (if dial > after_move {1} else {0}) + full_turns;
                dial = after_move;
            },
            _ => panic!("Invalid format")
        }

        if dial == 0 {
            on_zero += 1;
        }
    }

    (on_zero, through_zero)
}

#[ext]
impl i32 {
    fn modulo(self, modulo: i32) -> i32 {
        let mut res = self % modulo;
        if res < 0 { res += modulo }
        res
    }
}

