use std::{
    collections::{HashMap, HashSet},
    iter::Map,
};

use itertools::Itertools;

type Coords = (i32, i32);

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
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

#[derive(Clone)]
struct Guard {
    coords: Coords,
    direction: Direction,
    // map of coords that have been visited by the guard in a certain direction
    // important to detect infite loops
    previous_positions: HashMap<Coords, HashSet<Direction>>,
}

impl Guard {
    // Walks the guard in a repetitive pattern until it goes outside the map
    // or enters an infinite loop
    fn walk(&mut self, map: &HashMap<Coords, char>) -> bool {
        let next_coords = self.direction.get_next_coords(self.coords);
        if let Some(next_char) = map.get(&next_coords) {
            if next_char == &'#' {
                self.direction = self.direction.rotate();
                return self.walk(map);
            }
        }
        self.coords = next_coords;
        if let Some(prev_directions) = self.previous_positions.get_mut(&self.coords) {
            if prev_directions.contains(&self.direction) {
                return true;
            }
            prev_directions.insert(self.direction);
        } else {
            self.previous_positions
                .insert(self.coords, [self.direction].iter().cloned().collect());
        }
        return false;
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
                previous_positions: HashMap::new(),
            };
        }
    }
    panic!("No guard found in map");
}

fn traverse_map(mut map: HashMap<Coords, char>, mut guard: Guard) -> (HashMap<Coords, char>, bool) {
    if let Some(curr_pos) = map.get_mut(&guard.coords) {
        *curr_pos = 'X';
    }
    let mut infinite_loop = false;
    loop {
        infinite_loop = guard.walk(&map);
        if infinite_loop {
            break;
        }
        if let Some(next_pos) = map.get_mut(&guard.coords) {
            if next_pos == &'.' {
                *next_pos = 'X';
            }
        } else {
            break;
        }
        // print_map(&map);
    }
    (map, infinite_loop)
}

fn is_infinit_loop_when_adding_obstacle_at(
    mut guard: Guard,
    mut map: HashMap<Coords, char>,
    obstacle: Coords,
) -> bool {
    map.insert(obstacle, '#');
    let guard = guard.clone();
    traverse_map(map, guard).1
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
    use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn example_1() {
        let input = get_input(file!(), "example.txt");
        let map = parse_input(input);
        let guard = find_guard(&map);
        let map = traverse_map(map, guard).0;
        assert_eq!(41, map.values().filter(|&c| *c == 'X').count());
    }

    #[test]
    fn input_1() {
        let input = get_input(file!(), "input.txt");
        let map = parse_input(input);
        let guard = find_guard(&map);
        let map = traverse_map(map, guard).0;
        assert_eq!(5129, map.values().filter(|&c| *c == 'X').count());
    }

    #[test]
    fn example_2() {
        let input = get_input(file!(), "example.txt");
        let map = parse_input(input);
        let guard = find_guard(&map);
        let map_with_x = traverse_map(map.clone(), guard.clone());
        let result = map_with_x
            .0
            .iter()
            .filter(|(coords, c)| *c == &'X' || *coords == &guard.coords)
            .map(|(coords, _)| *coords)
            .map(|obstacle| {
                if is_infinit_loop_when_adding_obstacle_at(guard.clone(), map.clone(), obstacle) {
                    1
                } else {
                    0
                }
            })
            .sum();
        assert_eq!(6, result);
    }

    #[test]
    fn input_2() {
        let input = get_input(file!(), "input.txt");
        let map = parse_input(input);
        let guard = find_guard(&map);
        let map_with_x = traverse_map(map.clone(), guard.clone());
        let result = map_with_x
            .0
            .par_iter()
            .filter(|(coords, c)| *c == &'X' || *coords == &guard.coords)
            .map(|(coords, _)| *coords)
            .map(|obstacle| {
                if is_infinit_loop_when_adding_obstacle_at(guard.clone(), map.clone(), obstacle) {
                    1
                } else {
                    0
                }
            })
            .sum();
        assert_eq!(1888, result);
    }
}
