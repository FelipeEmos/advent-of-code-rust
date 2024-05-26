use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let mut items = line.split(&[':', '|']);
            let id: u32 = items
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            let winning_numbers: Vec<u32> = items
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            let card_numbers: Vec<u32> = items
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            Card {
                id,
                winning_numbers,
                card_numbers,
            }
        })
        .collect()
}

fn count_winning(card: &Card) -> u32 {
    card.card_numbers
        .iter()
        .filter(|n| card.winning_numbers.contains(n))
        .count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse_input(input);
    let sum = games
        .iter()
        .map(|card| match count_winning(card) {
            0 => 0,
            win_count => (2 as u32).pow(win_count as u32 - 1),
        })
        .sum::<u32>();

    println!("SUM {:?}", sum);

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let all_cards = parse_input(input);
    let mut copies_count: HashMap<u32, u32> = HashMap::new();

    all_cards.iter().for_each(|card| {
        let card_copies = *copies_count.get(&card.id).unwrap_or(&0);
        for new_card_id in (1 + card.id)..(1 + card.id + count_winning(card)) {
            copies_count
                .entry(new_card_id)
                .and_modify(|existing| *existing += card_copies + 1)
                .or_insert(card_copies + 1);
        }
    });

    let sum: u32 = copies_count.values().sum::<u32>() + all_cards.len() as u32;
    println!("COPIES_COUNT {:?}", copies_count.values().sum::<u32>());
    println!("ALL_CARDS {:?}", all_cards.len());

    println!("SUM {:?}", sum);
    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
