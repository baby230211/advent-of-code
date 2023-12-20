use std::ops::Not;

fn main() {
    let contents = advent_of_code_2023::run().unwrap();
    let res = process(&contents).unwrap();
    println!("{}", res);
}

#[derive(Debug)]
struct Seed {
    value: u64,
}
#[derive(Debug)]
struct Layer {
    dest: u64,
    src: u64,
    range: u64,
}
fn process(contents: &str) -> Result<u64, ()> {
    let res = contents
        .split_once("\n\n")
        .and_then(|(seeds, rest)| {
            let (_, seeds) = seeds.split_once(": ")?;
            let seeds_vec = seeds
                .split(" ")
                .map(|s| Seed {
                    value: s.parse::<u64>().unwrap(),
                })
                .collect::<Vec<_>>();
            let mut i = rest.split("\n");

            let mut layers = vec![];
            let mut resources = Vec::new();
            while let Some(line) = i.next() {
                if line.is_empty() | line.contains("map:") {
                    if layers.is_empty().not() {
                        layers.sort_by(|a: &Layer, b: &Layer| a.src.partial_cmp(&b.src).unwrap());
                        resources.push(layers);
                        layers = vec![];
                        continue;
                    }
                    continue;
                }
                let mut l = line.split_whitespace();
                let dest: u64 = l.next().unwrap().parse().unwrap();
                let src: u64 = l.next().unwrap().parse().unwrap();
                let range: u64 = l.next().unwrap().parse().unwrap();
                layers.push(Layer { dest, src, range });
            }
            if layers.is_empty().not() {
                layers.sort_by(|a: &Layer, b: &Layer| a.src.partial_cmp(&b.src).unwrap());
                resources.push(layers);
            }

            println!("{:?}", seeds_vec);
            Some((seeds_vec, resources))
        })
        .and_then(|(mut seeds_num, map)| {
            let mut i = map.iter();
            while let Some(layers) = i.next() {
                println!("{:?}", layers);
                seeds_num.iter_mut().for_each(|seed| {
                    let mut j = layers.iter();
                    while let Some(layer) = j.next() {
                        if seed.value >= layer.src && seed.value < layer.src + layer.range {
                            seed.value = layer.dest + seed.value - layer.src;
                            break;
                        }
                    }
                });
            }

            seeds_num.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
            return Some(seeds_num);
        })
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .value;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(35, process(contents).unwrap());
    }
}
