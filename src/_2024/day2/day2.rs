fn parse_input(input: String) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let test: Vec<u32> = line.split(" ").map(|x| x.parse::<u32>().unwrap()).collect();
        res.push(test);
    }
    res
}

#[derive(PartialEq)]
enum ListType {
    Assending,
    Descending,
    Unknown,
}

impl From<bool> for ListType {
    fn from(value: bool) -> Self {
        match value {
            true => ListType::Assending,
            false => ListType::Descending,
        }
    }
}

fn is_valid_list(list: &Vec<u32>, allowed_unsafes: u32) -> bool {
    let mut list_iter = list.into_iter();
    let mut prev = list_iter.next().unwrap();
    let mut list_type = ListType::Unknown;
    for current in list_iter {
        if prev == current {
            if allowed_unsafes == 0 {
                return false;
            }
            if !is_valid_list_removed_level(list, allowed_unsafes) {
                return false;
            }
            continue;
        }
        let is_assending = prev < current;
        if list_type == ListType::Unknown {
            list_type = is_assending.into();
        } else if list_type != is_assending.into() {
            if allowed_unsafes == 0 {
                return false;
            }
            if !is_valid_list_removed_level(list, allowed_unsafes) {
                return false;
            }
        }
        if prev.abs_diff(*current) > 3 {
            if allowed_unsafes == 0 {
                return false;
            }
            if !is_valid_list_removed_level(list, allowed_unsafes) {
                return false;
            }
        }
        prev = current;
    }
    true
}

fn is_valid_list_removed_level(list: &Vec<u32>, allowed_unsafes: u32) -> bool {
    let mut lists: Vec<Vec<u32>> = Vec::with_capacity(list.len());
    for index in 0..list.len() {
        let mut new_list = list.clone();
        new_list.remove(index);
        lists.push(new_list);
    }
    count_valid_lists(lists, allowed_unsafes - 1) > 0
}

fn count_valid_lists(lists: Vec<Vec<u32>>, allowed_unsafes: u32) -> usize {
    lists
        .into_iter()
        .filter(|x| is_valid_list(x, allowed_unsafes))
        .count()
}

#[cfg(test)]
mod tests {

    use crate::shared::file_parser::get_input;

    use super::*;

    #[test]
    fn example_1() {
        let input = get_input(file!(), "example.txt");
        let parsed = parse_input(input);
        assert_eq!(count_valid_lists(parsed, 0), 2);
    }

    #[test]
    fn input_1() {
        let input = get_input(file!(), "input.txt");
        let parsed = parse_input(input);
        assert_eq!(count_valid_lists(parsed, 0), 257);
    }

    #[test]
    fn example_2_is_valid_list() {
        let input = get_input(file!(), "example.txt");
        let parsed = parse_input(input);
        assert_eq!(is_valid_list(&parsed[0], 1), true);
        assert_eq!(is_valid_list(&parsed[1], 1), false);
        assert_eq!(is_valid_list(&parsed[2], 1), false);
        assert_eq!(is_valid_list(&parsed[3], 1), true);
        assert_eq!(is_valid_list(&parsed[4], 1), true);
        assert_eq!(is_valid_list(&parsed[5], 1), true);
    }

    #[test]
    fn example_2() {
        let input = get_input(file!(), "example.txt");
        let parsed = parse_input(input);
        assert_eq!(count_valid_lists(parsed, 1), 4);
    }

    #[test]
    fn input_2() {
        let input = get_input(file!(), "input.txt");
        let parsed = parse_input(input);
        assert_eq!(count_valid_lists(parsed, 1), 328);
    }
}
