fn main() {
    let contents = advent_of_code_2023::run().unwrap();
    let res = process(&contents).unwrap();
    println!("{}", res);
}
#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn split_to_list(s: &str) -> Vec<u32> {
        return s
            .trim()
            .split(" ")
            .filter_map(|s| match s {
                "" => None,
                _ => Some(s.trim().parse::<u32>().unwrap()),
            })
            .collect::<Vec<_>>();
    }
    fn get_pair(time_list: Vec<u64>, distance_list: Vec<u64>) -> Vec<Race> {
        time_list
            .iter()
            .zip(distance_list.iter())
            .map(|(time, distance)| Race {
                time: *time,
                distance: *distance,
            })
            .collect::<Vec<_>>()
    }
    fn split_to_number(s: &str) -> u64 {
        s.trim()
            .split(" ")
            .filter_map(|s| match s {
                "" => None,
                _ => Some(s.trim().to_string()),
            })
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    }
    fn get_number_of_ways(&self) -> u32 {
        let mut count = 0;
        let time = self.time;
        let distance = self.distance;
        for i in 1..=time {
            if distance < (time - i) * i {
                count += 1;
            }
        }
        count
    }
}

fn process(contents: &str) -> Result<u32, ()> {
    let lines = contents.lines().collect::<Vec<_>>();
    println!("{:?}", lines);
    let list = lines
        .iter()
        .zip(lines.iter().skip(1))
        .flat_map(|(times, distance)| {
            let (_, times) = times.split_once(":").unwrap();
            let (_, distance) = distance.split_once(":").unwrap();
            println!("{:?} {:?}", times, distance);

            let time_list = vec![Race::split_to_number(times)];
            println!("{:?}", time_list);
            let distance_list = vec![Race::split_to_number(distance)];
            println!("{:?}", distance_list);
            Race::get_pair(time_list, distance_list)
        })
        .collect::<Vec<_>>();
    println!("{:?}", list);
    let mut sum = 1;
    for race in list {
        let ways = race.get_number_of_ways();
        if ways > 0 {
            sum *= ways;
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(71503, process(contents).unwrap());
    }
}
