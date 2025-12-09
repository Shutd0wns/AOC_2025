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

    let mut largest_area = 0;
    let mut points: Vec<(u64, u64)> = Vec::new();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(val) => val,
            Err(e) => return Err(e.to_string())
        };

        let split_values = line.split(',');
        let cords: (u64, u64) = match split_values.map(|x| x.parse::<u64>()).collect::<Result<Vec<_>, _>>() {
            Ok(val) => match val[..] {
                [x, y] => (x, y),
                _ => return Err("Invalid Format".to_string())
            },
            Err(_) => return Err("Not a number".to_string())
        };

        for point in &points {
            let area = (point.0.abs_diff(cords.0) + 1) * (point.1.abs_diff(cords.1) + 1);
            if largest_area < area {
                largest_area = area;
            }
        }
        points.push(cords);
    }

    let mut edges = Vec::new();

    for i in 0..points.len() {
        edges.push((points[i], points[(i+1) % points.len()]))
    }

    let mut largest_red_green_area = 0;

    for i in 0..points.len()  {
        for j in i+1..points.len() {
            let rect_edges: Vec<_>;
            if points[i].0 == points[j].0 {
                let x = points[i].0;
                let y1;
                let y2;
                if points[i].1.abs_diff(points[j].1) == 1 {
                    if largest_red_green_area < 2 {
                        largest_red_green_area = 2;
                    }
                    continue
                }
                else if points[i].1 < points[j].1 {
                    y1 = points[i].1;
                    y2 = points[j].1;
                }
                else {
                    y1 = points[j].1;
                    y2 = points[i].1;
                }
                rect_edges = vec![((x, y1), (x, y2))]
            }
            else if points[i].1 == points[j].1 {
                let y = points[i].1;
                let x1;
                let x2;
                if points[i].0.abs_diff(points[j].0) == 1 {
                    if largest_red_green_area < 2 {
                        largest_red_green_area = 2;
                    }
                    continue
                }
                else if points[i].0 < points[j].0 {
                    x1 = points[i].0;
                    x2 = points[j].0;
                }
                else {
                    x1 = points[j].0;
                    x2 = points[i].0;
                }
                rect_edges = vec![((x1, y), (x2, y))]
            }
            else {
                if points[i].0 > points[j].0 {
                    if points[i].1 > points[j].1 {
                        rect_edges = vec![((points[i].0 - 1, points[i].1 - 1), (points[j].0 + 1, points[i].1 - 1)),
                                              ((points[j].0 + 1, points[j].1 + 1), (points[j].0 + 1, points[i].1 - 1)),
                                              ((points[j].0 + 1, points[j].1 + 1), (points[i].0 - 1, points[j].1 + 1)),
                                              ((points[i].0 - 1, points[i].1 - 1), (points[i].0 - 1, points[j].1 + 1))];
                    }
                    else {
                        rect_edges = vec![((points[i].0 - 1, points[i].1 + 1), (points[j].0 + 1, points[i].1 + 1)),
                                              ((points[j].0 + 1, points[j].1 - 1), (points[j].0 + 1, points[i].1 + 1)),
                                              ((points[j].0 + 1, points[j].1 - 1), (points[i].0 - 1, points[j].1 - 1)),
                                              ((points[i].0 - 1, points[i].1 + 1), (points[i].0 - 1, points[j].1 - 1))];
                    }
                }
                else {
                    if points[i].1 > points[j].1 {
                        rect_edges = vec![((points[i].0 + 1, points[i].1 - 1), (points[j].0 - 1, points[i].1 - 1)),
                                              ((points[j].0 - 1, points[j].1 + 1), (points[j].0 - 1, points[i].1 - 1)),
                                              ((points[j].0 - 1, points[j].1 + 1), (points[i].0 + 1, points[j].1 + 1)),
                                              ((points[i].0 + 1, points[i].1 - 1), (points[i].0 + 1, points[j].1 + 1))];
                    }
                    else {
                        rect_edges = vec![((points[i].0 + 1, points[i].1 + 1), (points[j].0 - 1, points[i].1 + 1)),
                                              ((points[j].0 - 1, points[j].1 - 1), (points[j].0 - 1, points[i].1 + 1)),
                                              ((points[j].0 - 1, points[j].1 - 1), (points[i].0 + 1, points[j].1 - 1)),
                                              ((points[i].0 + 1, points[i].1 + 1), (points[i].0 + 1, points[j].1 - 1))];
                    }
                }
            }

            let mut inside = true;
            for rect_edge in &rect_edges {
                inside = is_inside(rect_edge, &edges);
                if !inside {
                    break
                }
            }

            if inside {
                let area = (points[i].0.abs_diff(points[j].0) + 1) * (points[i].1.abs_diff(points[j].1) + 1);
                if largest_red_green_area < area {
                    largest_red_green_area = area;
                }
            }
        }
    }

    Ok((largest_area,largest_red_green_area))
}

fn is_inside(line1: &((u64, u64), (u64, u64)), edges: &Vec<((u64, u64), (u64, u64))>) -> bool {
    match line1 {
        &((x1,y1), (x2, y2)) if x1 == x2 => {
            !edges.iter().any(|&point| {
                point.0.1 == point.1.1 &&
                ((point.0.0 <= x1 && x1 <= point.1.0) || (point.1.0 <= x1 && x1 <= point.0.0)) &&
                ((y1 <= point.0.1 && point.0.1 <= y2) || (y2 <= point.0.1 && point.0.1 <= y1))
            })
        },
        &((x1,y1), (x2, y2)) if y1 == y2 => {
            !edges.iter().any(|&point| {
                point.0.0 == point.1.0 &&
                ((point.0.1 <= y1 && y1 <= point.1.1) || (point.1.1 <= y1 && y1 <= point.0.1)) &&
                ((x1 <= point.0.0 && point.0.0 <= x2) || (x2 <= point.0.0 && point.0.0 <= x1))
            })
        },
        _ => false
    }
}