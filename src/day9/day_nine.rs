fn solution_1(input: Vec<Vec<isize>>) -> isize {
    let mut count = 0;
    for row in input {
        let mut last_value_from_each = vec![*row.last().unwrap()];
        let mut current_row = row;
        while current_row.first().unwrap() != &0isize || current_row.last().unwrap() != &0isize {
            let mut next_row = Vec::new();
            for (index, value) in current_row.iter().enumerate() {
                match current_row.get(index + 1) {
                    Some(next_value) => next_row.push(next_value - value),
                    None => last_value_from_each.push(*next_row.last().unwrap()),
                }
            }
            current_row = next_row;
        }
        // removing zero
        last_value_from_each.remove(last_value_from_each.len() - 1);
        last_value_from_each.reverse();
        let mut value_to_add = 0;
        for v in last_value_from_each {
            value_to_add += v;
        }
        count += value_to_add;
    }
    count
}

fn solution_2(input: Vec<Vec<isize>>) -> isize {
    let mut count = 0;
    for row in input {
        let mut first_value_from_each = Vec::new();
        let mut current_row = row;
        while current_row.first().unwrap() != &0isize || current_row.last().unwrap() != &0isize {
            let mut next_row = Vec::new();
            first_value_from_each.push(*current_row.first().unwrap());
            for (index, value) in current_row.iter().enumerate() {
                match current_row.get(index + 1) {
                    Some(next_value) => next_row.push(next_value - value),
                    None => (),
                }
            }
            current_row = next_row;
        }
        first_value_from_each.reverse();
        let mut value_to_add = 0;
        for v in first_value_from_each {
            value_to_add = v - value_to_add;
        }
        count += value_to_add;
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::{get_input, get_rows};

    use super::*;

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let test = rows
            .into_iter()
            .map(|row| {
                row.split(" ")
                    .map(|str| str.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .collect::<Vec<Vec<isize>>>();
        assert_eq!(114, solution_1(test));
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let test = rows
            .into_iter()
            .map(|row| {
                row.split(" ")
                    .map(|str| str.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .collect::<Vec<Vec<isize>>>();
        assert_eq!(114, solution_1(test));
    }

    #[test]
    fn example_2_test() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let test = rows
            .into_iter()
            .map(|row| {
                row.split(" ")
                    .map(|str| str.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .collect::<Vec<Vec<isize>>>();
        assert_eq!(2, solution_2(test));
    }

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let test = rows
            .into_iter()
            .map(|row| {
                row.split(" ")
                    .map(|str| str.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .collect::<Vec<Vec<isize>>>();
        assert_eq!(2, solution_2(test));
    }
}
