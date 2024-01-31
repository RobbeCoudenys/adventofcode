use core::num;
use std::{collections::HashMap, fs, ops::Add, path::Path};

fn extract_number(mut text: String, convert_spelled_numbers: bool) -> i32 {
    if convert_spelled_numbers {
        return replace_spelled_numbers(text);
    }
    let mut numbers: [Option<String>; 2] = [None, None];
    for n in text.chars() {
        if n.is_numeric() {
            if numbers[0].is_none() {
                numbers[0] = Some(n.to_string());
            } else {
                numbers[1] = Some(n.to_string());
            }
        }
    }
    if numbers[0].is_none() {
        return 0;
    }
    if numbers[1].is_none() {
        numbers[1] = numbers[0].clone();
    }
    numbers
        .iter()
        .map(|option| match option {
            Some(number) => number.to_owned(),
            None => String::from(""),
        })
        .collect::<String>()
        .parse()
        .expect("Should not happen")
}

fn replace_spelled_numbers(input: String) -> i32 {
    if input.is_empty() {
        return 0;
    }
    let mut numbers = Vec::<(usize, String)>::new();

    let mut concat_matching_indice = |str_to_match: &str, value_to_use: &str| {
        for (index, _) in input
            .match_indices(str_to_match)
            .collect::<Vec<(usize, &str)>>()
        {
            numbers.push((index, value_to_use.to_owned()));
        }
    };
    let mut concat_matching_indices = |spelled_number: &str, numeric_number: &str| {
        concat_matching_indice(spelled_number, numeric_number);
        concat_matching_indice(numeric_number, numeric_number);
    };
    concat_matching_indices("zero", "0");
    concat_matching_indices("one", "1");
    concat_matching_indices("two", "2");
    concat_matching_indices("three", "3");
    concat_matching_indices("four", "4");
    concat_matching_indices("five", "5");
    concat_matching_indices("six", "6");
    concat_matching_indices("seven", "7");
    concat_matching_indices("eight", "8");
    concat_matching_indices("nine", "9");
    numbers.sort_by(|(indexa, _), (indexb, _)| indexa.cmp(indexb));
    let mut output = String::from("");
    match numbers.first() {
        Some((_, value)) => output.push_str(value),
        None => todo!(),
    };
    match numbers.last() {
        Some((_, value)) => output.push_str(value),
        None => todo!(),
    };
    match output.parse() {
        Ok(result) => result,
        Err(_) => todo!(),
    }
}

fn extract_total(input: Vec<String>, convert_spelled_numbers: bool) -> i32 {
    let mut total: i32 = 0;
    for row in input {
        total += extract_number(row, convert_spelled_numbers);
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::day1::day_one::{extract_number, extract_total};
    use crate::shared::file_parser::{get_input, get_rows};

    use super::replace_spelled_numbers;

    #[test]
    fn example_two_digit_number() {
        let test_cases: Vec<(String, i32)> = vec![
            (String::from("1abc2"), 12),
            (String::from("pqr3stu8vwx"), 38),
            (String::from("a1b2c3d4e5f"), 15),
            (String::from("treb7uchet"), 77),
        ];

        for (input, expected) in test_cases {
            assert_eq!(extract_number(input, false), expected);
        }
    }

    #[test]
    fn first_example() {
        let input = get_input(file!(), "example.txt");
        assert_eq!(extract_total(get_rows(input), false), 142);
    }

    #[test]
    fn first_answer() {
        let input = get_input(file!(), "input.txt");
        assert_eq!(extract_total(get_rows(input), false), 55029);
    }

    #[test]
    fn test_number_replacement() {
        let test_cases: Vec<(String, i32)> = vec![
            (String::from("onetwothreefourfivesixseveneightnine"), 19),
            (String::from("oneight"), 18),
            (String::from("one2eighthree"), 13),
        ];
        for (input, expected) in test_cases {
            assert_eq!(replace_spelled_numbers(input), expected);
        }
        let test_cases: Vec<(String, i32)> = vec![
            (String::from("two1nine"), 29),
            (String::from("eightwothree"), 83),
            (String::from("abcone2threexyz"), 13),
            (String::from("xtwone3four"), 24),
            (String::from("4nineeightseven2"), 42),
            (String::from("zoneight234"), 14),
            (String::from("7pqrstsixteen"), 76),
        ];

        for (input, expected) in test_cases {
            assert_eq!(extract_number(input, true), expected);
        }
    }

    #[test]
    fn second_example() {
        let input = get_input(file!(), "example2.txt");
        assert_eq!(extract_total(get_rows(input), true), 281);
    }

    #[test]
    fn second_example_solution() {
        let input = get_input(file!(), "input.txt");
        assert_eq!(extract_total(get_rows(input), true), 55686);
    }
}
