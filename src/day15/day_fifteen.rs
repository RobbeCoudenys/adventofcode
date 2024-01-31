use std::{
    collections::HashMap,
    hash::Hash,
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
        let mut result = 0;
        for (box_nr, lenses) in self.iter() {
            for (index, lens) in lenses.iter().enumerate() {
                result.add_assign((box_nr + 1) * (index + 1) * lens.1);
            }
        }
        result
    }
}

fn solution_2(input: String) -> usize {
    let operations = input
        .split(',')
        .map(|sequence| Operation::from(sequence))
        .collect::<Vec<Operation>>();
    let mut boxes: Boxes = HashMap::new();
    for operation in operations {
        match operation {
            Operation::EQUAL(label, focal_length) => {
                let custom_hash = label.custom_hash();
                match boxes.get_mut(&custom_hash) {
                    Some(lenses) => {
                        let index = lenses.iter().position(|l| l.0.eq(&label));
                        match index {
                            Some(index) => lenses.get_mut(index).unwrap().1 = focal_length,
                            None => lenses.push((label, focal_length)),
                        }
                    }
                    None => {
                        let mut lenses: Lenses = Vec::new();
                        lenses.push((label, focal_length));
                        boxes.insert(custom_hash, lenses);
                    }
                }
            }
            Operation::DASH(label) => match boxes.get_mut(&label.custom_hash()) {
                Some(lenses) => {
                    if let Some(index) = lenses.iter().position(|l| l.0.eq(&label)) {
                        lenses.remove(index);
                    }
                }
                None => (),
            },
        }
    }
    boxes.focusing_power()
}

#[cfg(test)]
mod tests {

    use crate::shared::file_parser::get_input;

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
        assert_eq!(247763, solution_2(input));
    }
}
