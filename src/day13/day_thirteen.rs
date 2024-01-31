use std::ops::{AddAssign, SubAssign};

use itertools::Itertools;

#[derive(Debug)]
struct Reflection {
    rows: Vec<String>,
    columns: Vec<String>,
    block: String, // Used for debugging/comparing solutions
}

trait DifferByOne {
    fn differ_by_one(&self, other: &String) -> bool;
}

impl DifferByOne for String {
    fn differ_by_one(&self, other: &String) -> bool {
        let mut has_difference = false;
        if self.len() != other.len() {
            return false;
        }
        let mut other_chars = other.chars();
        for ch in self.chars() {
            let other_char = other_chars.next().unwrap();
            if ch.ne(&other_char) {
                if has_difference {
                    return false;
                }
                has_difference = true;
            }
        }

        has_difference
    }
}

impl From<&str> for Reflection {
    fn from(block: &str) -> Self {
        let mut chunk_size = 0;
        let rows = block
            .split('\n')
            .map(|r| {
                chunk_size = r.len();
                r.chars().collect::<String>()
            })
            .collect::<Vec<String>>();
        let columns = block
            .chars()
            .filter(|&c| c != '\n')
            .enumerate()
            .fold(vec![String::new(); chunk_size], |mut acc, (i, c)| {
                acc[i % chunk_size].push(c);
                acc
            })
            .into_iter()
            .map(|c| c.chars().collect::<String>())
            .collect::<Vec<String>>();

        Self {
            rows,
            columns,
            block: String::from(block),
        }
    }
}

impl Reflection {
    fn list_mirror_index(&self, list: &Vec<String>, with_smudge: bool) -> Option<usize> {
        let last_index = list.len();
        let middle = list.len() / 2;
        for i in 0..last_index {
            let mut start_index = 0;
            let intersection = i + 1;
            let mut end_index = last_index;
            if intersection <= middle {
                end_index = intersection * 2;
            } else {
                start_index = intersection - (last_index - intersection);
            }

            if start_index.eq(&intersection) {
                start_index.sub_assign(1);
            }
            let first_half = &list[start_index..intersection].into_iter().join("");
            let second_half = &list[intersection..end_index].into_iter().rev().join("");
            // let xor = first_half ^ second_half;
            // let smudge_equals = xor != 0 && (xor & (xor - 1)) == 0;
            if with_smudge {
                if first_half.differ_by_one(second_half) {
                    return Some(i);
                }
            } else if first_half.eq(second_half) {
                return Some(i);
            }
        }
        None
    }

    fn value(&self, with_smudge: bool) -> usize {
        // checking horizontally
        let mut value = 0;
        if let Some(index) = self.list_mirror_index(&self.rows, with_smudge) {
            value.add_assign((index + 1) * 100);
        }

        // checking vertically
        if let Some(index) = self.list_mirror_index(&self.columns, with_smudge) {
            value.add_assign(index + 1);
        }
        value
    }
}

fn solution_1(input: String) -> usize {
    input
        .split("\n\n")
        .filter(|block| !block.is_empty())
        .map(|r| Reflection::from(r).value(false))
        .sum()
}

fn solution_2(input: String) -> usize {
    input
        .split("\n\n")
        .filter(|block| !block.is_empty())
        .map(|r| Reflection::from(r).value(true))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn test_parser() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
        let result = Reflection::from(input);
        assert_eq!("#.##..##.", result.rows.get(0).unwrap());
        assert_eq!("#.#.##.#.", result.rows.get(6).unwrap());

        assert_eq!("#.##..#", result.columns.get(0).unwrap());
        assert_eq!("..##...", result.columns.get(8).unwrap());
    }

    #[test]
    fn test_value() {
        test_value_helper(
            5,
            "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.",
        );

        test_value_helper(
            400,
            "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#",
        );

        test_value_helper(1, "###.###.#.#\n...##......\n..#########\n###.#......\n##....#####\n......#..##\n###....#...\n...##....##\n##...#..###");
        test_value_helper(100, "##..###....\n##..###....\n..#.#.###..\n.#..#.#..#.\n#####...#..\n.....#####.\n...#.#.####");
        test_value_helper(1, "##.....##\n..######.\n.....#.#.\n.....#.#.\n..######.\n##..#..##\n....###.#\n...####..\n....#.##.\n###.#...#\n###..##.#");
        test_value_helper(10, "......##...\n#.....##...\n###.#..#.##\n.#######.##\n#.#.#...###\n..##.####..\n.####..####");
        test_value_helper(1600, "#.#......#.\n..##....##.\n..#..##..#.\n..#.####.#.\n#..#.##.#..\n.####..####\n.#.######.#\n##.#....#.#\n..########.\n#.#.####.#.\n...#.##.#..\n#..........\n###.####.##\n#.###...##.\n#####..####\n.#...##...#\n.#...##...#");
    }

    fn test_value_helper(expected: usize, input: &str) {
        let reflection = Reflection::from(input);
        assert_eq!(expected, reflection.value(false), "\n{}", input);
    }

    #[test]
    fn test_differ_by_one() {
        assert!(String::from("0001000100010001").differ_by_one(&String::from("0001000000010001")));
    }

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");
        assert_eq!(405, solution_1(input));
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        assert_eq!(29846, solution_1(input));
    }

    #[test]
    fn example_2_test() {
        let input = get_input(file!(), "example1.txt");
        assert_eq!(400, solution_2(input));
    }

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input1.txt");
        assert_eq!(25401, solution_2(input));
    }
}
