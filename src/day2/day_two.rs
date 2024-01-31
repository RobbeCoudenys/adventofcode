use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
enum CubeColor {
    RED,
    BLUE,
    GREEN,
}

#[derive(Eq, PartialEq, Debug)]
struct Game {
    id: i32,
    cube_sets: Vec<HashMap<CubeColor, i32>>,
}

impl From<String> for Game {
    fn from(row: String) -> Self {
        let id: i32;
        let mut cube_sets = Vec::<HashMap<CubeColor, i32>>::new();
        let mut game_id_and_cube_sets = row.split(": ");
        match game_id_and_cube_sets.next() {
            Some(game_id) => match game_id.strip_prefix("Game ") {
                Some(id_str) => id = id_str.parse().unwrap(),
                None => todo!(),
            },
            None => todo!(),
        }
        match game_id_and_cube_sets.next() {
            Some(cube_sets_as_str) => {
                for cube_set in cube_sets_as_str.split("; ") {
                    let mut cube_set_to_add = HashMap::<CubeColor, i32>::new();
                    for color in cube_set.split(", ") {
                        if color.ends_with(" blue") {
                            cube_set_to_add.insert(
                                CubeColor::BLUE,
                                color.replace(" blue", "").parse().unwrap(),
                            );
                        } else if color.ends_with(" red") {
                            cube_set_to_add
                                .insert(CubeColor::RED, color.replace(" red", "").parse().unwrap());
                        } else if color.ends_with(" green") {
                            cube_set_to_add.insert(
                                CubeColor::GREEN,
                                color.replace(" green", "").parse().unwrap(),
                            );
                        }
                    }
                    cube_sets.push(cube_set_to_add);
                }
            }
            None => todo!(),
        }

        Game { id, cube_sets }
    }
}

struct OptimizedGameSol1 {
    id: i32,
    max_red: i32,
    max_blue: i32,
    max_green: i32,
}

impl OptimizedGameSol1 {
    fn from(game: &Game) -> Self {
        let mut max_red: i32 = 0;
        let mut max_blue: i32 = 0;
        let mut max_green: i32 = 0;
        for cube_set in &game.cube_sets {
            for (color, count) in cube_set {
                match color {
                    CubeColor::RED => {
                        if count > &max_red {
                            max_red = count.to_owned();
                        }
                    }
                    CubeColor::BLUE => {
                        if count > &max_blue {
                            max_blue = count.to_owned();
                        }
                    }
                    CubeColor::GREEN => {
                        if count > &max_green {
                            max_green = count.to_owned();
                        }
                    }
                }
            }
        }

        OptimizedGameSol1 {
            id: game.id,
            max_red,
            max_blue,
            max_green,
        }
    }

    fn is_possible(this: &Self, bag: &HashMap<CubeColor, i32>) -> bool {
        let mut is_possible = true;
        for (cube_color, cube_amount) in bag {
            match cube_color {
                CubeColor::RED => is_possible = this.max_red <= *cube_amount,
                CubeColor::BLUE => is_possible = this.max_blue <= *cube_amount,
                CubeColor::GREEN => is_possible = this.max_green <= *cube_amount,
            }
            if !is_possible {
                break;
            }
        }
        is_possible
    }

    fn power(this: &Self) -> i32 {
        this.max_red * this.max_blue * this.max_green
    }
}

fn extract_games_from_rows(rows: Vec<String>) -> Vec<Game> {
    rows.iter()
        .filter(|row| row.len() > 0)
        .map(|row| Game::from(row.to_string()))
        .collect()
}

fn solution_1(games: Vec<Game>, bag: HashMap<CubeColor, i32>) -> i32 {
    games
        .into_iter()
        .map(|game| OptimizedGameSol1::from(&game))
        .filter(|optimized_game| OptimizedGameSol1::is_possible(optimized_game, &bag))
        .map(|optimized_game| optimized_game.id)
        .sum()
}

fn solution_2(games: Vec<Game>) -> i32 {
    games
        .into_iter()
        .map(|game| OptimizedGameSol1::from(&game))
        .map(|optimized_game| OptimizedGameSol1::power(&optimized_game))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::shared::file_parser::{get_input, get_rows};

    use super::*;

    fn create_bag(red: i32, blue: i32, green: i32) -> HashMap<CubeColor, i32> {
        vec![
            (CubeColor::RED, red),
            (CubeColor::GREEN, green),
            (CubeColor::BLUE, blue),
        ]
        .into_iter()
        .collect()
    }

    fn create_optimized_game(red: i32, blue: i32, green: i32) -> OptimizedGameSol1 {
        OptimizedGameSol1 {
            id: 0,
            max_red: red,
            max_blue: blue,
            max_green: green,
        }
    }

    #[test]
    fn test_parser() {
        let examples = vec![(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            Game {
                id: 1,
                cube_sets: vec![
                    vec![(CubeColor::BLUE, 3), (CubeColor::RED, 4)]
                        .into_iter()
                        .collect(),
                    vec![
                        (CubeColor::RED, 1),
                        (CubeColor::GREEN, 2),
                        (CubeColor::BLUE, 6),
                    ]
                    .into_iter()
                    .collect(),
                    vec![(CubeColor::GREEN, 2)].into_iter().collect(),
                ],
            },
        )];
        for (input, expected) in examples {
            assert_eq!(expected, Game::from(input.to_owned()));
        }
    }

    #[test]
    fn test_optimizer_day_one() {
        let game = Game {
            id: 1,
            cube_sets: vec![
                vec![(CubeColor::BLUE, 3), (CubeColor::RED, 4)]
                    .into_iter()
                    .collect(),
                vec![
                    (CubeColor::RED, 1),
                    (CubeColor::GREEN, 2),
                    (CubeColor::BLUE, 6),
                ]
                .into_iter()
                .collect(),
                vec![(CubeColor::GREEN, 2)].into_iter().collect(),
            ],
        };
        let optimized_game = OptimizedGameSol1::from(&game);
        assert_eq!(optimized_game.id, game.id);
        assert_eq!(optimized_game.max_red, 4);
        assert_eq!(optimized_game.max_green, 2);
        assert_eq!(optimized_game.max_blue, 6);
    }

    #[test]
    fn test_is_possible() {
        let balls_in_bag = create_bag(15, 15, 15);
        let game_1 = create_optimized_game(5, 5, 5);

        assert!(
            OptimizedGameSol1::is_possible(&game_1, &balls_in_bag),
            "This should be possible"
        );

        let balls_in_bag = create_bag(5, 5, 5);
        assert!(
            OptimizedGameSol1::is_possible(&game_1, &balls_in_bag),
            "This should be possible"
        );
    }

    #[test]
    fn example_1() {
        let balls_in_bag = create_bag(12, 14, 13);

        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        assert_eq!(8, solution_1(extract_games_from_rows(rows), balls_in_bag));
    }

    #[test]
    fn solution_1_test() {
        let balls_in_bag = create_bag(12, 14, 13);

        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let solution = solution_1(extract_games_from_rows(rows), balls_in_bag);
        assert_eq!(3059, solution);
    }

    #[test]
    fn test_power() {
        let game = Game {
            id: 1,
            cube_sets: vec![
                vec![(CubeColor::BLUE, 3), (CubeColor::RED, 4)]
                    .into_iter()
                    .collect(),
                vec![
                    (CubeColor::RED, 1),
                    (CubeColor::GREEN, 2),
                    (CubeColor::BLUE, 6),
                ]
                .into_iter()
                .collect(),
                vec![(CubeColor::GREEN, 2)].into_iter().collect(),
            ],
        };
        let optimized_game = OptimizedGameSol1::from(&game);
        assert_eq!(48, OptimizedGameSol1::power(&optimized_game));
    }

    #[test]
    fn example_2() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        assert_eq!(2286, solution_2(extract_games_from_rows(rows)));
    }

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let solution = solution_2(extract_games_from_rows(rows));
        assert_eq!(65371, solution);
    }
}
