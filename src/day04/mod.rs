use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn parse_passports(input: &str) -> impl Iterator<Item = HashMap<&str, &str>> + '_ {
    input.split("\n\n").map(|chunks| {
        chunks
            .split_whitespace()
            .map(|chunk| {
                let mut it = chunk.splitn(2, ':');
                (it.next().unwrap(), it.next().unwrap())
            })
            .collect()
    })
}

const VALID_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

const VALID_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub fn part1(input: &str) -> usize {
    parse_passports(input)
        .filter(|passport| {
            VALID_FIELDS
                .iter()
                .all(|field| passport.get(*field).is_some())
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    parse_passports(input)
        .filter(|passport| {
            VALID_FIELDS.into_iter().all(|field| {
                if let Some(value) = passport.get(field) {
                    match field {
                        "byr" => {
                            let num_value = value.parse().unwrap();
                            (1920..=2002).contains(&num_value)
                        }
                        "iyr" => {
                            let num_value = value.parse().unwrap();
                            (2010..=2020).contains(&num_value)
                        }
                        "eyr" => {
                            let num_value = value.parse().unwrap();
                            (2020..=2030).contains(&num_value)
                        }
                        "hgt" => {
                            lazy_static! {
                                static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
                            }
                            if let Some(cap) = RE.captures(value) {
                                let height = cap[1].parse().unwrap();
                                let unit = &cap[2];
                                match unit {
                                    "cm" => (150..=193).contains(&height),
                                    "in" => (59..=76).contains(&height),
                                    _ => panic!("Invalid unit: {}", unit),
                                }
                            } else {
                                false
                            }
                        }
                        "hcl" => {
                            lazy_static! {
                                static ref RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
                            }
                            RE.is_match(value)
                        }
                        "ecl" => VALID_EYE_COLORS.contains(value),
                        "pid" => {
                            lazy_static! {
                                static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
                            }
                            RE.is_match(value)
                        }
                        _ => panic!("Invalid field: {field}"),
                    }
                } else {
                    false
                }
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static TEST_INPUT_2: &str = include_str!("test_input_2.txt");
    static TEST_INPUT_3: &str = include_str!("test_input_3.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 2);
        assert_eq!(part1(INPUT), 192);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT_2), 0);
        assert_eq!(part2(TEST_INPUT_3), 4);
        assert_eq!(part2(INPUT), 101);
    }
}
