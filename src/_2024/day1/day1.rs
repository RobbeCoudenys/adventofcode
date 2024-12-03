use std::{collections::HashMap, ops::AddAssign};

fn parse_input(input: String) -> (Vec<u32>, Vec<u32>) {
    let total = input.lines().count();
    let mut first_col: Vec<u32> = Vec::with_capacity(total);
    let mut second_col: Vec<u32> = Vec::with_capacity(total);
    input.split("\n").into_iter().for_each(|row| {
        let mut iter = row.split("   ");
        let number1: u32 = iter.next().unwrap().parse().unwrap();
        let number2: u32 = iter.next().unwrap().parse().unwrap();
        first_col.push(number1);
        second_col.push(number2);
    });

    (first_col, second_col)
}

fn calculate_avg_distance(mut input: (Vec<u32>, Vec<u32>)) -> u32 {
    let mut col1 = input.0;
    col1.sort();
    let mut col2 = input.1;
    col2.sort();

    let mut result: u32 = 0;
    for i in 0..col1.len() {
        result.add_assign(col1.get(i).unwrap().abs_diff(*col2.get(i).unwrap()));
    }
    result
}

fn calculate_similarity(mut input: (Vec<u32>, Vec<u32>)) -> u32 {
    let mut col1 = input.0;
    let mut col2 = input.1;
    col2.sort();
    let mut col2_map = HashMap::new();

    let mut current = 0;
    let mut occurences = 0;
    col2.into_iter().for_each(|nr| {
        if current == nr {
            occurences.add_assign(1);
            return;
        }
        col2_map.insert(current, occurences);
        current = nr;
        occurences = 1;
    });
    col2_map.insert(current, occurences);
    let mut result = 0;
    col1.into_iter().for_each(|nr| {
        result.add_assign(col2_map.get(&nr).unwrap_or(&0) * nr);
    });
    result
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn example_1() {
        let input = get_input(file!(), "example.txt");
        let parsed = parse_input(input);
        let result = calculate_avg_distance(parsed);
        assert_eq!(11, result);
    }

    #[test]
    fn input_1() {
        let input = get_input(file!(), "input.txt");
        let parsed = parse_input(input);
        let result = calculate_avg_distance(parsed);
        assert_eq!(1603498, result);
    }

    #[test]
    fn example_2() {
        let input = get_input(file!(), "example.txt");
        let parsed = parse_input(input);
        let result = calculate_similarity(parsed);
        assert_eq!(31, result);
    }

    #[test]
    fn input_2() {
        let input = get_input(file!(), "input.txt");
        let parsed = parse_input(input);
        let result = calculate_similarity(parsed);
        assert_eq!(25574739, result);
    }
}
