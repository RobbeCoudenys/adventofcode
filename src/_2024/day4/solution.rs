use std::collections::HashMap;

type Coords = (i32, i32);
type Grid = HashMap<Coords, char>;

fn parse_input(input: String) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect()
}

// Part 1 is finding all XMAS occurences
fn count_xmas_part1(input: &Grid) -> u32 {
    let mut count = 0;
    input.iter().for_each(|(x_y, c)| {
        if c == &'X' && is_part_of_xmas(input, x_y) {
            count += 1;
        }
    });
    count
}

fn nr_of_xmasses_from_x_position(grid: &Grid, x_position: &Coords) -> u32 {
    let word = "XMAS";
    let mut count = 0;
    let directions = vec![
        Direction::Horizontal(true),
        Direction::Horizontal(false),
        Direction::Vertical(true),
        Direction::Vertical(false),
        Direction::Diagonal_UP(true),
        Direction::Diagonal_UP(false),
        Direction::Diagonal_DOWN(true),
        Direction::Diagonal_DOWN(false),
    ];
    for direction in directions {
        if is_xmas_in_direction(grid, x_position, &direction) {
            count += 1;
        }
    }
    count
}

enum Direction {
    Horizontal(bool),
    Vertical(bool),
    Diagonal_UP(bool),
    Diagonal_DOWN(bool),
}

impl Direction {
    fn calculate_next_position(&self, curr_position: Coords) -> Coords {
        match self {
            Direction::Horizontal(true) => {
                let next_x = curr_position.0 + 1;
                let next_y = curr_position.1;
                (next_x, next_y)
            }
            Direction::Horizontal(false) => {
                let next_x = curr_position.0 - 1;
                let next_y = curr_position.1;
                (next_x, next_y)
            }
            Direction::Vertical(true) => {
                let next_x = curr_position.0;
                let next_y = curr_position.1 + 1;
                (next_x, next_y)
            }
            Direction::Vertical(false) => {
                let next_x = curr_position.0;
                let next_y = curr_position.1 - 1;
                (next_x, next_y)
            }
            Direction::Diagonal_UP(true) => {
                let next_x = curr_position.0 + 1;
                let next_y = curr_position.1 + 1;
                (next_x, next_y)
            }
            Direction::Diagonal_UP(false) => {
                let next_x = curr_position.0 - 1;
                let next_y = curr_position.1 - 1;
                (next_x, next_y)
            }
            Direction::Diagonal_DOWN(true) => {
                let next_x = curr_position.0 + 1;
                let next_y = curr_position.1 - 1;
                (next_x, next_y)
            }
            Direction::Diagonal_DOWN(false) => {
                let next_x = curr_position.0 - 1;
                let next_y = curr_position.1 + 1;
                (next_x, next_y)
            }
        }
    }
}

fn is_xmas_in_direction(grid: &Grid, x_position: &Coords, direction: &Direction) -> bool {
    let mut curr_position = x_position.clone();
    for c in "MAS".chars() {
        curr_position = direction.calculate_next_position(curr_position);
        if let Some(next_char) = grid.get(&curr_position) {
            if next_char != &c {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

// Part 2 is finding MAS in a cross
// Solution: Find all A's and check if there is a M and S in the same direction
fn count_xmas_part2(input: &Grid) -> u32 {
    let mut count = 0;
    input.iter().for_each(|(x_y, c)| {
        if c == &'A' {
            if is_part_of_xmas(input, x_y) {
                count += 1;
            }
        }
    });
    count
}

fn is_part_of_xmas(grid: &Grid, a_coord: &Coords) -> bool {
    has_char_on_same_side(grid, a_coord, 'M') && has_char_on_same_side(grid, a_coord, 'S')
}

fn has_char_on_same_side(grid: &Grid, a_coord: &Coords, char: char) -> bool {
    let coord_left_up = (a_coord.0 - 1, a_coord.1 - 1);
    let coord_right_up = (a_coord.0 + 1, a_coord.1 - 1);
    let coord_right_down = (a_coord.0 + 1, a_coord.1 + 1);
    let coord_left_down = (a_coord.0 - 1, a_coord.1 + 1);
    (grid.get(&coord_left_up) == Some(&char) && grid.get(&coord_right_up) == Some(&char))
        || (grid.get(&coord_right_up) == Some(&char) && grid.get(&coord_right_down) == Some(&char))
        || (grid.get(&coord_right_down) == Some(&char) && grid.get(&coord_left_down) == Some(&char))
        || (grid.get(&coord_left_down) == Some(&char) && grid.get(&coord_left_up) == Some(&char))
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn example_1_count_directions() {
        let input = get_input(file!(), "example_all_directions.txt");
        let parsed = parse_input(input);
        assert_eq!(count_xmas_part1(&parsed), 8);
    }

    #[test]
    fn example_1_test_total() {
        assert_eq!(count_xmas_part1(&parse_input("XMAS".to_string())), 1);
        assert_eq!(count_xmas_part1(&parse_input("SAMX".to_string())), 1);
        assert_eq!(
            count_xmas_part1(&parse_input("...X\n...M\n...A\n...S".to_string())),
            1
        );
        assert_eq!(
            count_xmas_part1(&parse_input("...S\n...A\n...M\n...X".to_string())),
            1
        );
        assert_eq!(
            count_xmas_part1(&parse_input("X...\n.M..\n..A.\n...S".to_string())),
            1
        );
        assert_eq!(
            count_xmas_part1(&parse_input("S...\n.A..\n..M.\n...X".to_string())),
            1
        );
    }

    #[test]
    fn example_1() {
        let input = get_input(file!(), "example.txt");
        let parsed = parse_input(input);
        assert_eq!(18, count_xmas_part1(&parsed));
    }

    #[test]
    fn input_1() {
        let input = get_input(file!(), "input.txt");
        let parsed = parse_input(input);
        assert_eq!(2662, count_xmas_part1(&parsed));
    }

    #[test]
    fn example_2() {
        let input = get_input(file!(), "example.txt");
        let parsed = parse_input(input);
        assert_eq!(9, count_xmas_part2(&parsed));
    }

    #[test]
    fn input_2() {
        let input = get_input(file!(), "input.txt");
        let parsed = parse_input(input);
        assert_eq!(9, count_xmas_part2(&parsed));
    }
}
