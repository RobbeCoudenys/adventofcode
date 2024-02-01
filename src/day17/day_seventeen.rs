use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    ops::AddAssign,
    thread::panicking,
};

trait FromString {
    fn custom_from(input: String) -> Self;
}
type Cities = Vec<Vec<usize>>;

impl FromString for Cities {
    fn custom_from(value: String) -> Self {
        value
            .lines()
            .map(|row| {
                row.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>()
    }
}

// Define a structure to represent edges
#[derive(Clone, Eq, PartialEq)]
struct Edge {
    position: (usize, usize), // Grid position (i, j)
    cost: usize,
    consecutive_rights: usize,
    last_move: Option<Direction>,
}

#[derive(Clone, Eq, PartialEq, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    None,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Modified Dijkstra's algorithm for the grid with direction constraints
fn dijkstra_with_directions(
    grid: &Cities,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(Vec<(usize, usize)>, usize)> {
    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;
    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    dist.insert(start, 0);
    heap.push(Edge {
        position: start,
        cost: 0,
        consecutive_rights: 0,
        last_move: None,
    });

    while let Some(Edge {
        position,
        cost,
        consecutive_rights: consecutive_straights,
        last_move,
    }) = heap.pop()
    {
        if position == end {
            let mut path = vec![end];
            let mut u = end;
            while u != start {
                u = prev[&u];
                path.push(u);
            }
            path.reverse();
            let mut after_calculation = 0;
            for y in 0..max_y + 1 {
                for x in 0..max_x + 1 {
                    let index = path.iter().position(|coords| coords.eq(&(x, y)));
                    match index {
                        Some(index) => {
                            if index == 0 {
                                print!("{}", grid.get(y).unwrap().get(x).unwrap());
                                continue;
                            }
                            after_calculation.add_assign(grid.get(y).unwrap().get(x).unwrap());
                            let from = path.get(index - 1).unwrap();
                            let to = path.get(index).unwrap();
                            let direction = direction_from_coords(*from, *to);
                            match direction {
                                Direction::Right => print!(">"),
                                Direction::Left => print!("<"),
                                Direction::Up => print!("^"),
                                Direction::Down => print!("v"),
                                Direction::None => panic!("{:?} should not exist", to),
                            }
                        }
                        None => print!("{}", grid.get(y).unwrap().get(x).unwrap()),
                    }
                }
                print!("\n");
            }
            println!("{}", after_calculation);
            return Some((path, cost));
        }

        for neighbor in get_neighbors(position, max_x, max_y, last_move) {
            let move_direction = direction_from_coords(position, neighbor);
            let new_consecutive_straights = match last_move.eq(&Some(move_direction)) {
                true => consecutive_straights + 1,
                false => 0,
            };

            // Skip if this would be the fourth consecutive right move
            if new_consecutive_straights > 2 {
                continue;
            }

            let next_cost = cost + grid[neighbor.1][neighbor.0];

            if next_cost < *dist.get(&neighbor).unwrap_or(&usize::MAX) {
                heap.push(Edge {
                    position: neighbor,
                    cost: next_cost,
                    consecutive_rights: new_consecutive_straights,
                    last_move: Some(move_direction),
                });
                dist.insert(neighbor, next_cost);
                prev.insert(neighbor, position);
            }
        }
    }

    None
}

fn get_neighbors(
    position: (usize, usize),
    max_x: usize,
    max_y: usize,
    last_direction: Option<Direction>,
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    // left
    if last_direction != Some(Direction::Right) && position.0 > 0 {
        neighbors.push((position.0 - 1, position.1));
    }
    // Right
    if last_direction != Some(Direction::Left) && position.0 < max_x {
        neighbors.push((position.0 + 1, position.1));
    }
    // Up
    if last_direction != Some(Direction::Down) && position.1 > 0 {
        neighbors.push((position.0, position.1 - 1));
    }
    // Down
    if last_direction != Some(Direction::Up) && position.1 < max_y {
        neighbors.push((position.0, position.1 + 1));
    }
    neighbors
}

fn direction_from_coords(from: (usize, usize), to: (usize, usize)) -> Direction {
    match (from, to) {
        ((x1, _), (x2, _)) if x1 > x2 => Direction::Left,
        ((x1, _), (x2, _)) if x2 > x1 => Direction::Right,
        ((_, y1), (_, y2)) if y2 > y1 => Direction::Down,
        ((_, y1), (_, y2)) if y2 < y1 => Direction::Up,
        _ => Direction::None,
    }
}

fn solution_1(cities: Cities) -> Option<usize> {
    let max_x = cities.get(0).unwrap().len() - 1;
    let max_y = cities.len() - 1;
    let result = dijkstra_with_directions(&cities, (0, 0), (max_x, max_y));

    match result {
        Some((path, cost)) => {
            println!("Path: {:?}", path);
            println!("Cost: {}", cost);
            return Some(cost);
        }
        None => println!("No path found"),
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");
        let cities = Cities::custom_from(input);

        assert_eq!(102, solution_1(cities).unwrap());
    }

    #[test]
    fn example_1_2_test() {
        let input = get_input(file!(), "example2.txt");
        let cities = Cities::custom_from(input);

        assert_eq!(29, solution_1(cities).unwrap());
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        let cities = Cities::custom_from(input);
        assert_eq!(102, solution_1(cities).unwrap());
    }
}
