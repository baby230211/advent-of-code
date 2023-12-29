fn main() {
    let contents = advent_of_code_2023::run().unwrap();
    let res = process(&contents).unwrap();
    println!("{}", res);
}
#[derive(Debug)]
struct History {
    list: Vec<i64>,
}
impl History {
    fn new(list: Vec<i64>) -> Self {
        Self { list }
    }
    fn generate_sequence(l: &Vec<i64>) -> Vec<i64> {
        let diff_sequence =
            l.iter()
                .zip(l.iter().skip(1))
                .fold(Vec::new(), |mut acc, (first, last)| {
                    let offset = *last - *first;
                    acc.push(offset);
                    acc
                });
        diff_sequence
    }
    fn generate_extrapolate_sequence(&self) -> Vec<Vec<i64>> {
        let list = Self::generate_sequence(&self.list);
        let mut res: Vec<Vec<i64>> = vec![self.list.clone(), list];
        while res.last().unwrap().iter().any(|x| *x != 0) {
            let last = res
                .last()
                .expect("generate extrapolate sequence last error");
            let new_list = Self::generate_sequence(last);
            res.push(new_list);
        }
        println!("{:?}", res);
        res
    }
    fn get_next_value(extrapolate_sequence: &Vec<Vec<i64>>) -> i64 {
        let mut res = 0;
        let mut iter = extrapolate_sequence.iter();
        while let Some(list) = iter.next() {
            let last = list.last().expect("get next value last error");
            res += *last;
        }
        res
    }
    fn get_pre_value(extrapolate_sequence: &Vec<Vec<i64>>) -> i64 {
        let mut pre_sequence: Vec<i64> = Vec::new();
        let mut iter = extrapolate_sequence.iter().rev();
        while let Some(list) = iter.next() {
            let first = list.first().expect("get next value first error");
            if let Some(last) = pre_sequence.last() {
                let v = *first - *last;
                pre_sequence.push(v);
            } else {
                pre_sequence.push(*first);
            }
        }
        println!("pre_sequence: {:?}", pre_sequence);
        *pre_sequence.last().unwrap()
    }
}

fn process(contents: &str) -> Result<i64, ()> {
    let res = contents
        .lines()
        .map(|line| {
            let list = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().expect("parse error"))
                .collect::<Vec<_>>();
            let history = History::new(list);

            let extrapolate_sequence = history.generate_extrapolate_sequence();
            let value = History::get_pre_value(&extrapolate_sequence);
            value
        })
        .sum();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2, process(contents).unwrap());
    }
}
