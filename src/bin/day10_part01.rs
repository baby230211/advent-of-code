use std::collections::HashMap;

fn main() {
    let contents = advent_of_code_2023::run().unwrap();
    let res = process(&contents).unwrap();
    println!("{}", res);
}
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    value: String,
}
impl Point {
    fn walk(from: &Point, current: &Point, grid: &HashMap<(i32, i32), Point>) -> Point {
        let is_same_x = from.x == current.x;
        println!("from: {:?}, current: {:?}", from, current);
        let mut point = (current.x, current.y);
        if is_same_x {
            let is_from_top = from.y < current.y;
            let pipe = grid.get(&(current.x, current.y)).unwrap();
            match pipe.value.as_str() {
                "L" => {
                    point.0 = current.x + 1;
                }
                "J" => {
                    point.0 = current.x - 1;
                }
                "7" => {
                    point.0 = current.x - 1;
                }
                "F" => {
                    point.0 = current.x + 1;
                }
                "|" => {
                    point.1 = if is_from_top {
                        current.y + 1
                    } else {
                        current.y - 1
                    };
                }
                _ => {
                    point.0 = from.x;
                    point.1 = from.y;
                    println!("invalid pipe");
                }
            };
        } else {
            let is_from_left = from.x < current.x;
            let pipe = grid.get(&(current.x, current.y)).unwrap();
            match pipe.value.as_str() {
                "L" => {
                    point.1 = current.y - 1;
                }
                "J" => {
                    point.1 = current.y - 1;
                }
                "7" => {
                    point.1 = current.y + 1;
                }
                "F" => {
                    point.1 = current.y + 1;
                }
                "-" => {
                    point.0 = if is_from_left {
                        current.x + 1
                    } else {
                        current.x - 1
                    };
                }
                _ => {
                    point.0 = from.x;
                    point.1 = from.y;
                    println!("invalid pipe");
                }
            };
        };

        Point {
            x: point.0,
            y: point.1,
            value: grid.get(&(point.0, point.1)).unwrap().value.clone(),
        }
    }
    fn get_steps(&self, steps: (i32, i32), grid: &HashMap<(i32, i32), Point>) -> i32 {
        let mut from = self.clone();
        if let Some(pipe) = grid.get(&(from.x + steps.0, from.y + steps.1)) {
            if pipe.value == "." {
                return 0;
            }
        } else {
            return 0;
        }
        let mut next = Point {
            x: from.x + steps.0,
            y: from.y + steps.1,
            value: grid
                .get(&(from.x + steps.0, from.y + steps.1))
                .unwrap()
                .value
                .clone(),
        };
        let mut step = 1;
        while next.value != self.value {
            step += 1;
            let temp = next.clone();
            next = Point::walk(&from, &next, &grid);
            from = temp;
            println!("{:?}", next);
        }
        step
    }
}

fn process(contents: &str) -> Result<i32, ()> {
    let ground = ".";
    let start_pipe = "S";
    let pipes = vec!["|", "-", "L", "J", "7", "F", "S", "."];
    let mut start_point = (0, 0);
    let grid: HashMap<(i32, i32), Point> =
        contents
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (y, line)| {
                line.chars().enumerate().for_each(|(x, c)| {
                    let point = Point {
                        x: x as i32,
                        y: y as i32,
                        value: c.to_string(),
                    };
                    if c.to_string() == start_pipe.to_string() {
                        start_point = (x as i32, y as i32);
                    }
                    acc.insert((x as i32, y as i32), point);
                });
                acc
            });
    let mut steps_vec: Vec<i32> = vec![];
    let from = grid.get(&start_point).unwrap().clone();
    let top = from.get_steps((0, -1), &grid);
    let left = from.get_steps((-1, 0), &grid);
    let bottom = from.get_steps((0, 1), &grid);
    let right = from.get_steps((1, 0), &grid);
    steps_vec.push(top);
    steps_vec.push(left);
    steps_vec.push(bottom);
    steps_vec.push(right);
    let res = steps_vec.iter().max().unwrap();
    let res = *res / 2;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(4, process(contents).unwrap());
        let contents = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(8, process(contents).unwrap());
    }
}
