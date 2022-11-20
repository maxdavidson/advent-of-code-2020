use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Entry<'a> {
    a: usize,
    b: usize,
    letter: char,
    password: &'a str,
}

fn parse_entries(input: &str) -> impl Iterator<Item = Entry<'_>> + '_ {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<a>\d+)-(?P<b>\d+) (?P<letter>\w): (?P<password>\w+)").unwrap();
    }

    RE.captures_iter(input).map(|c| Entry {
        a: c.name("a").unwrap().as_str().parse().unwrap(),
        b: c.name("b").unwrap().as_str().parse().unwrap(),
        letter: c.name("letter").unwrap().as_str().chars().next().unwrap(),
        password: c.name("password").unwrap().as_str(),
    })
}

pub fn part1(input: &str) -> usize {
    parse_entries(input)
        .filter(|entry| {
            let count = entry
                .password
                .chars()
                .filter(|char| *char == entry.letter)
                .count();

            count >= entry.a && count <= entry.b
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    parse_entries(input)
        .filter(|entry| {
            let match_a = entry.letter == entry.password.chars().nth(entry.a - 1).unwrap();
            let match_b = entry.letter == entry.password.chars().nth(entry.b - 1).unwrap();

            match_a && !match_b || !match_a && match_b
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 2);
        assert_eq!(part1(INPUT), 614);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 1);
        assert_eq!(part2(INPUT), 354);
    }
}
