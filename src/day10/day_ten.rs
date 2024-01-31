use std::{
    collections::HashMap,
    ops::{AddAssign, SubAssign},
};

type Coordinate = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct Pipe {
    character: char,
    coordinate: Coordinate,
    links: Vec<Coordinate>,
}

impl Pipe {
    fn from(value: char, coordinate: &Coordinate, pipe_map: &PipeMazeMap) -> Self {
        let pipe = Self {
            coordinate: (coordinate.0, coordinate.1),
            links: Vec::new(),
            character: value,
        };
        match value.into() {
            '7' => pipe.left().down(pipe_map),
            'J' => pipe.up().left(),
            'F' => pipe.right(pipe_map).down(pipe_map),
            'L' => pipe.up().right(pipe_map),
            '|' => pipe.up().down(pipe_map),
            '-' => pipe.left().right(pipe_map),
            'S' => pipe.start(pipe_map),
            _ => pipe,
        }
    }

    fn add_link(&mut self, coordinate: Coordinate) {
        self.links.push(coordinate);
    }

    fn start(self, pipe_map: &PipeMazeMap) -> Self {
        self.left().right(pipe_map).up().down(pipe_map)
    }

    fn left(mut self) -> Self {
        if self.coordinate.0 > 0 {
            let coordinate = (self.coordinate.0 - 1, self.coordinate.1);
            self.add_link(coordinate);
        };
        self
    }

    fn right(mut self, pipe_map: &PipeMazeMap) -> Self {
        let coordinate = (self.coordinate.0 + 1, self.coordinate.1);
        if let Some(_) = pipe_map.get(&coordinate) {
            self.add_link(coordinate);
        }
        self
    }

    fn up(mut self) -> Self {
        if self.coordinate.1 > 0 {
            let coordinate = (self.coordinate.0, self.coordinate.1 - 1);
            self.add_link(coordinate);
        }
        self
    }

    fn down(mut self, pipe_map: &PipeMazeMap) -> Self {
        let coordinate = (self.coordinate.0, self.coordinate.1 + 1);
        if let Some(_) = pipe_map.get(&coordinate) {
            self.add_link(coordinate);
        }
        self
    }

    fn find_next<'a>(&self, coming_from: &Pipe, pipe_map: &'a PipeMazeWrapper) -> Option<&'a Pipe> {
        self.links
            .iter()
            .filter_map(|c| pipe_map.value.get(c))
            .find(|next_pipe| {
                next_pipe.coordinate.ne(&coming_from.coordinate)
                    && next_pipe.links.contains(&self.coordinate)
            })
    }
}

struct PipeMazeWrapper {
    value: HashMap<Coordinate, Pipe>,
}

impl PipeMazeWrapper {
    fn get_start(&self) -> &Pipe {
        *self
            .value
            .values()
            .filter(|v| v.character.eq(&'S'))
            .collect::<Vec<&Pipe>>()
            .first()
            .unwrap()
    }
}

impl From<PipeMazeMap> for PipeMazeWrapper {
    fn from(pipe_maze_map: PipeMazeMap) -> Self {
        let mut map = HashMap::new();
        for (coordinate, value) in &pipe_maze_map {
            map.insert(
                (coordinate.0, coordinate.1),
                Pipe::from(*value, coordinate, &pipe_maze_map),
            );
        }
        Self { value: map }
    }
}

type PipeMazeMap = HashMap<Coordinate, char>;

fn pipe_maze_from_string(rows: Vec<String>) -> PipeMazeMap {
    let mut pipe_maze = PipeMazeMap::new();
    for (y, row) in rows.into_iter().enumerate() {
        for (x, c) in row.split("").filter(|r| !r.is_empty()).enumerate() {
            if let Some(ch) = c.chars().next() {
                pipe_maze.insert((x, y), ch);
            } else {
                println!("{}, {}", x, y);
            }
        }
    }
    pipe_maze
}

fn solution_1(pipe_maze: PipeMazeWrapper) -> usize {
    let mut pipe_count_collection: Vec<usize> = Vec::new();
    let start_pipe = pipe_maze.get_start();

    for first_pipe_coordinate in &start_pipe.links {
        let mut has_next = true;
        if let Some(first_pipe) = pipe_maze.value.get(first_pipe_coordinate) {
            if !first_pipe.links.contains(&start_pipe.coordinate) {
                continue;
            }
            // starting with 1 because we skipped the first link
            let mut nr_of_pipes = 1;
            let mut previous_pipe = start_pipe;
            let mut current_pipe = first_pipe;
            while has_next && current_pipe.ne(&start_pipe) {
                match current_pipe.find_next(previous_pipe, &pipe_maze) {
                    Some(next_pipe) => {
                        nr_of_pipes.add_assign(1);
                        previous_pipe = current_pipe;
                        current_pipe = next_pipe;
                    }
                    None => has_next = false,
                }
            }
            if has_next {
                pipe_count_collection.push(nr_of_pipes);
            }
        }
    }

    pipe_count_collection.into_iter().min().unwrap() / 2
}

fn solution_2(pipe_maze: PipeMazeWrapper) -> usize {
    let start_pipe = pipe_maze.get_start();
    let mut pipe_loops = Vec::new();
    let mut already_done_in_reverse = Vec::new();

    for first_pipe_coordinate in &start_pipe.links {
        let mut connected_pipes: Vec<&Pipe> = Vec::new();
        let mut has_next = true;
        if let Some(first_pipe) = pipe_maze.value.get(first_pipe_coordinate) {
            if !first_pipe.links.contains(&start_pipe.coordinate) {
                continue;
            }
            // don't iterate over a loop that was already closed before in the
            // other direction
            if already_done_in_reverse.contains(&first_pipe) {
                continue;
            }
            connected_pipes.push(first_pipe);
            // starting with 1 because we skipped the first link
            let mut previous_pipe = start_pipe;
            let mut current_pipe = first_pipe;
            while has_next && current_pipe.ne(&start_pipe) {
                match current_pipe.find_next(previous_pipe, &pipe_maze) {
                    Some(next_pipe) => {
                        connected_pipes.push(next_pipe);
                        previous_pipe = current_pipe;
                        current_pipe = next_pipe;
                    }
                    None => {
                        has_next = false;
                        connected_pipes = Vec::new();
                    }
                }
            }
            if has_next {
                already_done_in_reverse.push(previous_pipe);
                pipe_loops.push(connected_pipes);
            }
        }
    }
    pipe_loops
        .into_iter()
        .map(|v| calculate_area(v))
        .min()
        .unwrap()
}

fn calculate_area(pipe_loop: Vec<&Pipe>) -> usize {
    // using shoelace formula
    let coordinates = pipe_loop
        .iter()
        .map(|p| p.coordinate)
        .collect::<Vec<Coordinate>>();
    let n = coordinates.len();
    let mut area = 0isize;

    for i in 0..n {
        let j = (i + 1) % n;
        area.add_assign((coordinates[i].0 * coordinates[j].1) as isize);
        area.sub_assign((coordinates[j].0 * coordinates[i].1) as isize);
    }

    // combining shoelace formula with pick's formula
    (area.abs() / 2) as usize - (n / 2) + 1
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::{get_input, get_rows};

    use super::*;

    #[test]
    fn feature() {
        let mut my_pipe_links = Vec::new();

        // Add a link
        my_pipe_links.push((1, 1));

        // Print the modified PipeLinks
    }

    #[test]
    fn test_pipe_from() {
        let row = String::from("JJJJJJJ");
        let pipe_maze = PipeMazeWrapper::from(pipe_maze_from_string(vec![row]));
    }

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let pipe_maze_map = pipe_maze_from_string(rows);
        let pipe_maze = PipeMazeWrapper::from(pipe_maze_map);
        let solution = solution_1(pipe_maze);
        assert_eq!(8, solution);
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let pipe_maze_map = pipe_maze_from_string(rows);
        let pipe_maze = PipeMazeWrapper::from(pipe_maze_map);
        let solution = solution_1(pipe_maze);
        assert_eq!(6931, solution);
    }

    #[test]
    fn example_2_test() {
        example_2_test_helper("example2.txt", 10);
        example_2_test_helper("example3.txt", 13);
        example_2_test_helper("example4.txt", 37);
        example_2_test_helper("example5.txt", 13);
    }

    fn example_2_test_helper(file_name: &str, expected_result: usize) {
        let input = get_input(file!(), file_name);
        let rows = get_rows(input);
        let pipe_maze_map = pipe_maze_from_string(rows);
        let pipe_maze = PipeMazeWrapper::from(pipe_maze_map);
        let solution = solution_2(pipe_maze);
        assert_eq!(expected_result, solution, "{}", file_name);
    }

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let pipe_maze_map = pipe_maze_from_string(rows);
        let pipe_maze = PipeMazeWrapper::from(pipe_maze_map);
        let solution = solution_2(pipe_maze);
        assert_eq!(357, solution);
    }
}
