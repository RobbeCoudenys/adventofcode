use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
enum SchemaValue {
    Number(usize),
    NumberPlaceHolder,
    Symbol(char),
    Empty,
}

impl SchemaValue {
    fn get_number_coordinate(content: &SchemaContent, coord: Coordinate) -> Option<Coordinate> {
        match content.get(&coord) {
            Some(schema_value) => match schema_value {
                SchemaValue::Number(value) => Option::Some(coord),
                SchemaValue::NumberPlaceHolder => SchemaValue::get_number_coordinate(
                    content,
                    Coordinate {
                        x: coord.x + 1,
                        y: coord.y,
                    },
                ),
                SchemaValue::Symbol(_) => Option::None,
                SchemaValue::Empty => Option::None,
            },
            None => Option::None,
        }
    }

    fn get_value(content: &SchemaContent, coord: Coordinate) -> usize {
        match content.get(&coord) {
            Some(schema_value) => match schema_value {
                SchemaValue::Number(number) => *number,
                SchemaValue::NumberPlaceHolder => 0,
                SchemaValue::Symbol(_) => 0,
                SchemaValue::Empty => 0,
            },
            None => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn from(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

type SchemaContent = HashMap<Coordinate, SchemaValue>;

struct Schema {
    content: SchemaContent,
}

impl Schema {
    fn from(rows: Vec<String>) -> Schema {
        let mut content: SchemaContent = HashMap::new();
        for (y, row) in rows.into_iter().enumerate() {
            Schema::add_row_to_content(&mut content, y, row);
        }
        Schema { content }
    }

    fn add_row_to_content(content: &mut SchemaContent, y: usize, row: String) {
        let mut current_number = String::new();

        for (x, character) in row.chars().enumerate() {
            if character.is_digit(10) {
                content.insert(Coordinate { x, y }, SchemaValue::NumberPlaceHolder);
                current_number.push(character);
            } else {
                if !current_number.is_empty() {
                    if let Ok(number) = current_number.parse::<usize>() {
                        content.insert(Coordinate { x: x - 1, y }, SchemaValue::Number(number));
                    }
                    current_number.clear();
                }
                if character.eq(&'.') {
                    content.insert(Coordinate { x, y }, SchemaValue::Empty);
                } else {
                    content.insert(Coordinate { x, y }, SchemaValue::Symbol(character));
                }
            }
        }

        // Check for the last number in the string
        if !current_number.is_empty() {
            if let Ok(number) = current_number.parse::<usize>() {
                content.insert(
                    Coordinate {
                        x: row.len() - 1,
                        y,
                    },
                    SchemaValue::Number(number),
                );
            }
        }
    }

    fn get_symbol_coords<'a>(&'a self, optional_symbol: Option<char>) -> Vec<&'a Coordinate> {
        let mut coordinates = Vec::new();
        for (coordinate, schema_value) in &self.content {
            if let SchemaValue::Symbol(symbol) = schema_value {
                match optional_symbol {
                    Some(symbol_to_match) => {
                        if symbol.eq(&symbol_to_match) {
                            coordinates.push(coordinate);
                        }
                    }
                    None => coordinates.push(coordinate),
                }
            }
        }
        coordinates
    }

    fn get_adjacent_coords(coordinate: &Coordinate) -> Vec<Coordinate> {
        let mut coords = Vec::new();
        let x = coordinate.x;
        let y = coordinate.y;

        if y > 0 && x > 0 {
            coords.push(Coordinate::from(x - 1, y - 1));
        }
        if y > 0 {
            coords.push(Coordinate::from(x, y - 1));
            coords.push(Coordinate::from(x + 1, y - 1));
        }
        if x > 0 {
            coords.push(Coordinate::from(x - 1, y));
            coords.push(Coordinate::from(x - 1, y + 1));
        }
        coords.push(Coordinate::from(x + 1, y));
        coords.push(Coordinate::from(x, y + 1));
        coords.push(Coordinate::from(x + 1, y + 1));
        coords
    }
}

fn solution_1(schema: Schema) -> usize {
    let symbol_coordinates = Schema::get_symbol_coords(&schema, Option::None);
    let mut total = 0;
    let mut adjacent_coordinates = HashSet::new();
    for symbol_coordinate in symbol_coordinates {
        for adjacant_coordinate in Schema::get_adjacent_coords(symbol_coordinate) {
            adjacent_coordinates.insert(adjacant_coordinate);
        }
    }
    let mut number_coordinates = HashSet::new();
    for adjacent_coordinate in adjacent_coordinates {
        if let Some(coordinate) =
            SchemaValue::get_number_coordinate(&schema.content, adjacent_coordinate)
        {
            number_coordinates.insert(coordinate);
        }
    }
    for number_coordinate in number_coordinates {
        total += SchemaValue::get_value(&schema.content, number_coordinate);
    }

    total
}

fn solution_2(schema: Schema) -> usize {
    let gear_coordinates = Schema::get_symbol_coords(&schema, Option::Some('*'));
    let mut total = 0;
    for gear_coordinate in gear_coordinates {
        let mut adjacent_coordinates = HashSet::new();
        for adjacant_coordinate in Schema::get_adjacent_coords(gear_coordinate) {
            adjacent_coordinates.insert(adjacant_coordinate);
        }
        let mut number_coordinates = HashSet::new();
        for adjacent_coordinate in adjacent_coordinates {
            if let Some(coordinate) =
                SchemaValue::get_number_coordinate(&schema.content, adjacent_coordinate)
            {
                number_coordinates.insert(coordinate);
            }
        }
        if number_coordinates.len() != 2 {
            continue;
        }
        let mut number_to_add = 1;
        for number_coordinate in number_coordinates {
            number_to_add *= SchemaValue::get_value(&schema.content, number_coordinate);
        }

        total += number_to_add;
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::{get_input, get_rows};

    use super::*;

    fn test_coord_value(content: &SchemaContent, coord_value: SchemaValue, x: usize, y: usize) {
        assert_eq!(&coord_value, content.get(&Coordinate { x, y }).unwrap());
    }

    #[test]
    fn example_1_test_row_1() {
        let row = String::from("467..114..");
        let schema = Schema::from(vec![row]);
        let content = &schema.content;
        test_coord_value(content, SchemaValue::NumberPlaceHolder, 0, 0);
        test_coord_value(content, SchemaValue::NumberPlaceHolder, 1, 0);
        test_coord_value(content, SchemaValue::Number(467), 2, 0);
        test_coord_value(content, SchemaValue::Empty, 3, 0);
        test_coord_value(content, SchemaValue::Empty, 4, 0);
        test_coord_value(content, SchemaValue::NumberPlaceHolder, 5, 0);
        test_coord_value(content, SchemaValue::NumberPlaceHolder, 6, 0);
        test_coord_value(content, SchemaValue::Number(114), 7, 0);
        test_coord_value(content, SchemaValue::Empty, 8, 0);
        test_coord_value(content, SchemaValue::Empty, 9, 0);
        let symbol_coords = Schema::get_symbol_coords(&schema, Option::None);
        assert!(symbol_coords.is_empty());

        let row = String::from("617*......");
        let schema = Schema::from(vec![row]);
        let content = &schema.content;
        test_coord_value(content, SchemaValue::NumberPlaceHolder, 0, 0);
        test_coord_value(content, SchemaValue::NumberPlaceHolder, 1, 0);
        test_coord_value(content, SchemaValue::Number(617), 2, 0);
        test_coord_value(content, SchemaValue::Symbol('*'), 3, 0);
        test_coord_value(content, SchemaValue::Empty, 4, 0);
        test_coord_value(content, SchemaValue::Empty, 5, 0);
        test_coord_value(content, SchemaValue::Empty, 6, 0);
        test_coord_value(content, SchemaValue::Empty, 7, 0);
        test_coord_value(content, SchemaValue::Empty, 8, 0);
        test_coord_value(content, SchemaValue::Empty, 9, 0);
        let symbol_coords = Schema::get_symbol_coords(&schema, Option::None);
        assert!(symbol_coords.contains(&&Coordinate { x: 3, y: 0 }));

        let row = String::from("..$..+.58.");
        let schema = Schema::from(vec![row]);
        let content = &schema.content;
        test_coord_value(content, SchemaValue::Empty, 0, 0);
        test_coord_value(content, SchemaValue::Empty, 1, 0);
        test_coord_value(content, SchemaValue::Symbol('$'), 2, 0);
        test_coord_value(content, SchemaValue::Empty, 3, 0);
        test_coord_value(content, SchemaValue::Empty, 4, 0);
        test_coord_value(content, SchemaValue::Symbol('+'), 5, 0);
        test_coord_value(content, SchemaValue::Empty, 6, 0);
        test_coord_value(content, SchemaValue::NumberPlaceHolder, 7, 0);
        test_coord_value(content, SchemaValue::Number(58), 8, 0);
        test_coord_value(content, SchemaValue::Empty, 9, 0);
        let symbol_coords = Schema::get_symbol_coords(&schema, Option::None);
        assert!(symbol_coords.contains(&&Coordinate { x: 2, y: 0 }));
        assert!(symbol_coords.contains(&&Coordinate { x: 5, y: 0 }));

        let row = String::from("5432.5.+.584");
        let schema = Schema::from(vec![row]);
        let content = &schema.content;
        test_coord_value(content, SchemaValue::NumberPlaceHolder, 0, 0);
        test_coord_value(content, SchemaValue::NumberPlaceHolder, 1, 0);
        test_coord_value(content, SchemaValue::NumberPlaceHolder, 2, 0);
        test_coord_value(content, SchemaValue::Number(5432), 3, 0);
        test_coord_value(content, SchemaValue::Empty, 4, 0);
        test_coord_value(content, SchemaValue::Number(5), 5, 0);
        test_coord_value(content, SchemaValue::Empty, 6, 0);
        test_coord_value(content, SchemaValue::Symbol('+'), 7, 0);
        test_coord_value(content, SchemaValue::Empty, 8, 0);
        test_coord_value(content, SchemaValue::NumberPlaceHolder, 9, 0);
        test_coord_value(content, SchemaValue::NumberPlaceHolder, 10, 0);
        test_coord_value(content, SchemaValue::Number(584), 11, 0);
        let symbol_coords = Schema::get_symbol_coords(&schema, Option::None);
        assert!(symbol_coords.contains(&&Coordinate { x: 7, y: 0 }));
    }

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let schema = Schema::from(rows);

        assert_eq!(4361, solution_1(schema));
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let schema = Schema::from(rows);

        assert_eq!(527144, solution_1(schema));
    }

    #[test]
    fn example_2_test() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let schema = Schema::from(rows);

        assert_eq!(467835, solution_2(schema));
    }

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let schema = Schema::from(rows);

        assert_eq!(81463996, solution_2(schema));
    }
}
