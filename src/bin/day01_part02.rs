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
                .char_indices()
                .filter_map(|(idx, letter)| letter.to_digit(10).or_else(|| word_to_digit(line, idx)))
                .collect();
            Some(digits.first()? * 10 + digits.last()?)
        })
        .sum();
    Ok(res.to_string())
}

fn word_to_digit(letter: &str, idx: usize) -> Option<u32> {
    let num_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digit = num_words
        .iter()
        .position(|w| letter[idx..].starts_with(w))?
        + 1;
    Some(digit as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!("281", process(contents).unwrap());
    }
}
