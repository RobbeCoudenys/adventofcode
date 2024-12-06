use std::{
    collections::{HashMap, HashSet},
    iter::Map,
};

use itertools::Itertools;

type Coords = (i32, i32);

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_next_coords(&self, coords: Coords) -> Coords {
        match self {
            Direction::North => (coords.0, coords.1 - 1),
            Direction::East => (coords.0 + 1, coords.1),
            Direction::South => (coords.0, coords.1 + 1),
            Direction::West => (coords.0 - 1, coords.1),
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

struct Guard {
    coords: Coords,
    direction: Direction,
}

impl Guard {
    fn walk(&mut self, map: &HashMap<Coords, char>) {
        let next_coords = self.direction.get_next_coords(self.coords);
        if let Some(next_char) = map.get(&next_coords) {
            if next_char == &'#' {
                self.direction = self.direction.rotate();
                return self.walk(map);
            }
        }
        self.coords = next_coords;
    }
}

fn parse_input(input: String) -> HashMap<Coords, char> {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), c);
        }
    }
    map
}

fn find_guard(map: &HashMap<Coords, char>) -> Guard {
    for (coords, c) in map {
        if *c == 'v' || *c == '^' || *c == '<' || *c == '>' {
            return Guard {
                coords: *coords,
                direction: match c {
                    '>' => Direction::East,
                    '<' => Direction::West,
                    '^' => Direction::North,
                    'v' => Direction::South,
                    _ => panic!("Invalid direction for char: {}", c),
                },
            };
        }
    }
    panic!("No guard found in map");
}

fn traverse_map(mut map: HashMap<Coords, char>, mut guard: Guard) -> HashMap<Coords, char> {
    if let Some(curr_pos) = map.get_mut(&guard.coords) {
        *curr_pos = 'X';
    }
    loop {
        guard.walk(&map);
        if let Some(next_pos) = map.get_mut(&guard.coords) {
            if next_pos == &'.' {
                *next_pos = 'X';
            }
        } else {
            break;
        }
        // print_map(&map);
    }
    map
}

fn print_map(map: &HashMap<Coords, char>) {
    println!();
    println!();
    map.keys()
        .sorted_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)))
        .for_each(|coords| {
            if coords.0 == 0 {
                println!();
            }
            print!("{}", map.get(coords).unwrap());
        });
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn example_1() {
        let input = get_input(file!(), "example.txt");
        let map = parse_input(input);
        let guard = find_guard(&map);
        let map = traverse_map(map, guard);
        assert_eq!(41, map.values().filter(|&c| *c == 'X').count());
    }

    #[test]
    fn input_1() {
        let input = get_input(file!(), "input.txt");
        let map = parse_input(input);
        let guard = find_guard(&map);
        let map = traverse_map(map, guard);
        assert_eq!(5129, map.values().filter(|&c| *c == 'X').count());
    }

    #[test]
    fn example_2() {
        let input = get_input(file!(), "example.txt");
        let map = parse_input(input);
        let guard = find_guard(&map);
        let map_with_x = traverse_map(map.clone(), guard);
        let guard_original_path = map_with_x
            .iter()
            .filter(|(_, c)| **c == 'X')
            .map(|(coords, _)| *coords)
            .collect::<Vec<Coords>>();
    }

    #[test]
    fn input_2() {
        let input = get_input(file!(), "input.txt");
    }
}
