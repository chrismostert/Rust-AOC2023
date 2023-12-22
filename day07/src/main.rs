use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hand {
    cards: Vec<char>,
    type_score: usize,
    bet: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.type_score == other.type_score {
            for (card_self, card_other) in self.cards.iter().zip(other.cards.iter()) {
                match card_score(card_self).cmp(&card_score(card_other)) {
                    std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                    _ => (),
                }
            }
            return std::cmp::Ordering::Equal;
        }
        self.type_score.cmp(&other.type_score)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn card_score(card: &char) -> u32 {
    if card.is_numeric() {
        return card.to_digit(10).unwrap();
    }
    match card {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

fn hand_type_score(cards: &[char], with_jokers: bool) -> usize {
    let mut counts = cards.iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(card).or_insert(0) += 1;
        acc
    });

    if with_jokers {
        let n_jokers = counts.remove(&'J').unwrap_or(0);
        if n_jokers == 5 {
            counts.insert(&'A', 5);
        } else {
            *counts.iter_mut().max_by_key(|entry| *entry.1).unwrap().1 += n_jokers;
        }
    }

    match counts.values().sorted().cloned().collect_vec()[..] {
        [5] => 6,
        [1, 4] => 5,
        [2, 3] => 4,
        [1, 1, 3] => 3,
        [1, 2, 2] => 2,
        [1, 1, 1, 2] => 1,
        _ => 0,
    }
}

fn total_winnings(hands: &[Hand], with_jokers: bool) -> usize {
    hands
        .iter()
        .map(|hand| {
            if with_jokers {
                Hand {
                    cards: hand
                        .cards
                        .iter()
                        .map(|&c| {
                            if c == 'J' {
                                return 'X';
                            };
                            c
                        })
                        .collect_vec(),
                    type_score: hand_type_score(&hand.cards, true),
                    bet: hand.bet,
                }
            } else {
                hand.clone()
            }
        })
        .sorted()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bet)
        .sum::<usize>()
}
fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (cards, bet) = line.split_once(' ').unwrap();
            Hand {
                cards: cards.chars().collect(),
                type_score: 0,
                bet: bet.parse().unwrap(),
            }
        })
        .collect_vec();

    let p1 = total_winnings(&input, false);
    let p2 = total_winnings(&input, true);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
