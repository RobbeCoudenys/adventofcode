use std::{
    collections::HashMap,
    ops::{AddAssign, SubAssign},
};

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
    cube_coords: Vec<(usize, usize)>,
    ball_coords: Vec<(usize, usize)>,
    max_x: usize,
    max_y: usize,
}

impl From<Platform> for PlatformSol2 {
    fn from<'a>(value: Platform) -> Self {
        let max_x = value.items.get(0).unwrap().len() - 1;
        let max_y = value.items.len() - 1;
        let mut cube_coords = Vec::new();
        let mut ball_coords = Vec::new();
        for (y, row) in value.items.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                match value {
                    Space::Space => (),
                    Space::Cube => cube_coords.push((x, y)),
                    Space::Ball => ball_coords.push((x, y)),
                }
            }
        }

        Self {
            cube_coords,
            ball_coords,
            max_x,
            max_y,
        }
    }
}

impl PlatformSol2 {
    fn balls_and_cubes_chain(
        &self,
    ) -> std::iter::Chain<std::slice::Iter<'_, (usize, usize)>, std::slice::Iter<'_, (usize, usize)>>
    {
        self.ball_coords.iter().chain(self.cube_coords.iter())
    }
    fn north(&mut self) {
        self.ball_coords.sort_by(|a, b| a.1.cmp(&b.1));
        for index in 0..self.ball_coords.iter().count() {
            let space = self.ball_coords.get(index).unwrap();

            let relevant_space_y = self
                .balls_and_cubes_chain()
                .filter(|c| c.0 == space.0 && c.1 < space.1)
                .max_by_key(|c| c.1)
                .map(|c| c.1);

            let space = self.ball_coords.get_mut(index).unwrap();

            if let Some(space_above) = relevant_space_y {
                space.1 = space_above + 1;
            } else {
                space.1 = 0;
            }
        }
    }
    fn west(&mut self) {
        self.ball_coords.sort_by(|a, b| a.0.cmp(&b.0));
        for index in 0..self.ball_coords.iter().count() {
            let space = self.ball_coords.get(index).unwrap();

            let relevant_space_x = self
                .balls_and_cubes_chain()
                .filter(|c| c.1 == space.1 && c.0 < space.0)
                .max_by_key(|c| c.0)
                .map(|c| c.0);

            let space = self.ball_coords.get_mut(index).unwrap();

            if let Some(space_left) = relevant_space_x {
                space.0 = space_left + 1;
            } else {
                space.0 = 0;
            }
        }
    }
    fn south(&mut self) {
        self.ball_coords.sort_by(|a, b| a.1.cmp(&b.1).reverse());
        for index in 0..self.ball_coords.iter().count() {
            let space = self.ball_coords.get(index).unwrap();

            let relevant_space_y = self
                .balls_and_cubes_chain()
                .filter(|c| c.0 == space.0 && c.1 > space.1)
                .min_by_key(|c| c.1)
                .map(|c| c.1);

            let space = self.ball_coords.get_mut(index).unwrap();

            if let Some(space_below) = relevant_space_y {
                space.1 = space_below - 1;
            } else {
                space.1 = self.max_y;
            }
        }
    }
    fn east(&mut self) {
        self.ball_coords.sort_by(|a, b| a.0.cmp(&b.0).reverse());
        for index in 0..self.ball_coords.iter().count() {
            let space = self.ball_coords.get(index).unwrap();

            let relevant_space_x = self
                .balls_and_cubes_chain()
                .filter(|c| c.1 == space.1 && c.0 > space.0)
                .min_by_key(|c| c.0)
                .map(|c| c.0);

            let space = self.ball_coords.get_mut(index).unwrap();

            if let Some(space_right) = relevant_space_x {
                space.0 = space_right - 1;
            } else {
                space.0 = self.max_x;
            }
        }
    }

    fn cycle(&mut self) {
        let mut cycle_map: HashMap<Vec<(usize, usize)>, Vec<(usize, usize)>> = HashMap::new();
        let total_cycles = 1_000_000_000;

        let mut loop_size = 0;
        let mut cycle_count = 0;
        while cycle_count < total_cycles {
            // self.print();
            let cycle_start = &self.ball_coords;
            if loop_size == 0 {
                if let Some(next) = cycle_map.get(cycle_start) {
                    loop_size.add_assign(1);
                    println!(
                        "starting score: {}",
                        self.weigth_to_north_provide_list(&cycle_start)
                    );
                    // detect loop
                    let mut current = next;
                    while let Some(next) = cycle_map.get(current) {
                        loop_size.add_assign(1);
                        if next.eq(cycle_start) {
                            break;
                        }
                        current = next;
                    }
                    println!("Starting cycle count: {}", cycle_count);
                    println!("Loop size: {}", loop_size);
                    let remaining_cycles = (total_cycles - cycle_count) % loop_size;
                    println!("Remaining cycles: {}", remaining_cycles);
                    cycle_count = total_cycles - remaining_cycles;
                    println!("New cycle count: {}", cycle_count);
                    continue;
                }
            }
            let previous = self.ball_coords.to_vec();
            self.north();
            self.west();
            self.south();
            self.east();

            cycle_map.insert(previous, self.ball_coords.to_vec());
            cycle_count.add_assign(1);
        }
    }

    fn weigth_to_north_provide_list(&self, list: &Vec<(usize, usize)>) -> usize {
        let top_beam_weight = self.max_y + 1;
        list.iter().map(|b| top_beam_weight - b.1).sum()
    }

    fn weigth_to_north(&self) -> usize {
        self.weigth_to_north_provide_list(&self.ball_coords)
    }

    fn print(&mut self) {
        print!("\n\n");
        let mut balls_and_cubes = self
            .balls_and_cubes_chain()
            .map(|c| (c.0, c.1))
            .collect::<Vec<(usize, usize)>>();
        balls_and_cubes.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        for y in 0..self.max_y + 1 {
            for x in 0..self.max_x + 1 {
                let matching_coord = balls_and_cubes
                    .iter()
                    .filter(|c| c.0.eq(&x) && c.1.eq(&y))
                    .next();
                match matching_coord {
                    Some(coord) => {
                        if self.ball_coords.contains(coord) {
                            print!("0");
                        } else {
                            print!("#");
                        }
                    }
                    None => print!("."),
                }
            }
            print!("\n");
        }

        print!("Total value: {}\n", self.weigth_to_north());
    }

    fn set_balls(&mut self, test: &Vec<(usize, usize)>) {
        self.ball_coords = test.to_vec();
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

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input1.txt");
        assert_eq!(104671, solution_2(input));
    }
}
