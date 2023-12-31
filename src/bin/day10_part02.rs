use std::collections::HashMap;

fn main() {
    let contents = advent_of_code_2023::run().unwrap();
    let res = process(&contents).unwrap();
    println!("{}", res);
}
#[derive(Debug, Clone)]
enum Direction {
    Left,
}
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    value: String,
    visited: bool,
}
impl Point {
    fn walk(from: &Point, current: &Point, grid: &HashMap<(i32, i32), Point>) -> Point {
        let is_same_x = from.x == current.x;
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
            visited: false,
        }
    }
    fn get_steps(
        &self,
        steps: (i32, i32),
        grid: &HashMap<(i32, i32), Point>,
    ) -> Option<(i32, Vec<Point>)> {
        let mut queue = vec![];
        let mut from = self.clone();
        if let Some(pipe) = grid.get(&(from.x + steps.0, from.y + steps.1)) {
            if pipe.value == "." {
                return None;
            }
        } else {
            return None;
        }
        let mut next = Point {
            x: from.x + steps.0,
            y: from.y + steps.1,
            value: grid
                .get(&(from.x + steps.0, from.y + steps.1))
                .unwrap()
                .value
                .clone(),
            visited: false,
        };
        let mut step = 1;
        while next.value != self.value {
            step += 1;
            let temp = next.clone();
            queue.push(next.clone());
            next = Point::walk(&from, &next, &grid);
            from = temp;
        }
        Some((step / 2, queue))
    }

    fn check_intersection(
        &self,
        check_point: &(i32, i32),
        grid: &HashMap<(i32, i32), Point>,
    ) -> bool {
        let current_pipe = grid.get(&(self.x, self.y)).unwrap();
        if current_pipe.visited {
            return false;
        }
        if let Some(check_pipe) = grid.get(&check_point) {
            let value = check_pipe.value.as_str();
            let is_visited = check_pipe.visited;
            if !is_visited {
                return false;
            }
            if value == "|" || value == "L" || value == "J" {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    fn check_direction(
        &self,
        direction: &Direction,
        grid: &HashMap<(i32, i32), Point>,
    ) -> bool {
        let current_pipe = grid.get(&(self.x, self.y)).unwrap();
        let mut cross = 0;
        let mut cross_point = vec![];
        match direction {
            Direction::Left => {
                let mut current_x = self.x - 1;
                while current_x >= 0 {
                    if current_pipe.check_intersection(&(current_x, self.y), &grid) {
                        cross_point.push((current_x, self.y));
                        cross += 1;
                    }
                    current_x -= 1;
                }
            }
        };
        if self.x == 12 && self.y == 4 {
            println!("cross_point {:?}", cross_point);
            println!("cross {}", cross);
        }
        if cross % 2 == 0 {
            return false;
        }
        true
    }
}

fn process(contents: &str) -> Result<i32, ()> {
    // intersection of polygons
    // 1. get the path points and store them in a hashmap
    // 2. run through the grid and check the intersection of the polygons

    let start_pipe = "S";
    let mut start_point = (0, 0);
    let mut large_grid: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut grid: HashMap<(i32, i32), Point> =
        contents
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (y, line)| {
                let mut x_vec: Vec<(i32, i32)> = Vec::new();
                line.chars().enumerate().for_each(|(x, c)| {
                    let point = Point {
                        x: x as i32,
                        y: y as i32,
                        value: c.to_string(),
                        visited: false,
                    };
                    if c.to_string() == start_pipe.to_string() {
                        start_point = (x as i32, y as i32);
                    }
                    x_vec.push((x as i32, y as i32));

                    acc.insert((x as i32, y as i32), point);
                });
                large_grid.push(x_vec);
                acc
            });
    let from = grid.get(&start_point).unwrap().clone();
    let positions = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let step = positions
        .iter()
        .filter_map(|pos| from.get_steps(*pos, &grid))
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap();
    step.1.iter().for_each(|p| {
        let point: &mut Point = grid.get_mut(&(p.x, p.y)).unwrap();
        point.visited = true;
    });
    large_grid.iter().for_each(|p| {
        p.iter().for_each(|p| {
            if grid.get(&p).unwrap().visited {
                print!("X");
            } else {
                print!("{}", grid.get(&p).unwrap().value);
            }
        });
        println!("");
    });

    let res = large_grid.iter().fold(0, |acc, p| {
        acc + p.iter().fold(0, |acc, p| {
            let point = grid.get(&p).unwrap();
            if point.value == "S" {
                return acc;
            }
            let is_cross = point.check_direction(&Direction::Left,  &grid);

            if is_cross {
                println!("cross point {:?} {:?}", p, point);
                acc + 1
            } else {
                acc
            }
        })
    });
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(8, process(contents).unwrap());
    }
    #[test]
    fn test_process_2() {
        let contents = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(4, process(contents).unwrap());
    }

    #[test]
    fn test_process_3() {
        let contents = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(10, process(contents).unwrap());
    }
}
