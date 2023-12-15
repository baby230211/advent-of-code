use std::error::Error;

fn main() {
    let contents = advent_of_code_2023::run().unwrap();
    let res = process(&contents).unwrap();
    println!("{}", res);
}
#[derive(Debug, Default)]
struct CubeSet {
    r: u32,
    g: u32,
    b: u32,
}
impl CubeSet {
    fn parse_round(s: &str) -> Result<CubeSet, Box<dyn Error>> {
        let mut set = CubeSet::default();
        for round in s.split(", ") {
            let (n, color) = round
                .split_once(" ")
                .expect("expect has a space on number and color");
            let n: u32 = n.parse()?;
            match color {
                "red" => set.r += n,
                "green" => set.g += n,
                "blue" => set.b += n,
                _ => return Err(format!("unexpected color name came up  {color}").into()),
            }
        }
        Ok(set)
    }
}

fn process(contents: &str) -> Result<u32, ()> {
    let res: u32 = contents
        .lines()
        .filter_map(|line| {
            let (_, sets_part) = line.split_once(": ").expect("expected line to have a : ");
            let sets = sets_part
                .split("; ")
                .map(CubeSet::parse_round)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            let cube_set = sets.iter().fold(CubeSet::default(), |mut acc, x| {
                if acc.b < x.b {
                    acc.b = x.b;
                }
                if acc.g < x.g {
                    acc.g = x.g
                }
                if acc.r < x.r {
                    acc.r = x.r
                }
                acc
            });
            Some(cube_set.b * cube_set.r * cube_set.g)
        })
        .sum();
    return Ok(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, process(contents).unwrap());
    }
}
