use itertools::Itertools;
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

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    value: Value,
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
    let res: Vec<Vec<Point>> = contents
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let x_line: Vec<Point> = line
                .chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '.' => Point {
                        x: x as i32,
                        y: y as i32,
                        value: Value::Empty,
                    },

                    c if char.is_digit(10) => Point {
                        x: x as i32,
                        y: y as i32,
                        value: Value::Number(c.to_digit(10).unwrap().try_into().unwrap()),
                    },
                    c => Point {
                        x: x as i32,
                        y: y as i32,
                        value: Value::Symbol(c),
                    },
                })
                .collect();
            return x_line;
        })
        .collect();

    let mut numbers: Vec<Vec<&Point>> = Vec::new();
    let mut symbols_map: HashMap<(i32, i32), &Point> = HashMap::new();
    // get all numbers
    for x_line in res.iter() {
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
    let mut total = 0;
    for symbol in symbols_map
        .iter()
        .filter(|(_, value)| matches!(value.value, Value::Symbol('*')))
    {
        println!("{:?}", symbol);
        let mut indexes_of_numbers: Vec<usize> = Vec::new();
        for (idx, points) in numbers.iter().enumerate() {
            for point in points.iter() {
                let pos_to_check = positions
                    .iter()
                    .map(|(x_pos, y_pos)| {
                        let x = symbol.1.x + x_pos;
                        let y = symbol.1.y + y_pos;
                        (x, y)
                    })
                    .collect::<Vec<(i32, i32)>>();
                pos_to_check.iter().any(|(x, y)| {
                    if point.x == *x && point.y == *y {
                        indexes_of_numbers.push(idx);
                        return true;
                    }
                    return false;
                });
            }
        }
        println!("{:?}", indexes_of_numbers);
        if indexes_of_numbers.iter().unique().count() != 2 {
            continue;
        }

        let gear_ratio = indexes_of_numbers
            .iter()
            .unique()
            .map(|idx| {
                let num: u32 = numbers[*idx]
                    .iter()
                    .map(|point| match point.value {
                        Value::Number(n) => n.to_string(),
                        _ => panic!("not a number"),
                    })
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                return num;
            })
            .product::<u32>();
        total += gear_ratio;
    }
    Ok(total)
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
        assert_eq!(467835, process(contents).unwrap());
    }
}
