fn parse_input(input: String) -> Vec<(u128, Vec<u128>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result: u128 = parts.next().unwrap().parse().unwrap();
            let parts = parts.next().unwrap().split(" ");
            let numbers: Vec<u128> = parts.map(|part| part.parse().unwrap()).collect();
            (result, numbers)
        })
        .collect()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

impl Operation {
    fn apply(&self, a: u128, b: u128) -> u128 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concat => format!("{}{}", a, b).parse().unwrap(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Operations {
    operations: Vec<Operation>,
    allow_concat: bool,
}

impl Operations {
    fn new(size: usize, allow_concat: bool) -> Self {
        Operations {
            operations: vec![Operation::Add; size],
            allow_concat,
        }
    }

    fn next(&mut self) -> Option<()> {
        if self.operations.iter().all(|op| op == &Operation::Multiply) {
            return None;
        }
        let mut switch_next_bit = false;
        for index in 0..self.operations.len() {
            if index == 0 {
                if let Some(first) = self.operations.get_mut(index) {
                    match first {
                        Operation::Add => {
                            if self.allow_concat {
                                *first = Operation::Concat;
                            } else {
                                *first = Operation::Multiply;
                            }
                        }
                        Operation::Multiply => {
                            *first = Operation::Add;
                            switch_next_bit = true;
                        }
                        Operation::Concat => {
                            *first = Operation::Multiply;
                        }
                    }
                } else {
                    unreachable!();
                }
                continue;
            }
            if let Some(op) = self.operations.get_mut(index) {
                if switch_next_bit {
                    match op {
                        Operation::Add => {
                            if self.allow_concat {
                                *op = Operation::Concat;
                            } else {
                                *op = Operation::Multiply;
                            }
                            switch_next_bit = false;
                            continue;
                        }
                        Operation::Multiply => {
                            *op = Operation::Add;
                            switch_next_bit = true;
                            continue;
                        }
                        Operation::Concat => {
                            *op = Operation::Multiply;
                            switch_next_bit = false;
                            continue;
                        }
                    }
                }
            }
        }
        Option::Some(())
    }
}

fn can_operations_combine_result(input: &(u128, Vec<u128>), allow_concat: bool) -> bool {
    let (result, numbers) = input;
    if numbers.iter().any(|n| n >= &&result) {
        return false;
    }
    let mut operations = Operations::new(numbers.len() - 1, allow_concat);
    loop {
        let mut tmp_result = numbers[0];
        for (index, operation) in operations.operations.iter().enumerate() {
            tmp_result = operation.apply(tmp_result, numbers[index + 1]);
        }
        if &tmp_result == result {
            return true;
        }
        if operations.next().is_none() {
            break;
        }
    }
    false
}
// Old way of solving part 1
fn can_operations_combine_result_old(input: &(u128, Vec<u128>)) -> bool {
    let (result, numbers) = input;
    if numbers.iter().any(|n| n >= &&result) {
        return false;
    }
    for operation_mix in 0..2usize.pow((numbers.len() - 1) as u32) {
        let mut tmp_result = numbers[0];
        for number_index in 1..numbers.len() {
            let operation = (operation_mix >> (number_index - 1)) & 0b1;

            match operation {
                0 => tmp_result += numbers[number_index],
                1 => tmp_result *= numbers[number_index],
                _ => unreachable!(),
            }
            if &tmp_result == result {
                return true;
            }
        }
    }
    return false;
}

fn part2(input: &(u128, Vec<u128>)) -> bool {
    let (result, numbers) = input;
    if numbers.iter().any(|n| n >= &&result) {
        return false;
    }
    if &numbers.iter().filter(|nr| nr != &&1u128).sum::<u128>() > result {
        return false;
    }
    for operation_mix in 0..2usize.pow((numbers.len() - 1) as u32) {
        let mut tmp_result = numbers[0];
        for number_index in 1..numbers.len() {
            let operation = (operation_mix >> (number_index - 1)) & 1;

            match operation {
                0 => tmp_result += numbers[number_index],
                1 => tmp_result *= numbers[number_index],
                _ => unreachable!(),
            }
            if &tmp_result == result {
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn test_operations_iterator() {
        let mut operations = Operations::new(3, false);
        assert_eq!(
            vec![Operation::Add, Operation::Add, Operation::Add],
            operations.operations
        );
        operations.next();
        assert_eq!(
            vec![Operation::Multiply, Operation::Add, Operation::Add],
            operations.operations
        );
        operations.next();
        assert_eq!(
            vec![Operation::Add, Operation::Multiply, Operation::Add],
            operations.operations
        );
        operations.next();
        assert_eq!(
            vec![Operation::Multiply, Operation::Multiply, Operation::Add],
            operations.operations
        );
        operations.next();
        assert_eq!(
            vec![Operation::Add, Operation::Add, Operation::Multiply],
            operations.operations
        );
        operations.next();
        assert_eq!(
            vec![Operation::Multiply, Operation::Add, Operation::Multiply],
            operations.operations
        );
        operations.next();
        assert_eq!(
            vec![Operation::Add, Operation::Multiply, Operation::Multiply],
            operations.operations
        );
        operations.next();
        assert_eq!(
            vec![
                Operation::Multiply,
                Operation::Multiply,
                Operation::Multiply
            ],
            operations.operations
        );
        assert_eq!(None, operations.next());
    }

    #[test]
    fn example_1_can_operation_combine_result() {
        assert_eq!(
            true,
            can_operations_combine_result(&(5, vec![1, 2, 3]), false)
        );
    }

    #[test]
    fn example_1() {
        let input = get_input(file!(), "example.txt");
        let parsed = parse_input(input);
        assert_eq!(
            3749u128,
            parsed
                .into_iter()
                .filter(|i| can_operations_combine_result(i, false))
                .map(|(result, _)| result)
                .sum()
        );
    }

    #[test]
    fn input_1() {
        let input = get_input(file!(), "input.txt");
        let parsed = parse_input(input);
        assert_eq!(
            21572148763543u128,
            parsed
                .into_iter()
                .filter(|i| can_operations_combine_result(i, false))
                .map(|(result, _)| result)
                .sum()
        );
    }

    #[test]
    fn example_2() {
        let input = get_input(file!(), "example.txt");
        let parsed = parse_input(input);
        assert_eq!(
            11387u128,
            parsed
                .into_iter()
                .filter(|i| can_operations_combine_result(i, true))
                .map(|(result, _)| result)
                .sum()
        );
    }

    #[test]
    fn input_2() {
        let input = get_input(file!(), "input.txt");
        let parsed = parse_input(input);
        assert_eq!(
            581941094529163u128,
            parsed
                .into_iter()
                .filter(|i| can_operations_combine_result(i, true))
                .map(|(result, _)| result)
                .sum()
        );
    }
}
