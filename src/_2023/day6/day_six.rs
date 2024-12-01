use std::ops::{AddAssign, MulAssign};

struct Race {
    race_duration_in_ms: usize,
    distance_record: usize,
}

impl Race {
    fn calculate_optimal_button_press_time(&self) -> usize {
        self.race_duration_in_ms / 2
    }

    fn calculate_distance(&self, button_press_duration: usize) -> usize {
        let boat_speed = button_press_duration;
        let time_left = self.race_duration_in_ms - button_press_duration;
        let distance_traveled = boat_speed * time_left;
        distance_traveled
    }

    // could be optimized by taking into account the sqrt(distance_record) as a
    // starting point for traversing lower
    fn nr_of_better_solutions_than_record(&self) -> usize {
        let mut nr_of_better_solutions = 0;
        let optimal_button_press = self.calculate_optimal_button_press_time();
        let drive_duration = self.race_duration_in_ms - optimal_button_press;
        let mut current_solution = (optimal_button_press, drive_duration);
        nr_of_better_solutions.add_assign(1);
        if current_solution.0 != current_solution.1 {
            nr_of_better_solutions.add_assign(1);
        }
        let mut should_continue = true;
        while should_continue {
            current_solution = (current_solution.0 - 1, current_solution.1 + 1);
            if self.distance_record < self.calculate_distance(current_solution.0) {
                nr_of_better_solutions.add_assign(2);
            } else {
                should_continue = false;
            }
        }
        nr_of_better_solutions
    }

    fn calculate_better_solutions_than_record(&self) -> Vec<(usize, usize)> {
        let mut better_solutions = Vec::new();
        let optimal_button_press = self.calculate_optimal_button_press_time();
        let drive_duration = self.race_duration_in_ms - optimal_button_press;
        let mut current_solution = (optimal_button_press, drive_duration);
        better_solutions.push(current_solution);
        if current_solution.0 != current_solution.1 {
            better_solutions.push((current_solution.1, current_solution.0));
        }
        let mut should_continue = true;
        while should_continue {
            current_solution = (current_solution.0 - 1, current_solution.1 + 1);
            if self.distance_record < self.calculate_distance(current_solution.0) {
                better_solutions.push(current_solution);
                better_solutions.push((current_solution.1, current_solution.0));
            } else {
                should_continue = false;
            }
        }
        better_solutions
    }
}

type Records = Vec<Race>;

fn solution_1(records: Records) -> usize {
    let mut result = 1;
    for race in records {
        result.mul_assign(race.nr_of_better_solutions_than_record());
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::shared::method_duration::log_method_duration;

    use super::*;

    #[test]
    fn test_calculation() {
        let race = Race {
            race_duration_in_ms: 7,
            distance_record: 9,
        };
        assert_eq!(0, race.calculate_distance(0));
        assert_eq!(6, race.calculate_distance(1));
        assert_eq!(10, race.calculate_distance(2));
        assert_eq!(12, race.calculate_distance(3));
        assert_eq!(12, race.calculate_distance(4));
        assert_eq!(10, race.calculate_distance(5));
        assert_eq!(6, race.calculate_distance(6));
        assert_eq!(0, race.calculate_distance(7));
    }

    #[test]
    fn example_1_test() {
        let input: Records = vec![
            Race {
                race_duration_in_ms: 7,
                distance_record: 9,
            },
            Race {
                race_duration_in_ms: 15,
                distance_record: 40,
            },
            Race {
                race_duration_in_ms: 30,
                distance_record: 200,
            },
        ];
        let race1 = input.get(0).unwrap();
        let race2 = input.get(1).unwrap();
        let race3 = input.get(2).unwrap();
        assert_eq!(4, race1.calculate_better_solutions_than_record().len());
        assert_eq!(8, race2.calculate_better_solutions_than_record().len());
        assert_eq!(9, race3.calculate_better_solutions_than_record().len());
        assert_eq!(288, solution_1(input));
    }

    #[test]
    fn solution_1_test() {
        let input: Records = vec![
            Race {
                race_duration_in_ms: 61,
                distance_record: 430,
            },
            Race {
                race_duration_in_ms: 67,
                distance_record: 1036,
            },
            Race {
                race_duration_in_ms: 75,
                distance_record: 1307,
            },
            Race {
                race_duration_in_ms: 71,
                distance_record: 1150,
            },
        ];
        assert_eq!(316800, solution_1(input));
    }

    #[test]
    fn example_2_test() {
        let input: Records = vec![Race {
            race_duration_in_ms: 71530,
            distance_record: 940200,
        }];
        assert_eq!(71503, solution_1(input));
    }

    #[test]
    fn solution_2_test() {
        let input: Records = vec![Race {
            race_duration_in_ms: 61677571,
            distance_record: 430103613071150,
        }];
        assert_eq!(
            45647654,
            log_method_duration(|| {
                return solution_1(input);
            })
        );
    }
}
