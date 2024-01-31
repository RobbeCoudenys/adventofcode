use std::{
    collections::{HashMap, HashSet},
    ops::{AddAssign, SubAssign},
};

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
    max_x: usize,
    max_y: usize,
}

impl From<Platform> for PlatformSol2 {
    fn from<'a>(value: Platform) -> Self {
        let max_x = value.items.get(0).unwrap().len() - 1;
        let max_y = value.items.len() - 1;
        let cubes_and_balls = value
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
        Self {
            cubes_and_balls,
            max_x,
            max_y,
        }
    }
}

impl PlatformSol2 {
    fn north(&mut self) {
        self.cubes_and_balls.sort_by(|a, b| a.y.cmp(&b.y));
        for index in 0..self.cubes_and_balls.iter().count() {
            let space = self.cubes_and_balls.get(index).unwrap();
            match space.value {
                Space::Space => (),
                Space::Cube => (),
                Space::Ball => {
                    let relevant_space_y = self
                        .cubes_and_balls
                        .iter()
                        .filter(|c| c.x == space.x && c.y < space.y)
                        .max_by_key(|c| c.y)
                        .map(|c| c.y);

                    let space = self.cubes_and_balls.get_mut(index).unwrap();

                    if let Some(space_above) = relevant_space_y {
                        space.y = space_above + 1;
                    } else {
                        space.y = 0;
                    }
                }
            }
        }
    }
    fn west(&mut self) {
        self.cubes_and_balls.sort_by(|a, b| a.x.cmp(&b.x));
        for index in 0..self.cubes_and_balls.iter().count() {
            let space = self.cubes_and_balls.get(index).unwrap();
            match space.value {
                Space::Space => (),
                Space::Cube => (),
                Space::Ball => {
                    let relevant_space_x = self
                        .cubes_and_balls
                        .iter()
                        .filter(|c| c.y == space.y && c.x < space.x)
                        .max_by_key(|c| c.x)
                        .map(|c| c.x);

                    let space = self.cubes_and_balls.get_mut(index).unwrap();

                    if let Some(space_left) = relevant_space_x {
                        space.x = space_left + 1;
                    } else {
                        space.x = 0;
                    }
                }
            }
        }
    }
    fn south(&mut self) {
        self.cubes_and_balls.sort_by(|a, b| a.y.cmp(&b.y).reverse());
        for index in 0..self.cubes_and_balls.iter().count() {
            let space = self.cubes_and_balls.get(index).unwrap();
            match space.value {
                Space::Space => (),
                Space::Cube => (),
                Space::Ball => {
                    let relevant_space_y = self
                        .cubes_and_balls
                        .iter()
                        .filter(|c| c.x == space.x && c.y > space.y)
                        .min_by_key(|c| c.y)
                        .map(|c| c.y);

                    let space = self.cubes_and_balls.get_mut(index).unwrap();

                    if let Some(space_below) = relevant_space_y {
                        space.y = space_below - 1;
                    } else {
                        space.y = self.max_y;
                    }
                }
            }
        }
    }
    fn east(&mut self) {
        self.cubes_and_balls.sort_by(|a, b| a.x.cmp(&b.x).reverse());
        for index in 0..self.cubes_and_balls.iter().count() {
            let space = self.cubes_and_balls.get(index).unwrap();
            match space.value {
                Space::Space => (),
                Space::Cube => (),
                Space::Ball => {
                    let relevant_space_x = self
                        .cubes_and_balls
                        .iter()
                        .filter(|c| c.y == space.y && c.x > space.x)
                        .min_by_key(|c| c.x)
                        .map(|c| c.x);

                    let space = self.cubes_and_balls.get_mut(index).unwrap();

                    if let Some(space_right) = relevant_space_x {
                        space.x = space_right - 1;
                    } else {
                        space.x = self.max_x;
                    }
                }
            }
        }
    }

    fn get_simple_representation(&mut self) -> Vec<(usize, usize)> {
        self.cubes_and_balls
            .sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
        self.cubes_and_balls
            .iter()
            .map(|c| (c.x, c.y))
            .collect::<Vec<(usize, usize)>>()
    }

    fn cycle(&mut self) {
        let mut cycle_map: HashMap<Vec<(usize, usize)>, Vec<(usize, usize)>> = HashMap::new();

        let mut current = self.get_simple_representation();
        for _ in 0..1000000000 {
            let mut current = self.get_simple_representation();
            if let Some(test) = cycle_map.get(&current) {
                self.set_balls(test);
                continue;
            }
            self.north();
            self.west();
            self.south();
            self.east();

            let new = self.get_simple_representation();
            cycle_map.insert(current, new);

            // self.print();
        }
    }

    fn weigth_to_north(&self) -> usize {
        let top_beam_weight = self.max_y + 1;
        let mut result = 0;
        for space in self.cubes_and_balls.iter() {
            match space.value {
                Space::Ball => {
                    result.add_assign(top_beam_weight - space.y);
                }
                _ => (),
            }
        }
        result
    }

    fn print(&mut self) {
        print!("\n\n\n");
        self.cubes_and_balls
            .sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        for y in 0..self.max_y + 1 {
            for x in 0..self.max_x + 1 {
                let matching_coord = self
                    .cubes_and_balls
                    .iter()
                    .filter(|c| c.x.eq(&x) && c.y.eq(&y))
                    .next();
                match matching_coord {
                    Some(coord) => match coord.value {
                        Space::Space => print!("."),
                        Space::Cube => print!("#"),
                        Space::Ball => print!("O"),
                    },
                    None => print!("."),
                }
            }
            print!("\n");
        }
    }

    fn set_balls(&mut self, test: &Vec<(usize, usize)>) {
        for (index, coord) in self
            .cubes_and_balls
            .iter_mut()
            .filter(|c| c.value.eq(&Space::Ball))
            .enumerate()
        {
            let new_coord = test.get(index).unwrap();
            coord.x = new_coord.0;
            coord.y = new_coord.1;
        }
    }
}

fn solution_2(input: String) -> usize {
    let platform = Platform::from(input);
    let mut platform2 = PlatformSol2::from(platform);
    // platform2.print();
    platform2.cycle();
    platform2.weigth_to_north()
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
    fn example_1_test_use_sol_2() {
        let input = get_input(file!(), "example1.txt");
        let platform = Platform::from(input);
        let mut platform2 = PlatformSol2::from(platform);
        platform2.north();
        assert_eq!(136, platform2.weigth_to_north());
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        assert_eq!(108889, solution_1(input));
    }
    #[test]
    fn solution_1_test_use_sol_2() {
        let input = get_input(file!(), "input1.txt");
        let platform = Platform::from(input);
        let mut platform2 = PlatformSol2::from(platform);
        platform2.north();
        assert_eq!(108889, platform2.weigth_to_north());
    }

    #[test]
    fn example_2_test() {
        let input = get_input(file!(), "example1.txt");
        assert_eq!(64, solution_2(input));
    }
}
