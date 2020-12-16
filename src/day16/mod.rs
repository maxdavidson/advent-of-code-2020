use std::{mem, ops::RangeInclusive};

use lazy_static::lazy_static;
use regex::Regex;

type Ticket = Box<[usize]>;
type RulePosition = usize;

#[derive(Debug, Clone)]
struct Rule<'a> {
    name: &'a str,
    ranges: [RangeInclusive<usize>; 2],
}

impl Rule<'_> {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn matches(&self, value: usize) -> bool {
        self.ranges.iter().any(|range| range.contains(&value))
    }
}

#[derive(Copy, Clone)]
struct Bitset(usize);

impl std::fmt::Debug for Bitset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:064b}", self.0)
    }
}

impl Bitset {
    pub fn new() -> Self {
        Bitset(0)
    }

    pub fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn inversed(self) -> Self {
        Self(!self.0)
    }

    pub fn iter(self) -> impl Iterator<Item = usize> {
        (0..8 * mem::size_of::<u64>()).filter(move |val| self.contains(*val))
    }

    pub fn contains(self, val: usize) -> bool {
        self.0 & (1 << val) != 0
    }

    pub fn insert(&mut self, val: usize) {
        self.0 |= 1 << val;
    }

    pub fn intersect(&mut self, val: Self) {
        self.0 &= val.0;
    }
}

#[derive(Debug)]
struct Notes<'a> {
    pub rules: Vec<Rule<'a>>,
    pub my_ticket: Ticket,
    pub nearby_tickets: Vec<Ticket>,
}

impl<'a> From<&'a str> for Notes<'a> {
    fn from(input: &'a str) -> Self {
        lazy_static! {
            static ref RULES_RE: Regex = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
            static ref TICKET_RE: Regex = Regex::new(r"^(?:\d+,?)+$").unwrap();
        }

        let rules = RULES_RE
            .captures_iter(input)
            .map(|caps| Rule {
                name: caps.get(1).unwrap().as_str(),
                ranges: [
                    caps[2].parse().unwrap()..=caps[3].parse().unwrap(),
                    caps[4].parse().unwrap()..=caps[5].parse().unwrap(),
                ],
            })
            .collect();

        let mut tickets = input
            .lines()
            .filter(|line| TICKET_RE.is_match(line))
            .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect());

        let my_ticket = tickets.next().unwrap();
        let nearby_tickets = tickets.collect();

        Self {
            rules,
            my_ticket,
            nearby_tickets,
        }
    }
}

pub fn part1(input: &str) -> usize {
    let notes = Notes::from(input);

    notes
        .nearby_tickets
        .iter()
        .flat_map(|ticket| ticket.iter().copied())
        .filter(|value| !notes.rules.iter().any(|rule| rule.matches(*value)))
        .sum()
}

fn find_rule_positions(
    rules: &[Rule],
    allowed_rules_per_position: &[Bitset],
    visited_rules: Vec<RulePosition>,
) -> Option<Vec<RulePosition>> {
    let current_position = visited_rules.len();

    if current_position == rules.len() {
        Some(visited_rules)
    } else {
        allowed_rules_per_position[current_position]
            .iter()
            .find_map(|rule| {
                if !visited_rules.contains(&rule) {
                    let mut visited_rules = visited_rules.clone();
                    visited_rules.push(rule);

                    find_rule_positions(rules, allowed_rules_per_position, visited_rules)
                } else {
                    None
                }
            })
    }
}

pub fn part2(input: &str) -> usize {
    let Notes {
        rules,
        my_ticket,
        nearby_tickets,
    } = Notes::from(input);

    let valid_nearby_tickets: Vec<_> = nearby_tickets
        .into_iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|value| rules.iter().any(|rule| rule.matches(*value)))
        })
        .collect();

    let mut allowed_rules_per_position: Vec<_> = (0..my_ticket.len())
        .map(|position| {
            let mut allowed_rules = Bitset::new();

            for (rule_index, rule) in rules.iter().enumerate() {
                if valid_nearby_tickets
                    .iter()
                    .all(|ticket| rule.matches(ticket[position]))
                {
                    allowed_rules.insert(rule_index);
                }
            }

            allowed_rules
        })
        .collect();

    let mut visited_positions = Bitset::new();

    while let Some((position, allowed_rules)) = allowed_rules_per_position
        .iter()
        .copied()
        .enumerate()
        .find(|(position, rules)| rules.len() == 1 && !visited_positions.contains(*position))
    {
        visited_positions.insert(position);

        for (other_position, other_allowed_rules) in
            allowed_rules_per_position.iter_mut().enumerate()
        {
            if other_position != position {
                other_allowed_rules.intersect(allowed_rules.inversed());
            }
        }
    }

    let rule_positions = find_rule_positions(&rules, &allowed_rules_per_position, Vec::new())
        .expect("No rule permutation found!");

    my_ticket
        .iter()
        .enumerate()
        .map(|(rule_index, value)| (rules[rule_positions[rule_index]].name(), *value))
        .filter_map(|(rule_name, value)| {
            if rule_name.starts_with("departure") {
                Some(value)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_0: &str = include_str!("test_input_0.txt");
    static TEST_INPUT_1: &str = include_str!("test_input_1.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT_0), 71);
        assert_eq!(part1(INPUT), 26_988);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT_1), 1);
        assert_eq!(part2(INPUT), 426_362_917_709);
    }
}
