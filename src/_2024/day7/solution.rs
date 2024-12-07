#[derive(Clone)]
enum Operation {
    Add,
    Multiply,
}

fn parse_input(input: String) -> Vec<(u128, Vec<u128>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result: u128 = parts.next().unwrap().parse().unwrap();
            let mut parts = parts.next().unwrap().split(" ");
            let numbers: Vec<u128> = parts.map(|part| part.parse().unwrap()).collect();
            (result, numbers)
        })
        .collect()
}

fn can_operations_combine_result(input: &(u128, Vec<u128>)) -> bool {
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
    fn example_1_can_operation_combine_result() {
        assert_eq!(true, can_operations_combine_result(&(5, vec![1, 2, 3])));
    }

    #[test]
    fn example_1() {
        let input = get_input(file!(), "example.txt");
        let parsed = parse_input(input);
        assert_eq!(
            3749u128,
            parsed
                .into_iter()
                .filter(|i| can_operations_combine_result(i))
                .map(|(result, _)| result)
                .sum()
        );
    }

    #[test]
    fn input_1() {
        let input = get_input(file!(), "input.txt");
        let parsed = parse_input(input);
        assert_eq!(
            3749u128,
            parsed
                .into_iter()
                .filter(|i| can_operations_combine_result(i))
                .map(|(result, _)| result)
                .sum()
        );
    }

    #[test]
    fn example_2() {
        let input = get_input(file!(), "example.txt");
    }

    #[test]
    fn input_2() {
        let input = get_input(file!(), "input.txt");
    }
}
