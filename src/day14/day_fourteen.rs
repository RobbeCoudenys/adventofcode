use std::ops::{AddAssign, SubAssign};

use itertools::Itertools;

#[derive(PartialEq)]
enum Space {
    Space,
    Cube,
    Ball,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value.into() {
            'O' => Self::Ball,
            '#' => Self::Cube,
            '.' => Self::Space,
            should_not_happen => panic!("The char {} should not be present", should_not_happen),
        }
    }
}

struct Platform {
    items: Vec<Vec<Space>>,
}

impl From<String> for Platform {
    fn from(value: String) -> Self {
        let items = value
            .split('\n')
            .map(|s| s.chars().map(|c| Space::from(c)).collect::<Vec<Space>>())
            .collect::<Vec<Vec<Space>>>();
        Self { items }
    }
}

impl Platform {
    fn weigth_to_north(&self) -> usize {
        let top_beam_weight = self.items.len();
        let mut result = 0;
        for (y, row) in self.items.iter().enumerate() {
            for (x, space) in row.iter().enumerate() {
                match space {
                    Space::Ball => {
                        let mut empty_space_count = 0;
                        let mut current_y = y;
                        let mut should_continue = true;
                        while should_continue {
                            let current_space = self.items.get(current_y).unwrap().get(x).unwrap();
                            if let Space::Space = current_space {
                                empty_space_count.add_assign(1);
                            }
                            if current_y == 0 || current_space.eq(&Space::Cube) {
                                should_continue = false;
                            } else {
                                current_y.sub_assign(1);
                            }
                        }
                        result.add_assign(top_beam_weight - (y - empty_space_count));
                    }
                    _ => (),
                }
            }
        }
        result
    }
}

fn solution_1(input: String) -> usize {
    let platform = Platform::from(input);
    platform.weigth_to_north()
}

struct Coordinate {
    x: usize,
    y: usize,
    value: Space,
}

struct PlatformSol2 {
    cubes_and_balls: Vec<Coordinate>,
}

impl From<Platform> for PlatformSol2 {
    fn from<'a>(value: Platform) -> Self {
        let mut cubes_and_balls = value
            .items
            .into_iter()
            .enumerate()
            .flat_map(|(y, spaces)| {
                spaces
                    .into_iter()
                    .enumerate()
                    .map(move |(x, space)| Coordinate { x, y, value: space })
            })
            .filter(|coord| match coord.value {
                Space::Space => false,
                Space::Cube => true,
                Space::Ball => true,
            })
            .collect::<Vec<Coordinate>>();
        // cubes_and_balls.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
        Self { cubes_and_balls }
    }
}

impl PlatformSol2 {
    fn north(&mut self) {
        let old = self.cubes_and_balls.clone();
        let count = self
            .cubes_and_balls
            .iter()
            .filter(|sp| sp.value.eq(&Space::Ball))
            .count();
        for index in 0..count {
            let space = self.cubes_and_balls.get(index).unwrap();
            let relevant_space = self
                .cubes_and_balls
                .iter()
                .filter(|c| c.x == space.x && c.y < space.y)
                .sorted_by(|a, b| a.y.cmp(&b.y).reverse())
                .next();
            let mut space = self.cubes_and_balls.get_mut(index).unwrap();
            if let Some(space_above) = relevant_space {
                space.x.add_assign(1);
            } else {
                space.x = 0;
            }
        }
    }
    fn west(&mut self) {}
    fn south(&mut self) {}
    fn east(&mut self) {}
    fn cycle(&mut self) {
        for _ in 0..1000000000 {
            self.north();
            self.west();
            self.south();
            self.east();
        }
    }
}

fn solution_2(input: String) -> usize {
    let platform = Platform::from(input);
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");
        assert_eq!(136, solution_1(input));
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        assert_eq!(108889, solution_1(input));
    }

    #[test]
    fn example_2_test() {
        let input = get_input(file!(), "example1.txt");
        assert_eq!(64, solution_2(input));
    }
}
