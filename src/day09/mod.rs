use itertools::Itertools;

fn find_first_invalid_number(numbers: &[u64], preamble_length: usize) -> Option<u64> {
    numbers.windows(preamble_length + 1).find_map(|window| {
        let (last, preamble) = window.split_last()?;
        let valid = preamble
            .iter()
            .tuple_combinations()
            .any(|(a, b)| a != b && a + b == *last);
        if !valid {
            Some(*last)
        } else {
            None
        }
    })
}

pub fn part1(input: &str, preamble_length: usize) -> u64 {
    let numbers: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();

    find_first_invalid_number(&numbers, preamble_length).expect("No invalid number found")
}

pub fn part2(input: &str, preamble_length: usize) -> u64 {
    let numbers: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let sums: Vec<u64> = numbers
        .iter()
        .scan(0, |sum, num| {
            *sum += num;
            Some(*sum)
        })
        .collect();

    let first_invalid_number =
        find_first_invalid_number(&numbers, preamble_length).expect("No invalid number found");

    for i in 0..numbers.len() - 1 {
        let mut start = i;
        let mut end = numbers.len() - 1;

        while start <= end {
            let mid = (start + end) / 2;
            let sum = sums[mid] - sums[start];

            match sum.cmp(&first_invalid_number) {
                std::cmp::Ordering::Less => {
                    start = mid + 1;
                }
                std::cmp::Ordering::Greater => {
                    end = mid - 1;
                }
                std::cmp::Ordering::Equal => {
                    let (min, max) = numbers[i..=mid].iter().minmax().into_option().unwrap();
                    return min + max;
                }
            }
        }
    }

    panic!("No sequence found");
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT, 5), 127);
        assert_eq!(part1(INPUT, 25), 25_918_798);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT, 5), 62);
        assert_eq!(part2(INPUT, 25), 3_340_942);
    }
}
