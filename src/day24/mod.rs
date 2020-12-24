use std::{collections::HashSet, mem};

#[derive(Debug, Copy, Clone)]
enum Direction {
    East,
    West,
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
}

const ALL_DIRECTIONS: [Direction; 6] = [
    Direction::East,
    Direction::West,
    Direction::SouthEast,
    Direction::SouthWest,
    Direction::NorthEast,
    Direction::NorthWest,
];

impl Direction {
    pub fn step(self, (x, y): (i16, i16)) -> (i16, i16) {
        match self {
            Self::East => (x + 1, y),
            Self::West => (x - 1, y),
            Self::SouthEast => (x, y + 1),
            Self::SouthWest => (x - 1, y + 1),
            Self::NorthEast => (x + 1, y - 1),
            Self::NorthWest => (x, y - 1),
        }
    }
}

fn parse_line(mut input: &str) -> impl Iterator<Item = Direction> + '_ {
    std::iter::from_fn(move || {
        if let Some(remaining_input) = input.strip_prefix("e") {
            input = remaining_input;
            Some(Direction::East)
        } else if let Some(remaining_input) = input.strip_prefix("w") {
            input = remaining_input;
            Some(Direction::West)
        } else if let Some(remaining_input) = input.strip_prefix("se") {
            input = remaining_input;
            Some(Direction::SouthEast)
        } else if let Some(remaining_input) = input.strip_prefix("sw") {
            input = remaining_input;
            Some(Direction::SouthWest)
        } else if let Some(remaining_input) = input.strip_prefix("ne") {
            input = remaining_input;
            Some(Direction::NorthEast)
        } else if let Some(remaining_input) = input.strip_prefix("nw") {
            input = remaining_input;
            Some(Direction::NorthWest)
        } else {
            None
        }
    })
}

fn get_black_tiles(input: &str) -> HashSet<(i16, i16)> {
    let mut black_tiles = HashSet::new();

    for line in input.lines() {
        let pos = parse_line(line).fold((0, 0), |pos, dir| dir.step(pos));

        if !black_tiles.remove(&pos) {
            black_tiles.insert(pos);
        }
    }

    black_tiles
}

pub fn part1(input: &str) -> usize {
    let black_tiles = get_black_tiles(input);

    black_tiles.len()
}

pub fn part2(input: &str) -> usize {
    let mut black_tiles = get_black_tiles(input);
    let mut next_black_tiles = HashSet::new();
    let mut white_neighbor_tiles = HashSet::new();

    for _ in 0..100 {
        for pos in black_tiles.iter().copied() {
            let mut black_neighbor_count = 0;

            for dir in ALL_DIRECTIONS.iter() {
                let neighbor_pos = dir.step(pos);

                if black_tiles.contains(&neighbor_pos) {
                    black_neighbor_count += 1;
                } else {
                    white_neighbor_tiles.insert(neighbor_pos);
                }
            }

            if black_neighbor_count == 1 || black_neighbor_count == 2 {
                next_black_tiles.insert(pos);
            }
        }

        for pos in white_neighbor_tiles.iter().copied() {
            let mut black_neighbor_count = 0;

            for dir in ALL_DIRECTIONS.iter() {
                let neighbor_pos = dir.step(pos);

                if black_tiles.contains(&neighbor_pos) {
                    black_neighbor_count += 1;
                }
            }

            if black_neighbor_count == 2 {
                next_black_tiles.insert(pos);
            }
        }

        mem::swap(&mut black_tiles, &mut next_black_tiles);
        next_black_tiles.clear();
        white_neighbor_tiles.clear();
    }

    black_tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 10);
        assert_eq!(part1(INPUT), 394);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 2208);
        assert_eq!(part2(INPUT), 4036);
    }
}
