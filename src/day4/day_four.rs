use std::{
    collections::{HashMap, HashSet},
    mem,
    ops::Add,
};

#[derive(Clone)]
struct Card {
    id: usize,
    winning: HashSet<usize>,
    playing: HashSet<usize>,
}

impl From<String> for Card {
    fn from(row: String) -> Self {
        let mut card = Self {
            id: 0,
            winning: HashSet::new(),
            playing: HashSet::new(),
        };
        let mut card_id_and_numbers = row.split(": ");
        if let Some(card_id) = card_id_and_numbers.next() {
            if let Some(id_str) = card_id.strip_prefix("Card ") {
                card.id = id_str.trim().parse().unwrap();
            }
        }
        if let Some(numbers_str) = card_id_and_numbers.next() {
            let mut numbers_split = numbers_str.split(" | ");
            let extract_numbers = |numbers_str: &Option<&str>, numbers: &mut HashSet<usize>| {
                if let Some(winning_numbers_str) = numbers_str {
                    for winning_number_str in winning_numbers_str.split(" ") {
                        if winning_number_str.is_empty() {
                            continue;
                        }
                        numbers.insert(winning_number_str.parse().unwrap());
                    }
                }
            };
            extract_numbers(&numbers_split.next(), &mut card.winning);
            extract_numbers(&numbers_split.next(), &mut card.playing);
        }
        card
    }
}

impl Card {
    fn sol_1_matches(&self) -> usize {
        let mut nr_of_matches = 0;
        for winning_number in &self.winning {
            if self.playing.contains(winning_number) {
                if nr_of_matches == 0 {
                    nr_of_matches = 1;
                } else {
                    nr_of_matches *= 2;
                }
            }
        }
        nr_of_matches
    }

    fn match_count(&self) -> usize {
        let mut match_count = 0;
        for winning_number in &self.winning {
            if self.playing.contains(winning_number) {
                match_count += 1;
            }
        }
        match_count
    }

    fn from_rows(rows: Vec<String>) -> Vec<Card> {
        rows.into_iter().map(|row| Card::from(row)).collect()
    }
}

fn solution_1(cards: &Vec<Card>) -> usize {
    let mut count = 0;
    for card in cards {
        count += 2_usize.pow(Card::match_count(&card).try_into().unwrap()) / 2;
    }
    count
}

struct Solution2Memoization {
    card_id: usize,
    generated_cards: usize,
}

struct Sol2Memo {
    count: usize,
    memo: Vec<Card>,
}

impl From<usize> for Sol2Memo {
    fn from(value: usize) -> Self {
        Sol2Memo {
            count: value,
            memo: Vec::new(),
        }
    }
}

fn solution_2(
    all_cards: &Vec<Card>,
    cards_to_calculate: &Vec<Card>,
    mut memo: HashMap<usize, usize>,
    mut final_memo: HashMap<usize, usize>,
) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    for card in cards_to_calculate {
        memo.entry(card.id).and_modify(|val| *val += 1).or_insert(1);

        let match_count = Card::match_count(&card);
        let mut new_cards = Vec::new();

        for card_copy_index in (card.id)..(card.id + match_count) {
            if let Some(card_copy) = all_cards.get(card_copy_index) {
                new_cards.push(card_copy.clone());
            }
        }

        (memo, final_memo) = solution_2(all_cards, &new_cards, memo, final_memo);
    }
    (memo, final_memo)
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::shared::file_parser::{get_input, get_rows};

    use super::*;

    #[test]
    fn extract_card() {
        let examples = vec![(
            "Card   3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            Card {
                id: 3,
                winning: vec![1, 21, 53, 59, 44].into_iter().collect(),
                playing: vec![69, 82, 63, 72, 16, 21, 14, 1].into_iter().collect(),
            },
        )];
        for (input, expected) in examples {
            let result = Card::from(String::from(input));
            assert_eq!(expected.id, result.id);
            assert_eq!(expected.winning, result.winning);
            assert_eq!(expected.playing, result.playing);
        }
    }

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let cards = Card::from_rows(rows);
        assert_eq!(13, solution_1(&cards));
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let cards = Card::from_rows(rows);
        assert_eq!(21821, solution_1(&cards));
    }

    #[test]
    fn example_2_test() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let cards = Card::from_rows(rows);
        let (memo, _) = solution_2(&cards, &cards, HashMap::new(), HashMap::new());
        assert_eq!(30_usize, memo.values().into_iter().sum());
    }

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let cards = Card::from_rows(rows);
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let (memo, _) = solution_2(&cards, &cards, HashMap::new(), HashMap::new());
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!("{:?}", end - start);
        assert_eq!(5539496_usize, memo.values().into_iter().sum());
    }
}
