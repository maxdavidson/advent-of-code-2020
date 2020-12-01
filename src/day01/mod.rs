use itertools::Itertools;

fn parse_lines<T: std::str::FromStr>(input: &str) -> impl Iterator<Item = T> + '_ {
    input.trim().lines().filter_map(|line| line.parse().ok())
}

pub fn part1(input: &str) -> Option<u32> {
    let nums: Vec<u32> = parse_lines(input).collect();

    nums.into_iter()
        .tuple_combinations()
        .find_map(|(a, b)| if a + b == 2020 { Some(a * b) } else { None })
}

pub fn part2(input: &str) -> Option<u32> {
    let nums: Vec<u32> = parse_lines(input).collect();

    nums.into_iter().tuple_combinations().find_map(|(a, b, c)| {
        if a + b + c == 2020 {
            Some(a * b * c)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), Some(514_579));
        assert_eq!(part1(INPUT), Some(1_019_371));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), Some(241_861_950));
        assert_eq!(part2(INPUT), Some(278_064_990));
    }
}
