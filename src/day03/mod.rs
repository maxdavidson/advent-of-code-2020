fn walk_forest(forest: &str, right: usize, down: usize) -> impl Iterator<Item = u8> + '_ {
    forest
        .lines()
        .step_by(down)
        .zip((0..).step_by(right))
        .map(|(line, i)| line.as_bytes()[i % line.len()])
}

fn count_trees(forest: &str, right: usize, down: usize) -> usize {
    walk_forest(forest, right, down)
        .filter(|b| *b == b'#')
        .count()
}

pub fn part1(input: &str) -> usize {
    count_trees(input, 3, 1)
}

pub fn part2(input: &str) -> usize {
    count_trees(input, 1, 1)
        * count_trees(input, 3, 1)
        * count_trees(input, 5, 1)
        * count_trees(input, 7, 1)
        * count_trees(input, 1, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 7);
        assert_eq!(part1(INPUT), 270);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 336);
        assert_eq!(part2(INPUT), 2_122_848_000);
    }
}
