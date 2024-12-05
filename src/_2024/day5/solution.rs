use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn split_input(input: String) -> (String, String) {
    let mut split = input.split("\n\n");
    (
        split.next().unwrap().to_string(),
        split.next().unwrap().to_string(),
    )
}

fn page_order_index(page_ordering: String) -> HashMap<u32, HashSet<u32>> {
    let mut result: HashMap<u32, HashSet<u32>> = HashMap::new();
    page_ordering
        .lines()
        .map(|line| {
            let mut split = line.split("|");
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .for_each(|(a, b)| {
            result.entry(a).or_insert(HashSet::new()).insert(b);
        });
    result
}

fn page_numbers(page_numbers: String) -> Vec<Vec<u32>> {
    page_numbers
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

// Part 1: Find the sum of the middle elements of the rows that are valid
// Valid means that the elements are in the order defined by the page_ordering
fn part1(page_ordering: HashMap<u32, HashSet<u32>>, page_numbers: Vec<Vec<u32>>) -> Vec<u32> {
    let mut result = Vec::new();
    for row in page_numbers {
        let mut valid = true;
        for i in 0..row.len() - 1 {
            let a = row[i];
            let b = row[i + 1];
            if let Some(a_r) = page_ordering.get(&a) {
                if !a_r.contains(&b) {
                    valid = false;
                    break;
                }
            } else {
                valid = false;
                break;
            }
        }
        if valid {
            result.push(row[(row.len() - 1) / 2]);
        }
    }
    result
}

// Part 2: Find the invalid rows, place them in order and find the sum of the middle elements
fn part2(page_ordering: HashMap<u32, HashSet<u32>>, page_numbers: Vec<Vec<u32>>) -> Vec<u32> {
    let mut result = Vec::new();
    for row in page_numbers {
        let mut valid_order = row.clone();
        valid_order.sort_by(|a, b| {
            if let Some(a_r) = page_ordering.get(a) {
                if a_r.contains(b) {
                    return std::cmp::Ordering::Less;
                }
            }
            return std::cmp::Ordering::Greater;
        });
        if valid_order != row {
            result.push(valid_order[(valid_order.len() - 1) / 2]);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn example_1() {
        let input = get_input(file!(), "example.txt");
        let (page_ordering_str, page_numbers_str) = split_input(input);
        let page_ordering = page_order_index(page_ordering_str);
        let page_numbers = page_numbers(page_numbers_str);
        assert_eq!(143u32, part1(page_ordering, page_numbers).iter().sum());
    }

    #[test]
    fn input_1() {
        let input = get_input(file!(), "input.txt");
        let (page_ordering_str, page_numbers_str) = split_input(input);
        let page_ordering = page_order_index(page_ordering_str);
        let page_numbers = page_numbers(page_numbers_str);
        assert_eq!(4996u32, part1(page_ordering, page_numbers).iter().sum());
    }

    #[test]
    fn example_2() {
        let input = get_input(file!(), "example.txt");
        let (page_ordering_str, page_numbers_str) = split_input(input);
        let page_ordering = page_order_index(page_ordering_str);
        let page_numbers = page_numbers(page_numbers_str);
        assert_eq!(123u32, part2(page_ordering, page_numbers).iter().sum());
    }

    #[test]
    fn input_2() {
        let input = get_input(file!(), "input.txt");
        let (page_ordering_str, page_numbers_str) = split_input(input);
        let page_ordering = page_order_index(page_ordering_str);
        let page_numbers = page_numbers(page_numbers_str);
        assert_eq!(6311u32, part2(page_ordering, page_numbers).iter().sum());
    }
}
