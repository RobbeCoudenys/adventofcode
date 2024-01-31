use std::ops::AddAssign;

#[derive(Hash, PartialEq, Eq)]
struct Galaxy {
    id: usize,
    x: usize,
    y: usize,
}

impl Galaxy {
    fn calculate_distance(&self, other_galaxy: &Galaxy) -> usize {
        self.x.abs_diff(other_galaxy.x) + self.y.abs_diff(other_galaxy.y)
    }
}

struct MilkyWay {
    galaxies: Vec<Galaxy>,
    is_expanded: bool,
}

impl From<Vec<String>> for MilkyWay {
    fn from(rows: Vec<String>) -> Self {
        let mut milky_way = Vec::new();
        let mut id = 1;
        for (y, row) in rows.into_iter().enumerate() {
            for (x, character) in row.chars().enumerate() {
                match character.into() {
                    '#' => {
                        milky_way.push(Galaxy { id, x, y });
                        id.add_assign(1);
                    }
                    _ => (),
                }
            }
        }
        Self {
            galaxies: milky_way,
            is_expanded: false,
        }
    }
}

impl MilkyWay {
    fn get_max_x(&self) -> usize {
        self.galaxies.iter().map(|g| g.x).max().unwrap()
    }

    fn get_max_y(&self) -> usize {
        self.galaxies.iter().map(|g| g.y).max().unwrap()
    }

    fn get_empty_x(&self) -> Vec<usize> {
        let mut empty_x = Vec::new();
        for x in 0..self.get_max_x() {
            if !self
                .galaxies
                .iter()
                .map(|g| g.x)
                .collect::<Vec<usize>>()
                .contains(&x)
            {
                empty_x.push(x);
            }
        }
        empty_x
    }

    fn get_empty_y(&self) -> Vec<usize> {
        let mut empty_y = Vec::new();
        for y in 0..self.get_max_y() {
            if !self
                .galaxies
                .iter()
                .map(|g| g.y)
                .collect::<Vec<usize>>()
                .contains(&y)
            {
                empty_y.push(y);
            }
        }
        empty_y
    }

    fn expand_milky_way(&mut self, expansion_rate: usize) {
        if self.is_expanded {
            return;
        }
        self.galaxies.sort_by(|g1, g2| g1.y.cmp(&g2.y));
        let mut empty_x = self.get_empty_x();
        empty_x.reverse();
        for x in empty_x {
            for galaxy in self.galaxies.iter_mut().filter(|g| g.x > x) {
                galaxy.x.add_assign(expansion_rate - 1);
            }
        }
        let mut empty_y = self.get_empty_y();
        empty_y.reverse();
        for y in empty_y {
            for galaxy in self.galaxies.iter_mut().filter(|g| g.y > y) {
                galaxy.y.add_assign(expansion_rate - 1);
            }
        }
        self.is_expanded = true;
    }

    fn calculate_total_distance(&mut self) -> (usize, Vec<(usize, usize)>) {
        let mut total_distance = 0;
        let mut nr_of_pairs = Vec::new();
        for galaxy_index in 0..&self.galaxies.len() - 1 {
            let galaxy = self.galaxies.get(galaxy_index).unwrap();
            for other_galaxy_index in galaxy_index + 1..self.galaxies.len() {
                let other_galaxy = self.galaxies.get(other_galaxy_index).unwrap();
                total_distance.add_assign(galaxy.calculate_distance(other_galaxy));
                nr_of_pairs.push((galaxy.id, other_galaxy.id));
            }
        }
        (total_distance, nr_of_pairs)
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::{
        file_parser::{get_input, get_rows},
        method_duration::{self, log_method_duration},
    };

    use super::*;

    fn test_galaxy_distances_helper(expected: usize, x1: usize, y1: usize, x2: usize, y2: usize) {
        assert_eq!(
            expected,
            Galaxy {
                id: 1,
                x: x1,
                y: y1
            }
            .calculate_distance(&Galaxy {
                id: 1,
                x: x2,
                y: y2
            })
        );
        assert_eq!(
            expected,
            Galaxy {
                id: 1,
                x: x2,
                y: y2
            }
            .calculate_distance(&Galaxy {
                id: 1,
                x: x1,
                y: y1
            }),
        );
    }

    #[test]
    fn galaxy_distances() {
        test_galaxy_distances_helper(15, 0, 4, 10, 9);
        test_galaxy_distances_helper(2, 0, 0, 1, 1);
    }

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let mut milky_way = MilkyWay::from(rows);
        milky_way.galaxies.sort_by(|g1, g2| g1.y.cmp(&g2.y));
        milky_way.expand_milky_way(2);
        assert!(milky_way.galaxies.contains(&Galaxy { id: 1, x: 4, y: 0 }));
        assert!(milky_way.galaxies.contains(&Galaxy { id: 2, x: 9, y: 1 }));
        assert!(milky_way.galaxies.contains(&Galaxy { id: 7, x: 9, y: 10 }));
        assert!(milky_way.galaxies.contains(&Galaxy { id: 3, x: 0, y: 2 }));
        let (result, pairs) = milky_way.calculate_total_distance();
        assert_eq!(374, result);
        assert_eq!(36, pairs.len());
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let mut milky_way = MilkyWay::from(rows);
        milky_way.expand_milky_way(2);
        let (result, pairs) = log_method_duration(|| milky_way.calculate_total_distance());
        assert_eq!(9639160, result);
    }

    #[test]
    fn example_2_test_10_x() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let mut milky_way = MilkyWay::from(rows);
        milky_way.expand_milky_way(10);
        let (result, pairs) = milky_way.calculate_total_distance();
        assert_eq!(1030, result);
        assert_eq!(36, pairs.len());
    }

    #[test]
    fn example_2_test_100_x() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let mut milky_way = MilkyWay::from(rows);
        milky_way.expand_milky_way(100);
        let (result, pairs) = milky_way.calculate_total_distance();
        assert_eq!(8410, result);
    }

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let mut milky_way = MilkyWay::from(rows);
        milky_way.expand_milky_way(1_000_000);
        let (result, pairs) = log_method_duration(|| milky_way.calculate_total_distance());
        assert_eq!(752936133304, result);
    }
}
