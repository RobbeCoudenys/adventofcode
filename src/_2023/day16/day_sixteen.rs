use std::{
    collections::{HashMap, HashSet},
    ops::{AddAssign, ShlAssign, SubAssign},
    vec,
};

use itertools::concat;

#[derive(PartialEq, Eq, Clone)]
enum RayDirection {
    Left,
    Right,
    Up,
    Down,
}

struct Ray {
    coordinate: Coordinate,
    direction: RayDirection,
}

impl Ray {
    fn new(x: usize, y: usize, direction: RayDirection) -> Self {
        Self {
            coordinate: Coordinate::new(x, y),
            direction,
        }
    }

    fn next_coordinate(&mut self, max_x: usize, max_y: usize) -> bool {
        match self.direction {
            RayDirection::Left => {
                if self.coordinate.x == 0 {
                    return false;
                }
                self.coordinate.x.sub_assign(1);
            }
            RayDirection::Right => {
                if self.coordinate.x == max_x {
                    return false;
                }
                self.coordinate.x.add_assign(1);
            }
            RayDirection::Up => {
                if self.coordinate.y == 0 {
                    return false;
                }
                self.coordinate.y.sub_assign(1);
            }
            RayDirection::Down => {
                if self.coordinate.y == max_y {
                    return false;
                }
                self.coordinate.y.add_assign(1);
            }
        }
        true
    }

    fn change_direction(mut self, new_direction: RayDirection) -> Self {
        self.direction = new_direction;
        self
    }
}

enum Encounter {
    HorizontalSplitter(Vec<RayDirection>),
    VerticalSplitter(Vec<RayDirection>),
    LeftUpMirror(Vec<RayDirection>),
    LeftDownMirror(Vec<RayDirection>),
}

impl Encounter {
    fn get_next_rays(&mut self, ray: Ray) -> Vec<Ray> {
        if self.contains_direction(&ray.direction) {
            return Vec::new();
        }
        self.add_direction(ray.direction.clone());
        let x = ray.coordinate.x;
        let y = ray.coordinate.y;
        match self {
            Encounter::HorizontalSplitter(_) => match ray.direction {
                RayDirection::Left | RayDirection::Right => vec![ray],
                RayDirection::Up | RayDirection::Down => vec![
                    Ray::new(x, y, RayDirection::Left),
                    Ray::new(x, y, RayDirection::Right),
                ],
            },
            Encounter::VerticalSplitter(_) => match ray.direction {
                RayDirection::Left | RayDirection::Right => vec![
                    Ray::new(x, y, RayDirection::Up),
                    Ray::new(x, y, RayDirection::Down),
                ],
                RayDirection::Up | RayDirection::Down => vec![ray],
            },
            Encounter::LeftUpMirror(_) => match ray.direction {
                RayDirection::Left => vec![ray.change_direction(RayDirection::Down)],
                RayDirection::Right => vec![ray.change_direction(RayDirection::Up)],
                RayDirection::Up => vec![ray.change_direction(RayDirection::Right)],
                RayDirection::Down => vec![ray.change_direction(RayDirection::Left)],
            },
            Encounter::LeftDownMirror(_) => match ray.direction {
                RayDirection::Left => vec![ray.change_direction(RayDirection::Up)],
                RayDirection::Right => vec![ray.change_direction(RayDirection::Down)],
                RayDirection::Up => vec![ray.change_direction(RayDirection::Left)],
                RayDirection::Down => vec![ray.change_direction(RayDirection::Right)],
            },
        }
    }

    fn contains_direction(&self, direction: &RayDirection) -> bool {
        match self {
            Encounter::HorizontalSplitter(d) => d.contains(direction),
            Encounter::VerticalSplitter(d) => d.contains(direction),
            Encounter::LeftUpMirror(d) => d.contains(direction),
            Encounter::LeftDownMirror(d) => d.contains(direction),
        }
    }

    fn add_direction(&mut self, direction: RayDirection) {
        match self {
            Encounter::HorizontalSplitter(d) => d.push(direction),
            Encounter::VerticalSplitter(d) => d.push(direction),
            Encounter::LeftUpMirror(d) => d.push(direction),
            Encounter::LeftDownMirror(d) => d.push(direction),
        }
    }

    fn reset_directions(&mut self) {
        match self {
            Encounter::HorizontalSplitter(d) => d.clear(),
            Encounter::VerticalSplitter(d) => d.clear(),
            Encounter::LeftUpMirror(d) => d.clear(),
            Encounter::LeftDownMirror(d) => d.clear(),
        }
    }
}

impl From<char> for Encounter {
    fn from(value: char) -> Self {
        match value.into() {
            '|' => Self::VerticalSplitter(Vec::new()),
            '-' => Self::HorizontalSplitter(Vec::new()),
            '/' => Self::LeftUpMirror(Vec::new()),
            '\\' => Self::LeftDownMirror(Vec::new()),
            _ => panic!("This character should not be available {}", value),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}
impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Contraption {
    map: HashMap<Coordinate, Encounter>,
    rays: Vec<Ray>,
    energized_coordinates: HashSet<Coordinate>,
    max_x: usize,
    max_y: usize,
}

impl From<String> for Contraption {
    fn from(value: String) -> Self {
        let max_x = value.split('\n').next().unwrap().chars().count() - 1;
        let max_y = value.split('\n').count() - 1;
        Self {
            max_x,
            max_y,
            rays: Vec::new(),
            energized_coordinates: HashSet::new(),
            map: value
                .split('\n')
                .enumerate()
                .flat_map(|(y, row)| {
                    row.chars()
                        .enumerate()
                        .into_iter()
                        .filter(|(_, c)| c.ne(&'.'))
                        .map(move |(x, c)| (Coordinate::new(x, y), Encounter::from(c)))
                })
                .collect::<HashMap<Coordinate, Encounter>>(),
        }
    }
}

impl Contraption {
    fn print(&self) {
        for y in 0..self.max_y + 1 {
            for x in 0..self.max_x + 1 {
                let coordinate = Coordinate::new(x, y);
                if self.energized_coordinates.contains(&coordinate) {
                    print!("#");
                } else {
                    match self.map.get(&coordinate) {
                        Some(encounter) => match encounter {
                            Encounter::HorizontalSplitter(_) => print!("-"),
                            Encounter::VerticalSplitter(_) => print!("|"),
                            Encounter::LeftUpMirror(_) => print!("/"),
                            Encounter::LeftDownMirror(_) => print!("\\"),
                        },
                        None => {
                            print!(".");
                        }
                    }
                }
            }
            print!("\n");
        }
        print!("\n\n\n");
    }

    fn initialize_rays(&mut self, rays: Vec<Ray>) {
        self.energized_coordinates = HashSet::new();
        for encounter in self.map.values_mut() {
            encounter.reset_directions();
        }
        self.rays = rays;
    }

    fn shoot_rays_and_count_energized_spaces(&mut self) -> usize {
        while !self.rays.is_empty() {
            // self.print();
            let mut new_rays = Vec::new();
            for mut ray in self.rays.drain(..) {
                self.energized_coordinates
                    .insert(Coordinate::new(ray.coordinate.x, ray.coordinate.y));
                if ray.next_coordinate(self.max_x, self.max_y) {
                    match self.map.get_mut(&ray.coordinate) {
                        Some(encounter) => new_rays.extend(encounter.get_next_rays(ray)),
                        None => new_rays.push(ray),
                    }
                }
            }

            self.rays = new_rays;
        }
        self.energized_coordinates.len()
    }
}

fn solution_1(input: String) -> usize {
    let mut contraption = Contraption::from(input);
    contraption.rays.push(Ray::new(0, 0, RayDirection::Right));
    contraption.shoot_rays_and_count_energized_spaces()
}

fn solution_2(input: String) -> usize {
    let mut contraption = Contraption::from(input);
    let mut max_energized = 0;
    for x in 0..contraption.max_x + 1 {
        // top to bottom
        contraption.initialize_rays(vec![Ray::new(x, 0, RayDirection::Down)]);
        let energized = contraption.shoot_rays_and_count_energized_spaces();
        if energized > max_energized {
            max_energized = energized;
        }

        // bottom to top
        contraption.initialize_rays(vec![Ray::new(x, contraption.max_y, RayDirection::Up)]);
        let energized = contraption.shoot_rays_and_count_energized_spaces();
        if energized > max_energized {
            max_energized = energized;
        }
    }
    for y in 0..contraption.max_y + 1 {
        // left to right
        contraption.initialize_rays(vec![Ray::new(0, y, RayDirection::Right)]);
        let energized = contraption.shoot_rays_and_count_energized_spaces();
        if energized > max_energized {
            max_energized = energized;
        }

        // right to left
        contraption.initialize_rays(vec![Ray::new(contraption.max_x, y, RayDirection::Left)]);
        let energized = contraption.shoot_rays_and_count_energized_spaces();
        if energized > max_energized {
            max_energized = energized;
        }
    }
    max_energized
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");
        assert_eq!(46, solution_1(input));
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        assert_eq!(7046, solution_1(input));
    }

    #[test]
    fn example_2_test() {
        let input = get_input(file!(), "example1.txt");
        assert_eq!(51, solution_2(input));
    }

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input1.txt");
        assert_eq!(7313, solution_2(input));
    }

    #[test]
    fn example_2_test_individual_solution() {
        let input = get_input(file!(), "example1.txt");
        let mut contraption = Contraption::from(input);
        contraption.initialize_rays(vec![Ray::new(3, 0, RayDirection::Down)]);
        let result = contraption.shoot_rays_and_count_energized_spaces();
        assert_eq!(51, result);
    }
}
