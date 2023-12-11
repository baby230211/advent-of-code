fn main() {
    let contents = advent_of_code_2023::run().unwrap();
    let res = process(&contents).unwrap();
    println!("{}", res);
}

fn process(contents: &str) -> Result<String, ()> {
    let res: u32 = contents
        .lines()
        .filter_map(|line| {
            let digits: Vec<u32> = line
                .chars()
                .filter_map(|letter| letter.to_digit(10))
                .collect();
            Some(digits.first()? * 10 + digits.last()?)
        })
        .sum();
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(contents).unwrap());
    }
}
