use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

struct Data<'a>(pub HashMap<&'a str, HashMap<&'a str, usize>>);

impl<'a> Data<'a> {
    fn parse(input: &'a str) -> Self {
        lazy_static! {
            static ref RE_1: Regex = Regex::new(r"^(?P<color>[a-z ]+) bags? contain").unwrap();
            static ref RE_2: Regex =
                Regex::new(r"(?P<count>\d+) (?P<color>[a-z ]+) bags?").unwrap();
        }

        Data(
            input
                .lines()
                .map(|line| {
                    let color = RE_1.captures(line).unwrap().name("color").unwrap().as_str();
                    let color_counts = RE_2
                        .captures_iter(line)
                        .map(|c| {
                            let color = c.name("color").unwrap().as_str();
                            let count = c.name("count").unwrap().as_str().parse().unwrap();
                            (color, count)
                        })
                        .collect();
                    (color, color_counts)
                })
                .collect(),
        )
    }
}

pub fn part1(input: &str) -> usize {
    let data = Data::parse(input);

    impl<'a> Data<'a> {
        fn contains_shiny_gold(&self, color: &'a str) -> bool {
            self.0
                .get(color)
                .map(|color_counts| {
                    color_counts.contains_key("shiny gold")
                        || color_counts
                            .keys()
                            .any(|color| self.contains_shiny_gold(color))
                })
                .unwrap_or(false)
        }
    }

    data.0
        .keys()
        .filter(|key| data.contains_shiny_gold(key))
        .count()
}

pub fn part2(input: &str) -> usize {
    let data = Data::parse(input);

    impl<'a> Data<'a> {
        fn count_bags(&self, color: &'a str) -> usize {
            self.0
                .get(color)
                .map(|color_counts| {
                    color_counts
                        .iter()
                        .map(|(color, count)| count * (1 + self.count_bags(color)))
                        .sum()
                })
                .unwrap_or(0)
        }
    }

    data.count_bags("shiny gold")
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 4);
        assert_eq!(part1(INPUT), 211);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 32);
        assert_eq!(part2(INPUT), 12_414);
    }
}
