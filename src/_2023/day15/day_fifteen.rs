use std::{
    collections::HashMap,
    ops::{AddAssign, MulAssign},
};

enum Operation {
    EQUAL(String, usize),
    DASH(String),
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        if value.contains('=') {
            let mut split = value.split('=');
            let label = String::from(split.next().unwrap());
            let focal_length = split.next().unwrap();
            return Self::EQUAL(label, focal_length.parse().unwrap());
        }
        if value.contains('-') {
            let mut split = value.split('-');
            let label = String::from(split.next().unwrap());
            return Self::DASH(label);
        }
        panic!("Cannot parse {}", value);
    }
}

trait CustomHash {
    fn custom_hash(&self) -> usize;
}

impl CustomHash for String {
    fn custom_hash(&self) -> usize {
        let mut value = 0;
        for c in self.chars() {
            let ascii_value = c as usize;
            value.add_assign(ascii_value);
            value.mul_assign(17);
            value = value % 256;
        }
        value
    }
}

type Lenses = Vec<(String, usize)>;
type Boxes = HashMap<usize, Lenses>;

trait FocusingPower {
    fn focusing_power(&self) -> usize;
}

impl FocusingPower for Boxes {
    fn focusing_power(&self) -> usize {
        self.iter()
            .flat_map(|(box_nr, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(move |(index, lens)| (box_nr + 1) * (index + 1) * lens.1)
            })
            .sum()
    }
}

fn solution_2(input: String) -> usize {
    let operations = input
        .split(',')
        .map(Operation::from)
        .collect::<Vec<Operation>>();

    let mut boxes: Boxes = HashMap::new();

    for operation in operations {
        match operation {
            Operation::EQUAL(label, focal_length) => {
                let lenses = boxes.entry(label.custom_hash()).or_insert_with(Vec::new);
                match lenses.iter_mut().find(|lens| lens.0 == label) {
                    Some(lens) => lens.1 = focal_length,
                    None => lenses.push((label, focal_length)),
                }
            }
            Operation::DASH(label) => {
                if let Some(lenses) = boxes.get_mut(&label.custom_hash()) {
                    lenses.retain(|lens| lens.0 != label);
                }
            }
        }
    }

    boxes.focusing_power()
}

#[cfg(test)]
mod tests {

    use crate::shared::{file_parser::get_input, method_duration::log_method_duration};

    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(52, String::from("HASH").custom_hash());
        assert_eq!(30, String::from("rn=1").custom_hash());
    }

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");
        assert_eq!(
            1320usize,
            input
                .split(',')
                .map(|sequence| String::from(sequence).custom_hash())
                .sum()
        )
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        assert_eq!(
            517315usize,
            input
                .split(',')
                .map(|sequence| String::from(sequence).custom_hash())
                .sum()
        )
    }

    #[test]
    fn example_2_test() {
        let input = get_input(file!(), "example1.txt");
        assert_eq!(145, solution_2(input));
    }

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input1.txt");
        assert_eq!(247763, log_method_duration(|| solution_2(input)));
    }
}
