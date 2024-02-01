use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value.into() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => todo!(),
        }
    }
}

type LookupString = [char; 3];

type LookupMap = HashMap<LookupString, LookupValue>;

struct LookupMapWrapper {
    value: LookupMap,
}

impl From<&str> for LookupMapWrapper {
    fn from(value: &str) -> Self {
        let mut lookup_map = HashMap::new();
        for row in value.split('\n') {
            let mut split = row.split(" = ");
            let key: LookupString = split
                .next()
                .unwrap()
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let left_or_right = split.next().unwrap();
            let len = &left_or_right.len() - 1;
            let mut lalala = left_or_right[1..len].split(", ");
            let left: LookupString = lalala
                .next()
                .unwrap()
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let right: LookupString = lalala
                .next()
                .unwrap()
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            lookup_map.insert(key, LookupValue { left, right });
        }
        Self { value: lookup_map }
    }
}

struct LookupValue {
    left: LookupString,
    right: LookupString,
}

struct InstructionsWrapper {
    value: Vec<Direction>,
}

impl From<&str> for InstructionsWrapper {
    fn from(value: &str) -> Self {
        let mut instructions = Vec::new();
        for instruction in value.chars() {
            instructions.push(Direction::from(instruction));
        }
        Self {
            value: instructions,
        }
    }
}

struct Game {
    instructions: InstructionsWrapper,
    lookup_map: LookupMapWrapper,
}

impl Game {
    fn get_next(&self, key: LookupString, direction: &Direction) -> LookupString {
        match direction {
            Direction::Left => self.lookup_map.value.get(&key).unwrap().left,
            Direction::Right => self.lookup_map.value.get(&key).unwrap().right,
        }
    }

    fn get_starting_points(&self, start: char) -> HashSet<LookupString> {
        let mut starting_points = HashSet::new();
        for key in self.lookup_map.value.keys() {
            if key.last().unwrap().eq(&start) {
                starting_points.insert(*key);
            }
        }
        starting_points
    }
}

impl From<String> for Game {
    fn from(value: String) -> Self {
        let mut values = value.split("\n\n");
        let instructions = InstructionsWrapper::from(values.next().unwrap());
        let lookup_map = LookupMapWrapper::from(values.next().unwrap());
        Self {
            instructions,
            lookup_map,
        }
    }
}

fn solution_1(game: Game, start: LookupString, end: LookupString) -> usize {
    let mut current_val = start;
    let mut count = 0;
    let mut instruction_index = 0;
    let last_instruction_index = game.instructions.value.len() - 1;
    while current_val.ne(&end) {
        if instruction_index > last_instruction_index {
            instruction_index = 0;
        }
        count += 1;
        current_val = game.get_next(
            current_val,
            game.instructions.value.get(instruction_index).unwrap(),
        );
        instruction_index += 1;
    }
    count
}

fn solution_2(game: Game, start: char, end: char) -> usize {
    let mut current_keys = game.get_starting_points(start);
    let mut instruction_index = 0;
    let last_instruction_index = game.instructions.value.len() - 1;
    let mut test: HashMap<LookupString, usize> = HashMap::new();
    for key in current_keys {
        test.insert(key, 0);

        let mut current_val = key;
        let mut instruction_index = 0;
        while current_val.last().unwrap().ne(&end) {
            if instruction_index > last_instruction_index {
                instruction_index = 0;
            }
            test.entry(key).and_modify(|v| *v += 1);
            current_val = game.get_next(
                current_val,
                game.instructions.value.get(instruction_index).unwrap(),
            );
            instruction_index += 1;
        }
    }

    least_common_multiple(
        test.values()
            .into_iter()
            .map(|v| *v)
            .collect::<Vec<usize>>(),
    )
}

fn prime_factorization(mut num: usize) -> HashMap<usize, usize> {
    let mut factors = HashMap::new();
    let mut divisor = 2;

    while num > 1 {
        while num % divisor == 0 {
            *factors.entry(divisor).or_insert(0) += 1;
            num /= divisor;
        }

        divisor += 1;
    }

    factors
}

// Function to find the least common multiple
fn least_common_multiple(numbers: Vec<usize>) -> usize {
    // Collect prime factorizations of all numbers
    let mut all_factors = Vec::new();
    for &num in &numbers {
        all_factors.push(prime_factorization(num));
    }

    // Compute common prime factors
    let mut common_factors = HashMap::new();
    for factors in all_factors.iter() {
        for (&factor, &count) in factors.iter() {
            *common_factors.entry(factor).or_insert(usize::MIN) = common_factors
                .get(&factor)
                .map_or(count, |&max_count| max_count.max(count));
        }
    }

    // Multiply common prime factors to find the LCM
    let mut lcm = 1;
    for (&factor, &count) in common_factors.iter() {
        lcm *= factor.pow(count as u32);
    }

    lcm
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn test_parser() {
        let instructions_str = "LLR";
        let instructions = InstructionsWrapper::from(instructions_str);
        assert_eq!(&Direction::Left, instructions.value.get(0).unwrap());
        assert_eq!(&Direction::Left, instructions.value.get(1).unwrap());
        assert_eq!(&Direction::Right, instructions.value.get(2).unwrap());

        let row = "BBB = (AAA, ZZZ)";
        let lookup_map = LookupMapWrapper::from(row);
        assert!(lookup_map.value.contains_key(&['B', 'B', 'B']));
        let lookup_value = lookup_map.value.values().next().unwrap();
        assert_eq!(['A', 'A', 'A'], lookup_value.left);
        assert_eq!(['Z', 'Z', 'Z'], lookup_value.right);
    }

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");

        let game = Game::from(input);
        assert_eq!(6, solution_1(game, ['A', 'A', 'A'], ['Z', 'Z', 'Z']));
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");

        let game = Game::from(input);
        assert_eq!(19667, solution_1(game, ['A', 'A', 'A'], ['Z', 'Z', 'Z']));
    }

    #[test]
    fn example_2_test() {
        let input = get_input(file!(), "example2.txt");
        let game = Game::from(input);
        assert_eq!(6, solution_2(game, 'A', 'Z'));
    }

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input1.txt");
        let game = Game::from(input);
        assert_eq!(19185263738117, solution_2(game, 'A', 'Z'));
    }
}
