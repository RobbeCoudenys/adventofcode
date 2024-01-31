use std::vec;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Spring {
    Unknown,
    Working,
    Broken,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value.into() {
            '?' => Self::Unknown,
            '.' => Self::Working,
            '#' => Self::Broken,
            _ => panic!("{} should not be present", value),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct SpringRow {
    springs: Vec<Spring>,
    checksum: Vec<usize>,
}

impl From<&str> for SpringRow {
    fn from(row: &str) -> Self {
        let mut split = row.split(' ');

        // springs
        let springs = split
            .next()
            .unwrap()
            .chars()
            .map(|c| Spring::from(c))
            .collect::<Vec<Spring>>();

        // checksum
        let checksum = split
            .next()
            .unwrap()
            .split(',')
            .map(|nr_as_str| nr_as_str.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Self::new(springs, checksum)
    }
}

impl SpringRow {
    fn new(springs: Vec<Spring>, checksum: Vec<usize>) -> Self {
        // let checksum_sum = checksum.iter().sum();
        Self { springs, checksum }
    }

    fn to_s2(self) -> Self {
        let mut springs = Vec::new();
        let mut checksum = Vec::new();

        for _ in 0..4 {
            springs.extend_from_slice(&self.springs);
            springs.push(Spring::Unknown);

            checksum.extend_from_slice(&self.checksum);
        }

        // Add the last set of springs without adding Spring::Unknown
        springs.extend_from_slice(&self.springs);
        checksum.extend_from_slice(&self.checksum);

        Self::new(springs, checksum)
    }

    // inspired by: https://nickymeuleman.netlify.app/garden/aoc2023-day12
    // Could not find efficient solution myself
    fn count_possible_arangements(&mut self) -> usize {
        // making broken recursion easier
        self.springs.push(Spring::Working);
        let mut cache = vec![vec![None; self.springs.len()]; self.checksum.len()];
        self.count_possible_arangements_inner(&mut cache)
    }

    fn count_possible_arangements_inner(&self, cache: &mut Vec<Vec<Option<usize>>>) -> usize {
        if self.checksum.is_empty() {
            return if self.springs.contains(&Spring::Broken) {
                // Too many previous unknowns were counted as damaged
                0
            } else {
                // All remaining unknowns are operational
                1
            };
        }
        if self.springs.len() < self.checksum.iter().sum::<usize>() + self.checksum.len() {
            // Not enough space for remaining numbers
            return 0;
        }
        if let Some(cached) = cache[self.checksum.len() - 1][self.springs.len() - 1] {
            return cached;
        }
        let mut arangements = 0;
        if self.springs[0] != Spring::Broken {
            // Assume operational
            arangements += Self::new(self.springs[1..].to_vec(), self.checksum.clone())
                .count_possible_arangements_inner(cache);
        }
        let next_group_size = self.checksum[0];
        if !self.springs[..next_group_size].contains(&Spring::Working)
            && self.springs[next_group_size] != Spring::Broken
        {
            // Assume damaged
            arangements += Self::new(
                self.springs[next_group_size + 1..].to_vec(),
                self.checksum[1..].to_vec().clone(),
            )
            .count_possible_arangements_inner(cache);
        }
        cache[self.checksum.len() - 1][self.springs.len() - 1] = Some(arangements);
        arangements
    }
}

fn fast_solution(spring_rows: Vec<SpringRow>) -> usize {
    spring_rows
        .into_iter()
        .map(|mut sr| sr.count_possible_arangements())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::{get_input, get_rows};

    use super::*;

    #[test]
    fn test_spring_row_parser() {
        let row = "???.### 1,1,3";
        let spring = SpringRow::from(row);
        assert_eq!(&Spring::Unknown, spring.springs.get(0).unwrap());
        assert_eq!(&Spring::Unknown, spring.springs.get(1).unwrap());
        assert_eq!(&Spring::Unknown, spring.springs.get(2).unwrap());
        assert_eq!(&Spring::Working, spring.springs.get(3).unwrap());
        assert_eq!(&Spring::Broken, spring.springs.get(4).unwrap());
        assert_eq!(&Spring::Broken, spring.springs.get(5).unwrap());
        assert_eq!(&Spring::Broken, spring.springs.get(6).unwrap());

        assert_eq!(&1, spring.checksum.get(0).unwrap());
        assert_eq!(&1, spring.checksum.get(1).unwrap());
        assert_eq!(&3, spring.checksum.get(2).unwrap());

        let row = "...?#...???..?#?. 2,2,2";
        let spring = SpringRow::from(row);

        assert_eq!(&Spring::Working, spring.springs.get(0).unwrap());
        assert_eq!(&Spring::Working, spring.springs.get(1).unwrap());
        assert_eq!(&Spring::Working, spring.springs.get(2).unwrap());
        assert_eq!(&Spring::Unknown, spring.springs.get(3).unwrap());
        assert_eq!(&Spring::Broken, spring.springs.get(4).unwrap());
        assert_eq!(&Spring::Working, spring.springs.get(5).unwrap());
        assert_eq!(&Spring::Working, spring.springs.get(6).unwrap());
        assert_eq!(&Spring::Working, spring.springs.get(7).unwrap());
        assert_eq!(&Spring::Unknown, spring.springs.get(8).unwrap());
        assert_eq!(&Spring::Unknown, spring.springs.get(9).unwrap());
        assert_eq!(&Spring::Unknown, spring.springs.get(10).unwrap());
        assert_eq!(&Spring::Working, spring.springs.get(11).unwrap());
        assert_eq!(&Spring::Working, spring.springs.get(12).unwrap());
        assert_eq!(&Spring::Unknown, spring.springs.get(13).unwrap());
        assert_eq!(&Spring::Broken, spring.springs.get(14).unwrap());
        assert_eq!(&Spring::Unknown, spring.springs.get(15).unwrap());
        assert_eq!(&Spring::Working, spring.springs.get(16).unwrap());
    }

    #[test]
    fn test_possible_arrangements() {
        test_possible_arrangements_helper("???.### 1,1,3", 1);
        test_possible_arrangements_helper(".??..??...?##. 1,1,3", 4);
        test_possible_arrangements_helper("?#?#?#?#?#?#?#? 1,3,1,6", 1);
        test_possible_arrangements_helper("????.#...#... 4,1,1", 1);
        test_possible_arrangements_helper("????.######..#####. 1,6,5", 4);
        test_possible_arrangements_helper("?###???????? 3,2,1", 10);
    }

    #[test]
    fn test_valid_solution() {
        assert_eq!(
            0,
            SpringRow::new(
                vec![
                    Spring::Broken,
                    Spring::Broken,
                    Spring::Broken,
                    Spring::Working,
                    Spring::Broken,
                    Spring::Broken,
                ],
                vec![3, 2, 1]
            )
            .count_possible_arangements()
        );
        assert_eq!(
            1,
            SpringRow::new(
                vec![
                    Spring::Broken,
                    Spring::Working,
                    Spring::Working,
                    Spring::Working,
                    Spring::Broken,
                    Spring::Working,
                    Spring::Broken,
                    Spring::Broken,
                    Spring::Broken
                ],
                vec![1, 1, 3]
            )
            .count_possible_arangements()
        );
        assert_eq!(
            1,
            SpringRow::from(".#..#...###. 1,1,3").count_possible_arangements()
        );
        assert_eq!(
            0,
            SpringRow::from(".#..#..####. 1,1,3").count_possible_arangements()
        );
    }

    fn test_possible_arrangements_helper(input: &str, expected: usize) {
        let mut spring = SpringRow::from(input);
        assert_eq!(expected, spring.count_possible_arangements());
    }

    #[test]
    fn test_extension() {
        let sr = SpringRow::from(".#..#...###. 1,1,3");
        let sr = sr.to_s2();
        assert_eq!(15, sr.checksum.len());
        assert_eq!(64, sr.springs.len());
    }

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let spring_rows = rows
            .into_iter()
            .filter(|r| !r.is_empty())
            .map(|r| SpringRow::from(r.as_str()))
            .collect::<Vec<SpringRow>>();
        assert_eq!(21, fast_solution(spring_rows));
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let spring_rows = rows
            .into_iter()
            .filter(|r| !r.is_empty())
            .map(|r| SpringRow::from(r.as_str()))
            .collect::<Vec<SpringRow>>();
        assert_eq!(7407, fast_solution(spring_rows));
    }

    #[test]
    fn example_2_test() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let spring_rows = rows
            .into_iter()
            .filter(|r| !r.is_empty())
            .map(|r| SpringRow::from(r.as_str()).to_s2())
            .collect::<Vec<SpringRow>>();

        assert_eq!(525152, fast_solution(spring_rows));
    }

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let spring_rows = rows
            .into_iter()
            .filter(|r| !r.is_empty())
            .map(|r| SpringRow::from(r.as_str()).to_s2())
            .collect::<Vec<SpringRow>>();

        assert_eq!(30568243604962, fast_solution(spring_rows));
    }
}
