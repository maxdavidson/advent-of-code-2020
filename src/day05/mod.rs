use itertools::Itertools;
use std::collections::HashMap;

type BoardingPass = (usize, usize);

fn boarding_pass_id((row, column): BoardingPass) -> usize {
    row * 8 + column
}

fn boarding_pass(input: &str) -> BoardingPass {
    let mut row_from = 0;
    let mut row_to = 127;

    let mut column_from = 0;
    let mut column_to = 7;

    for c in input.chars() {
        match c {
            'B' => row_from = (row_to + row_from + 1) / 2,
            'F' => row_to = (row_to + row_from) / 2,
            'R' => column_from = (column_from + column_to + 1) / 2,
            'L' => column_to = (column_from + column_to) / 2,
            _ => {}
        }
    }

    (row_to, column_to)
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(boarding_pass)
        .map(boarding_pass_id)
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let boarding_passes_by_id: HashMap<_, _> = input
        .lines()
        .map(|input| {
            let pass = boarding_pass(input);
            (boarding_pass_id(pass), pass)
        })
        .collect();

    let (first_row, last_row) = boarding_passes_by_id
        .values()
        .map(|(row, _)| row)
        .minmax()
        .into_option()
        .unwrap();

    for row in (first_row + 1)..=(last_row - 1) {
        for column in 0..=7 {
            let id = boarding_pass_id((row, column));
            if !boarding_passes_by_id.contains_key(&id)
                && (boarding_passes_by_id.contains_key(&(id - 1))
                    || boarding_passes_by_id.contains_key(&(id + 1)))
            {
                return id;
            }
        }
    }

    panic!("No solution found")
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(boarding_pass_id(boarding_pass("FBFBBFFRLR")), 357);
        assert_eq!(boarding_pass_id(boarding_pass("BFFFBBFRRR")), 567);
        assert_eq!(boarding_pass_id(boarding_pass("FFFBBBFRRR")), 119);
        assert_eq!(boarding_pass_id(boarding_pass("BBFFBBFRLL")), 820);
        assert_eq!(part1(INPUT), 935);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 743)
    }
}
