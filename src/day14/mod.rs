use std::{collections::HashMap, iter};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum BitMask {
    On,
    Off,
    Unchanged,
    Floating,
}

const MASK_SIZE: usize = 36;
type Mask = [BitMask; MASK_SIZE];

#[derive(Debug)]
struct Program {
    pub mask: Mask,
    pub memory_init: Box<[(u64, u64)]>,
}

fn programs(input: &str) -> impl Iterator<Item = Program> + '_ {
    lazy_static! {
        static ref MASK_RE: Regex = Regex::new(r"^mask = (?P<mask>[01X]{36})$").unwrap();
        static ref MEM_RE: Regex = Regex::new(r"^mem\[(?P<index>\d+)\] = (?P<value>\d+)$").unwrap();
    }

    let mut lines = input.lines().peekable();

    iter::from_fn(move || {
        let line = lines.next()?;
        let mask_str = MASK_RE.captures(line)?.name("mask")?.as_str();

        let mut mask = [BitMask::Off; 36];
        for (i, c) in mask_str.chars().rev().enumerate() {
            mask[i] = match c {
                '0' => BitMask::Off,
                '1' => BitMask::On,
                'X' => BitMask::Floating,
                _ => panic!("Invalid mask char!"),
            }
        }

        let mut memory_init = Vec::new();

        while let Some(caps) = lines
            .peek()
            .and_then(|next_line| MEM_RE.captures(next_line))
        {
            lines.next().unwrap();
            let index = caps.name("index")?.as_str().parse().unwrap();
            let value = caps.name("value")?.as_str().parse().unwrap();
            memory_init.push((index, value));
        }

        Some(Program {
            mask,
            memory_init: memory_init.into(),
        })
    })
}

fn apply_mask(mask: Mask, value: u64) -> u64 {
    mask.iter()
        .enumerate()
        .fold(value, |value, (i, mask)| match mask {
            BitMask::On => value | 1 << i,
            BitMask::Off => value & !(1 << i),
            BitMask::Unchanged | BitMask::Floating => value,
        })
}

pub fn part1(input: &str) -> u64 {
    let mut memory = HashMap::new();

    for program in programs(input) {
        for (address, value) in program.memory_init.iter().copied() {
            let masked_value = apply_mask(program.mask, value);
            memory.insert(address, masked_value);
        }
    }

    memory.values().sum()
}

fn for_each_mask(mask: Mask, mut apply: impl FnMut(Mask)) {
    fn for_each_mask_from_index(mask: Mask, index: usize, apply: &mut impl FnMut(Mask)) {
        if index == MASK_SIZE {
            apply(mask);
        } else {
            match mask[index] {
                BitMask::On => {
                    for_each_mask_from_index(mask, index + 1, apply);
                }
                BitMask::Off => {
                    let mut mask = mask;
                    mask[index] = BitMask::Unchanged;
                    for_each_mask_from_index(mask, index + 1, apply);
                }
                BitMask::Floating => {
                    let mut mask_on = mask;
                    mask_on[index] = BitMask::On;
                    for_each_mask_from_index(mask_on, index + 1, apply);

                    let mut mask_off = mask;
                    mask_off[index] = BitMask::Off;
                    for_each_mask_from_index(mask_off, index + 1, apply);
                }
                BitMask::Unchanged => {
                    panic!("Unexpected unchanged bitmask");
                }
            }
        }
    }

    for_each_mask_from_index(mask, 0, &mut apply)
}

pub fn part2(input: &str) -> u64 {
    let mut memory = HashMap::new();

    for program in programs(input) {
        for (address, value) in program.memory_init.iter().copied() {
            for_each_mask(program.mask, |mask| {
                let masked_address = apply_mask(mask, address);
                memory.insert(masked_address, value);
            });
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_0: &str = include_str!("test_input_0.txt");
    static TEST_INPUT_1: &str = include_str!("test_input_1.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT_0), 165);
        assert_eq!(part1(INPUT), 9_615_006_043_476);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT_1), 208);
        assert_eq!(part2(INPUT), 4_275_496_544_925);
    }
}
