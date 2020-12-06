use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut it = group
                .trim()
                .split('\n')
                .map(|line| line.chars().collect::<HashSet<_>>());

            let mut intersection = it.next().unwrap();

            for chars in it {
                intersection.retain(|c| chars.contains(c));
            }

            intersection.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 11);
        assert_eq!(part1(INPUT), 6565);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 6);
        assert_eq!(part2(INPUT), 3137);
    }
}
