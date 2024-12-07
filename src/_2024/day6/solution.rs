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
    #[inline]
    fn get_next_coords(&self, coords: Coords) -> Coords {
        match self {
            Direction::North => (coords.0, coords.1 - 1),
            Direction::East => (coords.0 + 1, coords.1),
            Direction::South => (coords.0, coords.1 + 1),
            Direction::West => (coords.0 - 1, coords.1),
        }
    }

    #[inline]
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
    fn walk(&mut self, map: &HashMap<Coords, char>, obstacle: Option<&Coords>) -> bool {
        let next_coords = self.direction.get_next_coords(self.coords);
        if let Some(next_char) = map.get(&next_coords) {
            if next_char == &'#' || Some(&next_coords) == obstacle {
                self.direction = self.direction.rotate();
                return self.walk(map, obstacle);
            }
        }
        self.coords = next_coords;
        if Option::None == map.get(&next_coords) {
            return false;
        }
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
            let mut previous_positions = HashMap::new();
            previous_positions.insert(*coords, [Direction::North].iter().cloned().collect());
            return Guard {
                coords: *coords,
                direction: match c {
                    '>' => Direction::East,
                    '<' => Direction::West,
                    '^' => Direction::North,
                    'v' => Direction::South,
                    _ => panic!("Invalid direction for char: {}", c),
                },
                previous_positions,
            };
        }
    }
    panic!("No guard found in map");
}

fn traverse_map(
    map: &HashMap<Coords, char>,
    guard: &Guard,
    obstacle: Option<&Coords>,
) -> (Guard, bool) {
    let mut guard = guard.clone();
    let mut infinite_loop = false;
    loop {
        infinite_loop = guard.walk(&map, obstacle);
        if infinite_loop {
            break;
        }
        if Option::None == map.get(&guard.coords) {
            break;
        }
        // print_map(&map);
    }
    (guard, infinite_loop)
}

fn is_infinit_loop_when_adding_obstacle_at(
    guard: &Guard,
    map: &HashMap<Coords, char>,
    obstacle: &Coords,
) -> bool {
    traverse_map(map, guard, Option::Some(obstacle)).1
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
        let final_guard = traverse_map(&map, &guard, Option::None).0;
        assert_eq!(41, final_guard.previous_positions.len());
    }

    #[test]
    fn input_1() {
        let input = get_input(file!(), "input.txt");
        let map = parse_input(input);
        let guard = find_guard(&map);
        let final_guard = traverse_map(&map, &guard, Option::None).0;
        assert_eq!(5129, final_guard.previous_positions.len());
    }

    #[test]
    fn example_2() {
        let input = get_input(file!(), "example.txt");
        let map = parse_input(input);
        let guard = find_guard(&map);
        let map_with_x = traverse_map(&map, &guard, Option::None);
        let result = map_with_x
            .0
            .previous_positions
            .keys()
            .map(|obstacle| {
                if is_infinit_loop_when_adding_obstacle_at(&guard, &map, obstacle) {
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
        let map_with_x = traverse_map(&map, &guard, Option::None);
        let obstacles: Vec<&Coords> = map_with_x.0.previous_positions.keys().collect();
        let result = obstacles
            .par_iter()
            .map(|obstacle| {
                if is_infinit_loop_when_adding_obstacle_at(&guard, &map, *obstacle) {
                    1
                } else {
                    0
                }
            })
            .sum();
        assert_eq!(1888, result);
    }
}
