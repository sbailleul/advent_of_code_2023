use std::collections::HashSet;
#[derive(Debug)]
struct Card {
    id: u16,
    instance: u32,
    winning_numbers: HashSet<u8>,
    numbers_you_have: HashSet<u8>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (card, numbers) = value.split_once(':').unwrap();
        let id = card.replace("Card", "").trim().parse().unwrap();
        let (winning_numbers, numbers_you_have) = numbers.split_once('|').unwrap();
        let numbers_to_set = |txt_numbers: &str| -> HashSet<u8> {
            txt_numbers
                .trim()
                .split(' ')
                .map(|n| n.parse::<u8>())
                .filter_map(|n| n.ok())
                .collect()
        };
        Card {
            id,
            instance: 1,
            numbers_you_have: numbers_to_set(numbers_you_have),
            winning_numbers: numbers_to_set(winning_numbers),
        }
    }
}
impl Card {
    fn matching_numbers(&self) -> u32 {
        self.numbers_you_have
            .intersection(&self.winning_numbers)
            .count() as u32
    }
    fn value(&self) -> u32 {
        let winning_matches = self.matching_numbers();
        if winning_matches > 0 {
            2u32.pow(
                self.numbers_you_have
                    .intersection(&self.winning_numbers)
                    .count() as u32
                    - 1,
            )
        } else {
            0
        }
    }
}

pub fn step_1(input_content: &str) -> String {
    let pile_value = input_content
        .lines()
        .map(|l| Card::from(l).value())
        .sum::<u32>();
    pile_value.to_string()
}
pub fn step_2(input_content: &str) -> String {
    let mut cards = input_content.lines().map(Card::from).collect::<Vec<Card>>();
    for i in 0..cards.len() {
        let matching_numbers = cards[i].matching_numbers();
        // dbg!(&cards[i].id);
        if matching_numbers > 0 {
            for j in i + 1..i + 1 + matching_numbers as usize {
                if j < cards.len() {
                    cards[j].instance += cards[i].instance;
                    // dbg!(&cards[j].id, &cards[j].instance);
                }
            }
        }
    }
    cards.iter().map(|c| c.instance).sum::<u32>().to_string()
}
#[cfg(test)]
mod tests {
    use crate::challenge_4::{step_1, step_2};

    const INPUT_CONTENT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    #[test]
    fn step_1_should_works() {
        assert_eq!(step_1(INPUT_CONTENT), "13");
    }
    #[test]
    fn step_2_should_works() {
        assert_eq!(step_2(INPUT_CONTENT), "30");
    }
}
