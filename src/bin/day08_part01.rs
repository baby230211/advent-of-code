use std::collections::HashMap;

fn main() {
    let contents = advent_of_code_2023::run().unwrap();
    let res = process(&contents).unwrap();
    println!("{}", res);
}

#[derive(Debug)]
struct NextStep {
    left: String,
    right: String,
}

fn process(contents: &str) -> Result<u32, ()> {
    let (steps, lookup_map) = contents.split_once("\n\n").unwrap();
    println!("steps: {}", steps);
    let maps = lookup_map.lines().fold(HashMap::new(), |mut acc, x| {
        let (key, value) = x.split_once(" = ").unwrap();
        let (left, right) = value.split_once(", ").unwrap();
        acc.insert(
            key,
            NextStep {
                left: left.replace("(", "").to_string(),
                right: right.replace(")", "").to_string(),
            },
        );
        acc
    });

    let mut step = 0;
    let mut current = "AAA";
    let mut iter = steps.chars().cycle();
    while let Some(next_iter) = iter.next() {
        step += 1;
        let next_step = maps.get(current).unwrap();
        match next_iter {
            'L' => current = &next_step.left,
            'R' => current = &next_step.right,
            _ => panic!("Invalid step"),
        }
        println!("current: {}", current);
        if current == "ZZZ" {
            break;
        }
    }

    Ok(step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, process(contents).unwrap());
    }
}
