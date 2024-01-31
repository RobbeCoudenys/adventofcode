use std::ops::Range;

use super::seeds::Seeds;

#[derive(PartialEq)]
struct AlmenacRange {
    source: usize,
    destination: usize,
    length: usize,
}

impl AlmenacRange {
    fn calculate_value_for_seed(option: Option<&Self>, input: usize) -> usize {
        match option {
            Some(mapping) => {
                return mapping.get_value_for_seed(input);
            }
            None => input,
        }
    }

    fn get_value_for_seed(&self, input: usize) -> usize {
        input - self.source + self.destination
    }

    fn get_min(&self) -> usize {
        self.source
    }

    fn get_max(&self) -> usize {
        self.source + self.length - 1
    }
}

struct AlmenacMap {
    from: String,
    to: String,
    mappings: Vec<AlmenacRange>,
}

impl PartialEq for AlmenacMap {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
    }
}

impl AlmenacMap {
    fn get_corresponding_number(&self, input: usize) -> usize {
        AlmenacRange::calculate_value_for_seed(self.get_matched_range(input), input)
    }

    fn get_matched_range(&self, input: usize) -> Option<&AlmenacRange> {
        if self.mappings.last().unwrap().get_min() > input {
            return None;
        }
        for mapping in &self.mappings {
            if mapping.get_min() <= input && input <= mapping.get_max() {
                return Some(mapping);
            }
        }
        None
    }

    fn get_ranges_inside_map(&self, seeds: Range<usize>) -> Seeds {
        let mut ranges = Seeds::new();
        for mapping in &self.mappings {
            let mut start = seeds.start;
            let mut end = seeds.end;
            if mapping.get_min() >= seeds.end || mapping.get_max() <= seeds.start {
                continue;
            }
            if mapping.get_min() > seeds.start {
                start = mapping.get_min();
            }
            if mapping.get_max() < seeds.end {
                end = mapping.get_max();
            }

            ranges.push(Range {
                start: mapping.get_value_for_seed(start),
                end: mapping.get_value_for_seed(end),
            });
        }
        if ranges.is_empty() {
            ranges.push(seeds);
        }
        ranges
    }

    fn get_corresponding_ranges(&self, seeds: Seeds) -> Seeds {
        let mut new_seeds = Seeds::new();
        for seed in seeds {
            let start_mapping_match = self.get_matched_range(seed.start);
            let end_mapping_match = self.get_matched_range(seed.end);
            if start_mapping_match == end_mapping_match {
                new_seeds.push(Range {
                    start: AlmenacRange::calculate_value_for_seed(start_mapping_match, seed.start),
                    end: AlmenacRange::calculate_value_for_seed(end_mapping_match, seed.end),
                });
            } else {
                for test in self.get_ranges_inside_map(seed) {
                    new_seeds.push(test);
                }
            }
        }
        new_seeds
    }
}

impl From<&str> for AlmenacMap {
    fn from(value: &str) -> Self {
        let mut from = String::new();
        let mut to = String::new();
        let mut mappings = Vec::new();
        for (index, row) in value.split('\n').enumerate() {
            if row.is_empty() {
                continue;
            }
            if index == 0 {
                if let Some(from_and_to_str) = row.strip_suffix(" map:") {
                    let mut split = from_and_to_str.split('-');
                    from.push_str(split.next().unwrap());
                    split.next();
                    to.push_str(split.next().unwrap());
                }
                continue;
            }
            mappings.push(AlmenacRange::from(row));
        }
        mappings.sort_by(|m1, m2| m2.source.cmp(&m1.source));
        Self { from, to, mappings }
    }
}

impl From<&str> for AlmenacRange {
    fn from(value: &str) -> Self {
        let mut values_as_str = value.split(' ');
        let destination: usize = values_as_str.next().unwrap().parse().unwrap();
        let source: usize = values_as_str.next().unwrap().parse().unwrap();
        let length: usize = values_as_str.next().unwrap().parse().unwrap();

        Self {
            source,
            destination,
            length,
        }
    }
}

struct Game {
    seeds: Seeds,
    maps: Vec<AlmenacMap>,
}

impl From<String> for Game {
    fn from(value: String) -> Self {
        let mut splits = value.split("\n\n");

        let seeds = Seeds::from(splits.next().unwrap());
        let mut maps = Vec::new();
        for map_as_str in splits {
            if map_as_str.is_empty() {
                continue;
            }
            maps.push(AlmenacMap::from(map_as_str));
        }
        Self { seeds, maps }
    }
}

impl Game {
    fn get_corresponding_ranges(
        &self,
        input: Range<usize>,
        start: &str,
        destination: &str,
    ) -> Seeds {
        let mut last_to = start;
        let mut corresponding_ranges = Seeds::new();
        corresponding_ranges.push(input);
        while last_to.ne(destination) {
            (last_to, corresponding_ranges) = self.get_next(last_to, corresponding_ranges);
        }
        corresponding_ranges
    }

    fn get_next(&self, start: &str, corresponding_ranges: Seeds) -> (&str, Seeds) {
        let corresponding_map = self.maps.iter().find(|m| m.from.eq(start)).unwrap();
        (
            &corresponding_map.to,
            corresponding_map.get_corresponding_ranges(corresponding_ranges),
        )
    }
}

fn solution(game: &Game, start: String, destination: String) -> usize {
    game.seeds
        .clone()
        .into_iter()
        .map(|seed| {
            game.get_corresponding_ranges(seed, &start, &destination)
                .into_iter()
                .map(|r| r.start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::shared::{file_parser::get_input, method_duration::log_method_duration};

    use super::*;

    #[test]
    fn seeds_from_string() {
        let seeds_as_str = "seeds: 79 14 55 13";
        let seeds = Seeds::from(seeds_as_str);
        assert_eq!(2, seeds.len());
        assert_eq!(79, seeds.get(0).unwrap().start);
        assert_eq!(92, seeds.get(0).unwrap().end);
        assert_eq!(55, seeds.get(1).unwrap().start);
        assert_eq!(67, seeds.get(1).unwrap().end);
    }

    #[test]
    fn map_from_string() {
        let map_as_str = "soil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15";
        let almenac_map = AlmenacMap::from(map_as_str);
        assert_eq!("soil", &almenac_map.from);
        assert_eq!("fertilizer", &almenac_map.to);
        assert_eq!(&0, &almenac_map.mappings.get(1).unwrap().destination);
        assert_eq!(&15, &almenac_map.mappings.get(1).unwrap().source);
        assert_eq!(&37, &almenac_map.mappings.get(1).unwrap().length);

        assert_eq!(&37, &almenac_map.mappings.get(0).unwrap().destination);
        assert_eq!(&52, &almenac_map.mappings.get(0).unwrap().source);
        assert_eq!(&2, &almenac_map.mappings.get(0).unwrap().length);

        assert_eq!(&39, &almenac_map.mappings.get(2).unwrap().destination);
        assert_eq!(&0, &almenac_map.mappings.get(2).unwrap().source);
        assert_eq!(&15, &almenac_map.mappings.get(2).unwrap().length);
    }

    #[test]
    fn test_get_corresponding_ranges() {
        let map_as_str = "soil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15";
        let almenac_map = AlmenacMap::from(map_as_str);
        almenac_map.test_corresponding_range_single_value(12, 51);
        almenac_map.test_corresponding_range_single_value(14, 53);
        almenac_map.test_corresponding_range_single_value(15, 0);
        almenac_map.test_corresponding_range_single_value(51, 36);
        almenac_map.test_corresponding_range_single_value(52, 37);
        almenac_map.test_corresponding_range_single_value(53, 38);
        almenac_map.test_corresponding_range_single_value(54, 54);
        almenac_map.test_corresponding_range_single_value(55, 55);
    }

    impl AlmenacMap {
        fn test_corresponding_range_single_value(&self, input: usize, expected_output: usize) {
            let ranges = self.get_corresponding_ranges(Seeds::from(vec![Range {
                start: input,
                end: input,
            }]));
            assert_eq!(1, ranges.len());
            let range = ranges.get(0).unwrap();
            assert_eq!(expected_output, range.start, "{}", range.start);
            assert_eq!(expected_output, range.end);
        }
    }

    #[test]
    fn soil_to_fertilizer_corresponding_number() {
        let map_as_str = "seed-to-soil map:\n50 98 2\n52 50 48";
        let almenac_map = AlmenacMap::from(map_as_str);
        assert_eq!(13, AlmenacMap::get_corresponding_number(&almenac_map, 13));
        assert_eq!(14, AlmenacMap::get_corresponding_number(&almenac_map, 14));
        assert_eq!(81, AlmenacMap::get_corresponding_number(&almenac_map, 79));
        assert_eq!(57, AlmenacMap::get_corresponding_number(&almenac_map, 55));
        assert_eq!(99, AlmenacMap::get_corresponding_number(&almenac_map, 97));
        assert_eq!(50, AlmenacMap::get_corresponding_number(&almenac_map, 98));
        assert_eq!(51, AlmenacMap::get_corresponding_number(&almenac_map, 99));
    }

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");
        let game = Game::from(input);
        assert_eq!(
            35,
            solution(&game, String::from("seed"), String::from("location"))
        );
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        let game = Game::from(input);
        assert_eq!(
            31599214,
            solution(&game, String::from("seed"), String::from("location"))
        );
    }

    #[test]
    fn example_2_test() {
        let input = get_input(file!(), "example2.txt");
        let game = Game::from(input);
        assert_eq!(
            46,
            solution(&game, String::from("seed"), String::from("location"))
        );
    }

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input2.txt");
        let game = Game::from(input);
        let solution =
            log_method_duration(|| solution(&game, String::from("seed"), String::from("location")));
        assert_eq!(20358599, solution);
    }
}
