use regex::Regex;

fn get_muls(input: String) -> Vec<(u64, u64)> {
    // regex that matches mul(43,425) or any mul(x,y) where x and y are numbers that can be any length and extracted into a tuple of u64
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    mul_regex
        .captures_iter(&input)
        .map(|cap| {
            (
                cap[1].parse::<u64>().unwrap(),
                cap[2].parse::<u64>().unwrap(),
            )
        })
        .collect()
}

fn remove_donts(input: String) -> String {
    input
        .split("do()")
        .map(|do_string| do_string.split("don't()").into_iter().next().unwrap())
        .collect()
}

fn calculate_mul(input: Vec<(u64, u64)>) -> u64 {
    input.iter().map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn example_1() {
        let input = get_input(file!(), "example.txt");
        let parsed = get_muls(input);
        let result = calculate_mul(parsed);
        assert_eq!(161, result);
    }

    #[test]
    fn example_1_extract_mul() {
        assert_eq!(vec![(324, 214)], get_muls(String::from("mul(324,214)")));
        assert_eq!(vec![(83, 25)], get_muls(String::from("mul(83,25)")));
    }

    #[test]
    fn input_1() {
        let input = get_input(file!(), "input.txt");
        let parsed = get_muls(input);
        let result = calculate_mul(parsed);
        assert_eq!(160672468, result);
    }

    #[test]
    fn example_2() {
        let input = get_input(file!(), "example.txt");
        let parsed = get_muls(remove_donts(input));
        assert_eq!(48, calculate_mul(parsed));
    }

    #[test]
    fn input_2() {
        let input = get_input(file!(), "input.txt");
        let parsed = get_muls(remove_donts(input));
        assert_eq!(84893551, calculate_mul(parsed));
    }
}
