use std::collections::{HashMap, HashSet};

fn main() {
    let contents = advent_of_code_2023::run().unwrap();
    let res = process(&contents).unwrap();
    println!("{}", res);
}
#[derive(Debug)]
struct Card {
    instance_count: u32,
}

fn process(contents: &str) -> Result<u32, ()> {
    let mut card_map: HashMap<u32, Card> = HashMap::new();
    contents.lines().for_each(|line| {
        let (card_string, number_lists) = line.split_once(":").unwrap();
        let (_, card_id) = card_string.split_once(" ").unwrap();
        let (list_one, list_two) = number_lists.split_once("|").unwrap();
        let number_list_one = list_one
            .trim()
            .split(" ")
            .filter_map(|s| match s {
                "" => None,
                _ => Some(s.trim().parse::<u32>().unwrap()),
            })
            .collect::<HashSet<_>>();
        let number_list_two = list_two
            .trim()
            .split(" ")
            .filter_map(|s| match s {
                "" => None,
                _ => Some(s.trim().parse::<u32>().unwrap()),
            })
            .collect::<HashSet<_>>();
        let intersection_count: u32 = number_list_one
            .intersection(&number_list_two)
            .count()
            .try_into()
            .unwrap();
        println!("card_id: {}", card_id);
        let card_number_id = card_id.trim().parse::<u32>().unwrap();
        println!("{:?}", card_map);
        let card_instance_count = card_map.get_mut(&(card_number_id));
        match card_instance_count {
            Some(card) => {
                card.instance_count += 1;
            }
            None => {
                card_map.insert(card_number_id, Card { instance_count: 1 });
            }
        };
        let current_card_count = card_map.get(&card_number_id).unwrap().instance_count;
        for number in 1..intersection_count + 1 {
            if let Some(card) = card_map.get_mut(&(card_number_id + number)) {
                card.instance_count += 1 * current_card_count;
            } else {
                println!("inserting card: {}", card_number_id + number);
                card_map.insert(
                    card_number_id + number,
                    Card {
                        instance_count: 1 * current_card_count,
                    },
                );
            }
        }
    });

    let all_instances_count = card_map
        .values()
        .map(|card| card.instance_count)
        .sum::<u32>();
    Ok(all_instances_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, process(contents).unwrap());
    }
}
