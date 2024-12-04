#[cfg(test)]
mod tests {
    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn example_1() {
        let input = get_input(file!(), "example.txt");
    }

    #[test]
    fn input_1() {
        let input = get_input(file!(), "input.txt");
    }

    #[test]
    fn example_2() {
        let input = get_input(file!(), "example.txt");
    }

    #[test]
    fn input_2() {
        let input = get_input(file!(), "input.txt");
    }
}
