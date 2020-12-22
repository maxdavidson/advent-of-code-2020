use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

type Card = usize;
type Deck = VecDeque<Card>;

fn parse_decks(input: &str) -> (Deck, Deck) {
    let mut it = input
        .split("\n\n")
        .map(|chunk| chunk.lines().filter_map(|line| line.parse().ok()).collect());

    let first = it.next().unwrap();
    let second = it.next().unwrap();

    (first, second)
}

enum GameResult {
    Player1(Deck),
    Player2(Deck),
}

impl GameResult {
    fn deck(&self) -> &Deck {
        match self {
            Self::Player1(deck) => deck,
            Self::Player2(deck) => deck,
        }
    }

    pub fn score(&self) -> usize {
        self.deck()
            .iter()
            .rev()
            .zip(1..)
            .map(|(card, i)| i * card)
            .sum()
    }
}

fn play_game(mut deck1: Deck, mut deck2: Deck) -> Option<GameResult> {
    loop {
        if !deck1.is_empty() && deck2.is_empty() {
            break Some(GameResult::Player1(deck1));
        } else if deck1.is_empty() && !deck2.is_empty() {
            break Some(GameResult::Player2(deck2));
        }

        let card1 = deck1.pop_front()?;
        let card2 = deck2.pop_front()?;

        match card1.cmp(&card2) {
            Ordering::Equal => {
                break None;
            }
            Ordering::Greater => {
                deck1.push_back(card1);
                deck1.push_back(card2);
            }
            Ordering::Less => {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }
        }
    }
}

fn play_recursive_game(mut deck1: Deck, mut deck2: Deck) -> Option<GameResult> {
    let mut seen_games: HashSet<u64> = HashSet::new();

    loop {
        let game_hash = {
            let mut hasher = DefaultHasher::new();
            Hash::hash(&(&deck1, &deck2), &mut hasher);
            hasher.finish()
        };

        if seen_games.contains(&game_hash) || !deck1.is_empty() && deck2.is_empty() {
            break Some(GameResult::Player1(deck1));
        } else if deck1.is_empty() && !deck2.is_empty() {
            break Some(GameResult::Player2(deck2));
        }

        seen_games.insert(game_hash);

        let card1 = deck1.pop_front()?;
        let card2 = deck2.pop_front()?;

        if deck1.len() >= card1 && deck2.len() >= card2 {
            match play_recursive_game(
                deck1.iter().take(card1).copied().collect(),
                deck2.iter().take(card2).copied().collect(),
            )? {
                GameResult::Player1(_) => {
                    deck1.push_back(card1);
                    deck1.push_back(card2);
                }
                GameResult::Player2(_) => {
                    deck2.push_back(card2);
                    deck2.push_back(card1);
                }
            }
        } else {
            match card1.cmp(&card2) {
                Ordering::Equal => {
                    break None;
                }
                Ordering::Greater => {
                    deck1.push_back(card1);
                    deck1.push_back(card2);
                }
                Ordering::Less => {
                    deck2.push_back(card2);
                    deck2.push_back(card1);
                }
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (deck1, deck2) = parse_decks(input);
    let result = play_game(deck1, deck2).expect("Game failed :(");

    result.score()
}

pub fn part2(input: &str) -> usize {
    let (deck1, deck2) = parse_decks(input);
    let result = play_recursive_game(deck1, deck2).expect("Game failed :(");

    result.score()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 306);
        assert_eq!(part1(INPUT), 32_083);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 291);
        assert_eq!(part2(INPUT), 35_495);
    }
}
