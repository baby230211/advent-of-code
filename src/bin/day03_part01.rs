use std::{collections::HashMap, vec};

fn main() {
    let contents = advent_of_code_2023::run().unwrap();
    let res = process(&contents).unwrap();
    println!("{}", res);
}

#[derive(Debug)]
enum Value {
    Symbol(char),
    Number(u32),
    Empty,
}
#[derive(Debug, Hash, Eq, PartialEq)]
struct Position(i32, i32);

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    value: Value,
}

impl Point {
    fn new(x: i32, y: i32, value: Value) -> Self {
        Self { x, y, value }
    }
    fn collect_number(vec_point: &Vec<&Point>) -> u32 {
        let num = vec_point
            .iter()
            .filter_map(|point| match point.value {
                Value::Number(n) => {
                    return Some(n.to_string());
                }
                _ => return None,
            })
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        return num;
    }
}

fn process(contents: &str) -> Result<u32, ()> {
    let positions = [
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    let engine_grid: Vec<Vec<Point>> = contents
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let x_line: Vec<Point> = line
                .chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '.' => Point::new(x as i32, y as i32, Value::Empty),
                    c if char.is_digit(10) => Point::new(
                        x as i32,
                        y as i32,
                        Value::Number(c.to_digit(10).unwrap().try_into().unwrap()),
                    ),
                    c => Point::new(x as i32, y as i32, Value::Symbol(c)),
                })
                .collect();
            return x_line;
        })
        .collect();

    let mut numbers: Vec<Vec<&Point>> = Vec::new();
    let mut symbols_map: HashMap<(i32, i32), &Point> = HashMap::new();
    // get all numbers
    for x_line in engine_grid.iter() {
        for point in x_line.iter() {
            if let Value::Number(_) = point.value {
                match numbers.iter().last() {
                    Some(last_vec_point_number) => {
                        let last_point = *last_vec_point_number.iter().last().expect("last point");
                        if last_point.x == point.x - 1 && last_point.y == point.y {
                            numbers.last_mut().unwrap().push(point);
                        } else {
                            numbers.push(vec![point]);
                        }
                    }
                    None => {
                        numbers.push(vec![point]);
                    }
                }
            }
            if let Value::Symbol(_) = point.value {
                symbols_map.insert((point.x, point.y), point);
            }
        }
    }
    let re: u32 = numbers
        .iter()
        .filter_map(|vec_point| {
            let is_valid = vec_point.iter().any(|point| {
                positions.iter().any(|(x_pos, y_pos)| {
                    let x = point.x + x_pos;
                    let y = point.y + y_pos;
                    if symbols_map.contains_key(&(x, y)) {
                        return true;
                    } else {
                        return false;
                    }
                })
            });
            if is_valid {
                let number = Point::collect_number(vec_point);
                return Some(number);
            }
            return None;
        })
        .sum();
    Ok(re)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361, process(contents).unwrap());
    }
}
