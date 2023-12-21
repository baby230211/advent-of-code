use std::ops::Not;

fn main() {
    let contents = advent_of_code_2023::run().unwrap();
    let res = process(&contents).unwrap();
    println!("{}", res);
}

#[derive(Debug)]
struct Seed {
    value: u64,
    range: u64,
}
impl Seed {
    fn is_include(&self, layer: &Layer) -> bool {
        self.value >= layer.src && self.value < layer.src + layer.range
    }
    fn generate_from_outer(&mut self, layer: &Layer) -> Option<Seed> {
        if self.is_include(layer) {
            // intersection outer
            if (self.value + self.range) > (layer.src + layer.range) {
                let new_seed = Seed {
                    value: layer.src + layer.range,
                    range: self.value + self.range - layer.src - layer.range,
                };

                self.range = layer.src + layer.range - self.value;
                self.value = self.value - layer.src + layer.dest;
                return Some(new_seed);
            }
            return None;
        }
        // intersection lower
        // src is lower than self.value
        // self.value + range is higher than src
        // split to two pieces and return the intersection one
        if self.value < layer.src && self.value + self.range > layer.src {
            let new_seed = Seed {
                value: layer.src,
                range: self.value + self.range - layer.src,
            };
            self.range = self.range - new_seed.range;
            return Some(new_seed);
        }
        None
    }
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
                .step_by(2)
                .zip(seeds.split(" ").skip(1).step_by(2))
                .map(|(start_val, range)| {
                    let start_val = start_val.parse::<u64>().unwrap();
                    let range = range.parse::<u64>().unwrap();
                    Seed {
                        value: start_val,
                        range,
                    }
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

            Some((seeds_vec, resources))
        })
        .and_then(|(mut seeds_num, map)| {
            let mut i = map.iter();

            while let Some(layers) = i.next() {
                println!("------------");
                println!("{:?}", layers);
                let mut temp: Vec<Seed> = vec![];
                seeds_num.iter_mut().for_each(|seed| {
                    let mut j = layers.iter();
                    while let Some(layer) = j.next() {
                        if let Some(new_seed) = seed.generate_from_outer(layer) {
                            temp.push(new_seed);
                            break;
                        } else if seed.is_include(layer) {
                            // all includes
                            seed.value = seed.value - layer.src + layer.dest;
                            break;
                        }
                    }
                });
                while temp.is_empty().not() {
                    let mut temp2: Vec<Seed> = vec![];
                    temp.iter_mut().for_each(|seed| {
                        let mut j = layers.iter();
                        while let Some(layer) = j.next() {
                            if let Some(new_seed) = seed.generate_from_outer(layer) {
                                temp2.push(new_seed);
                                break;
                            } else if seed.is_include(layer) {
                                // all includes
                                seed.value = seed.value - layer.src + layer.dest;
                                break;
                            }
                        }
                    });
                    seeds_num.append(&mut temp);
                    temp.clear();
                    temp.append(&mut temp2);
                }
                println!("{:?}", seeds_num);
                seeds_num.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
            }

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
56 93 4

aaaa map:
53 53 3
44 56 27";
        assert_eq!(46, process(contents).unwrap());
    }
}
