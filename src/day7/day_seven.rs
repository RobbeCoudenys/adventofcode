use std::{collections::HashSet, path::Iter};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandValue {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Play {
    bid: usize,
    hand_value: HandValue,
    hand: Vec<usize>,
}

impl Play {
    fn from_2(value: &str) -> Self {
        let mut hand = Vec::new();
        let mut parts = value.split(" ");
        for char in parts.next().unwrap().chars().into_iter() {
            hand.push(card_value_2(char));
        }

        let hand_value = hand_value_2(&hand);
        let bid = parts.next().unwrap().parse().unwrap();
        Self {
            hand,
            hand_value,
            bid,
        }
    }
}

impl From<&str> for Play {
    fn from(value: &str) -> Self {
        let mut hand = Vec::new();
        let mut parts = value.split(" ");
        for char in parts.next().unwrap().chars().into_iter() {
            hand.push(card_value(char));
        }

        let hand_value = hand_value_1(&hand);
        let bid = parts.next().unwrap().parse().unwrap();
        Self {
            hand,
            hand_value,
            bid,
        }
    }
}

fn hand_value_1(hand: &Vec<usize>) -> HandValue {
    let test = hand
        .into_iter()
        .collect::<HashSet<&usize>>()
        .into_iter()
        .collect::<Vec<&usize>>();
    if test.len() == 5 {
        return HandValue::HighCard;
    }
    if test.len() == 4 {
        return HandValue::Pair;
    }
    if test.len() == 3 {
        if hand.iter().filter(|hv| hv.eq(test.get(0).unwrap())).count() == 2
            || hand.iter().filter(|hv| hv.eq(test.get(1).unwrap())).count() == 2
            || hand.iter().filter(|hv| hv.eq(test.get(2).unwrap())).count() == 2
        {
            return HandValue::TwoPair;
        }
        return HandValue::ThreeOfAKind;
    }
    if test.len() == 2 {
        let first_value_freq = hand.iter().filter(|hv| hv.eq(test.get(0).unwrap())).count();
        if first_value_freq == 2 || first_value_freq == 3 {
            return HandValue::FullHouse;
        } else {
            return HandValue::FourOfAKind;
        }
    }
    HandValue::FiveOfAKind
}

fn hand_value_2(hand: &Vec<usize>) -> HandValue {
    let mut test = hand.into_iter().collect::<HashSet<&usize>>();
    test.remove(&1usize);
    let test = test.into_iter().collect::<Vec<&usize>>();
    let nr_of_jacks = hand.iter().filter(|hv| hv.eq(&&1usize)).count();
    if test.len() == 5 {
        return HandValue::HighCard;
    }
    if test.len() == 4 {
        return HandValue::Pair;
    }
    if test.len() == 3 {
        if nr_of_jacks == 0
            && (hand.iter().filter(|hv| hv.eq(test.get(0).unwrap())).count() == 2
                || hand.iter().filter(|hv| hv.eq(test.get(1).unwrap())).count() == 2
                || hand.iter().filter(|hv| hv.eq(test.get(2).unwrap())).count() == 2)
        {
            return HandValue::TwoPair;
        }
        return HandValue::ThreeOfAKind;
    }
    if test.len() == 2 {
        let check_freq = |freq: usize| {
            if nr_of_jacks > 2 {
                return false;
            }
            return freq == 3 - nr_of_jacks || freq == 2 - nr_of_jacks;
        };
        let first_value_freq = hand.iter().filter(|hv| hv.eq(test.get(0).unwrap())).count();
        let second_value_freq = hand.iter().filter(|hv| hv.eq(test.get(1).unwrap())).count();
        if check_freq(first_value_freq) && check_freq(second_value_freq) {
            return HandValue::FullHouse;
        } else {
            return HandValue::FourOfAKind;
        }
    }
    HandValue::FiveOfAKind
}

fn card_value(value: char) -> usize {
    match value.into() {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

fn card_value_2(value: char) -> usize {
    match value.into() {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

fn solution_1(mut plays: Vec<Play>) -> usize {
    plays.sort_by(|play1, play2| {
        if play1.hand_value.ne(&play2.hand_value) {
            return play1.hand_value.cmp(&play2.hand_value);
        }
        for (index, card_hand_1) in play1.hand.iter().enumerate() {
            let card_hand_2 = play2.hand.get(index).unwrap();
            if card_hand_1.ne(card_hand_2) {
                return card_hand_1.cmp(card_hand_2);
            }
        }
        std::cmp::Ordering::Equal
    });
    let mut total_winnings = 0;
    for (index, play) in plays.iter().enumerate() {
        total_winnings += play.bid * (index + 1);
    }
    total_winnings
}

fn solution_2(mut plays: Vec<Play>) -> usize {
    plays.sort_by(|play1, play2| {
        if play1.hand_value.ne(&play2.hand_value) {
            return play1.hand_value.cmp(&play2.hand_value);
        }
        for (index, card_hand_1) in play1.hand.iter().enumerate() {
            let card_hand_2 = play2.hand.get(index).unwrap();
            if card_hand_1.ne(card_hand_2) {
                return card_hand_1.cmp(card_hand_2);
            }
        }
        std::cmp::Ordering::Equal
    });
    let mut total_winnings = 0;
    for (index, play) in plays.iter().enumerate() {
        total_winnings += play.bid * (index + 1);
    }
    total_winnings
}

#[cfg(test)]
mod tests {
    use crate::shared::file_parser::{get_input, get_rows};

    use super::*;

    #[test]
    fn test_parser() {
        let input = "32T3K 765";
        let play = Play::from(input);
        assert_eq!(765, play.bid);
        assert_eq!(&3, play.hand.get(0).unwrap());
        assert_eq!(&2, play.hand.get(1).unwrap());
        assert_eq!(&10, play.hand.get(2).unwrap());
        assert_eq!(&3, play.hand.get(3).unwrap());
        assert_eq!(&13, play.hand.get(4).unwrap());
    }

    #[test]
    fn test_hand_value() {
        assert_eq!(HandValue::FiveOfAKind, hand_value_1(&vec![3, 3, 3, 3, 3])); // five of a kind
        assert_eq!(HandValue::FourOfAKind, hand_value_1(&vec![3, 7, 3, 3, 3])); // four of a kind
        assert_eq!(HandValue::FullHouse, hand_value_1(&vec![3, 12, 3, 12, 3])); // full house
        assert_eq!(HandValue::ThreeOfAKind, hand_value_1(&vec![2, 3, 3, 3, 9])); // three of a kind
        assert_eq!(HandValue::TwoPair, hand_value_1(&vec![3, 14, 13, 14, 13])); // two pair
        assert_eq!(HandValue::Pair, hand_value_1(&vec![2, 10, 3, 10, 13])); // pair
        assert_eq!(HandValue::HighCard, hand_value_1(&vec![3, 14, 8, 2, 7])); // high ace
        assert_eq!(HandValue::HighCard, hand_value_1(&vec![13, 3, 7, 4, 2])); // high king
        assert_eq!(HandValue::HighCard, hand_value_1(&vec![6, 12, 3, 7, 8])); // high queen
        assert_eq!(HandValue::HighCard, hand_value_1(&vec![6, 3, 11, 8, 4])); // high jack
        assert_eq!(HandValue::HighCard, hand_value_1(&vec![5, 2, 6, 3, 10])); // high 10
        assert_eq!(HandValue::HighCard, hand_value_1(&vec![4, 3, 7, 9, 5])); // high 9
    }

    #[test]
    fn test_hand_value_2() {
        assert_eq!(HandValue::FiveOfAKind, hand_value_2(&vec![1, 1, 1, 1, 1])); // five of a kind
        assert_eq!(HandValue::FiveOfAKind, hand_value_2(&vec![3, 1, 3, 3, 3])); // four of a kind
        assert_eq!(HandValue::FourOfAKind, hand_value_2(&vec![1, 3, 3, 3, 9])); // three of a kind
        assert_eq!(HandValue::FullHouse, hand_value_2(&vec![1, 14, 13, 14, 13])); // two pair
        assert_eq!(
            HandValue::ThreeOfAKind,
            hand_value_2(&vec![1, 10, 3, 10, 13])
        ); // pair
        assert_eq!(HandValue::Pair, hand_value_2(&vec![3, 14, 8, 1, 7])); // high ace
        assert_eq!(HandValue::Pair, hand_value_2(&vec![13, 1, 7, 4, 2])); // high king
        assert_eq!(HandValue::Pair, hand_value_2(&vec![6, 1, 3, 7, 8])); // high queen
        assert_eq!(HandValue::Pair, hand_value_2(&vec![6, 1, 11, 8, 4])); // high jack
        assert_eq!(HandValue::Pair, hand_value_2(&vec![5, 2, 1, 3, 10])); // high 10
        assert_eq!(HandValue::Pair, hand_value_2(&vec![4, 3, 1, 9, 5])); // high 9

        assert_eq!(HandValue::Pair, hand_value_2(&vec![3, 2, 10, 3, 13]));
        assert_eq!(HandValue::TwoPair, hand_value_2(&vec![13, 13, 6, 7, 7]));
        assert_eq!(HandValue::FourOfAKind, hand_value_2(&vec![10, 5, 5, 1, 5]));
        assert_eq!(
            HandValue::FourOfAKind,
            hand_value_2(&vec![13, 10, 1, 1, 10])
        );
        assert_eq!(
            HandValue::FourOfAKind,
            hand_value_2(&vec![12, 12, 12, 1, 14])
        );
    }

    #[test]
    fn example_1_test() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let plays = rows
            .into_iter()
            .map(|row| Play::from(row.as_str()))
            .collect();
        assert_eq!(6440, solution_1(plays));
    }

    #[test]
    fn solution_1_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let plays = rows
            .into_iter()
            .map(|row| Play::from(row.as_str()))
            .collect();
        assert_eq!(253205868, solution_1(plays));
    }

    #[test]
    fn example_2_test() {
        let input = get_input(file!(), "example1.txt");
        let rows = get_rows(input);
        let plays = rows
            .into_iter()
            .map(|row| Play::from_2(row.as_str()))
            .collect();
        assert_eq!(5905, solution_2(plays));
    }

    #[test]
    fn solution_2_test() {
        let input = get_input(file!(), "input1.txt");
        let rows = get_rows(input);
        let plays = rows
            .into_iter()
            .map(|row| Play::from_2(row.as_str()))
            .collect();
        assert_eq!(5905, solution_2(plays));
    }
}
