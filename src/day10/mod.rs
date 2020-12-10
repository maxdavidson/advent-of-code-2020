use std::{cell::RefCell, collections::HashMap};

fn adapters(input: &str) -> Box<[u64]> {
    let mut numbers: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    numbers.push(0);
    numbers.push(numbers.iter().max().unwrap() + 3);
    numbers.sort_unstable();
    numbers.into()
}

pub fn part1(input: &str) -> u64 {
    let numbers = adapters(input);

    let mut groups = HashMap::new();
    let mut it = numbers.windows(2);
    while let Some(&[a, b]) = it.next() {
        let diff = b - a;
        *groups.entry(diff).or_default() += 1;
    }

    groups.values().product()
}

struct Part2Helper {
    cache: RefCell<HashMap<u64, u64>>,
    successors: HashMap<u64, Vec<u64>>,
}

impl Part2Helper {
    fn path_count_from_node(&self, node: u64) -> u64 {
        if let Some(count) = self.cache.borrow().get(&node) {
            return *count;
        }

        if let Some(nodes) = self.successors.get(&node) {
            let count = nodes
                .iter()
                .rfold(0, |count, node| count + self.path_count_from_node(*node));

            self.cache.borrow_mut().insert(node, count);

            count
        } else {
            1
        }
    }
}

pub fn part2(input: &str) -> u64 {
    let numbers = adapters(input);

    let successors: HashMap<u64, Vec<u64>> = numbers
        .windows(4)
        .map(|window| {
            let (node, rest) = window.split_first().unwrap();
            (
                *node,
                rest.iter()
                    .copied()
                    .filter(|node2| node2 - node <= 3)
                    .collect(),
            )
        })
        .collect();

    let part2_helper = Part2Helper {
        successors,
        cache: RefCell::new(HashMap::new()),
    };

    part2_helper.path_count_from_node(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_0: &str = include_str!("test_input_0.txt");
    static TEST_INPUT_1: &str = include_str!("test_input_1.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT_0), 7 * 5);
        assert_eq!(part1(TEST_INPUT_1), 22 * 10);
        assert_eq!(part1(INPUT), 1836);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT_0), 8);
        assert_eq!(part2(TEST_INPUT_1), 19_208);
        assert_eq!(part2(INPUT), 43_406_276_662_336);
    }
}
