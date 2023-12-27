use std::{collections::HashMap, u128};

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

fn process(contents: &str) -> Result<u128, ()> {
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
    let mut iter = steps.chars().cycle();
    let mut current_step = maps
        .iter()
        .filter_map(|(k, _)| match k.ends_with("A") {
            true => Some(*k),
            false => None,
        })
        .collect::<Vec<_>>();
    let mut steps_vec: Vec<u32> = vec![0; current_step.len()];
    while let Some(next_iter) = iter.next() {
        step += 1;
        current_step.iter_mut().enumerate().for_each(|(k, x)| {
            let next_step = maps.get(x).unwrap();
            match next_iter {
                'L' => *x = &next_step.left,
                'R' => *x = &next_step.right,
                _ => panic!("invalid input"),
            }
            if x.ends_with("Z") {
                let orginial_step = steps_vec.get_mut(k).unwrap();
                *orginial_step = step;
            }
        });
        if steps_vec.iter().all(|x| *x != 0) {
            break;
        }
    }

    // find lcm of this vec
    let sum: u128 = lcm(&steps_vec.iter().map(|x| *x as usize).collect::<Vec<_>>())
        .try_into()
        .unwrap();
    println!("steps_vec: {:?}", steps_vec);
    println!("current_step: {:?}", sum);

    Ok(sum)
}
fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, process(contents).unwrap());
    }
}
