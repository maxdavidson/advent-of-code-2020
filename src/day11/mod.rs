use itertools::Itertools;
use std::{collections::HashMap, ops::Range};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Seat {
    Empty,
    Occupied,
}

#[derive(Eq, PartialEq)]
struct SeatMap(HashMap<(i16, i16), Seat>);

impl<'a> From<&'a str> for SeatMap {
    fn from(input: &'a str) -> Self {
        Self(
            input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, c)| {
                        let pos = (y as i16, x as i16);
                        match c {
                            'L' => Some((pos, Seat::Empty)),
                            '#' => Some((pos, Seat::Occupied)),
                            _ => None,
                        }
                    })
                })
                .collect(),
        )
    }
}

impl ToString for SeatMap {
    fn to_string(&self) -> String {
        let (x_range, y_range) = self.bounds().unwrap();

        y_range
            .flat_map(|y| {
                x_range
                    .clone()
                    .map(move |x| match self.0.get(&(x, y)) {
                        Some(Seat::Empty) => 'L',
                        Some(Seat::Occupied) => '#',
                        None => '.',
                    })
                    .chain(std::iter::once('\n'))
            })
            .collect()
    }
}

impl SeatMap {
    fn bounds(&self) -> Option<(Range<i16>, Range<i16>)> {
        self.iter().fold(None, |result, ((x, y), _)| {
            if let Some((x_range, y_range)) = result {
                Some((
                    x_range.start.min(x)..x_range.end.max(x + 1),
                    y_range.start.min(y)..y_range.end.max(y + 1),
                ))
            } else {
                Some((x..x + 1, y..y + 1))
            }
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = ((i16, i16), Seat)> + '_ {
        self.0.iter().map(|(pos, seat)| (*pos, *seat))
    }

    pub fn adjacent_seats(&self, (x, y): (i16, i16)) -> impl Iterator<Item = Seat> + '_ {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter_map(move |(x_d, y_d)| {
                if let (0, 0) = (x_d, y_d) {
                    None
                } else {
                    self.0.get(&(x + x_d, y + y_d)).copied()
                }
            })
    }

    pub fn visible_seats(&self, (x, y): (i16, i16)) -> impl Iterator<Item = Seat> + '_ {
        let (x_range, y_range) = self.bounds().unwrap();

        (-1..=1)
            .cartesian_product(-1..=1)
            .flat_map(move |(x_d, y_d)| {
                if let (0, 0) = (x_d, y_d) {
                    None
                } else {
                    (1..)
                        .map(|n| (x + n * x_d, y + n * y_d))
                        .take_while(|(x, y)| x_range.contains(x) && y_range.contains(y))
                        .find_map(|pos| self.0.get(&pos).copied())
                }
            })
    }

    pub fn transformed(&self, rules: impl Fn(Seat, (i16, i16)) -> Seat) -> Self {
        Self(
            self.iter()
                .map(|(pos, seat)| (pos, rules(seat, pos)))
                .collect(),
        )
    }
}

pub fn part1(input: &str) -> usize {
    let mut seats = SeatMap::from(input);

    loop {
        let next_seats = seats.transformed(|seat, pos| match seat {
            Seat::Empty
                if !seats
                    .adjacent_seats(pos)
                    .any(|seat| matches!(seat, Seat::Occupied)) =>
            {
                Seat::Occupied
            }
            Seat::Occupied
                if 4 <= seats
                    .adjacent_seats(pos)
                    .filter(|seat| matches!(seat, Seat::Occupied))
                    .count() =>
            {
                Seat::Empty
            }
            seat => seat,
        });

        if seats == next_seats {
            break seats
                .iter()
                .filter(|(_, seat)| matches!(seat, Seat::Occupied))
                .count();
        }

        seats = next_seats;
    }
}

pub fn part2(input: &str) -> usize {
    let mut seats = SeatMap::from(input);

    loop {
        let next_seats = seats.transformed(|seat, pos| match seat {
            Seat::Empty
                if !seats
                    .visible_seats(pos)
                    .any(|seat| matches!(seat, Seat::Occupied)) =>
            {
                Seat::Occupied
            }
            Seat::Occupied
                if 5 <= seats
                    .visible_seats(pos)
                    .filter(|seat| matches!(seat, Seat::Occupied))
                    .count() =>
            {
                Seat::Empty
            }
            seat => seat,
        });

        if seats == next_seats {
            break seats
                .iter()
                .filter(|(_, seat)| matches!(seat, Seat::Occupied))
                .count();
        }

        seats = next_seats;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 37);
        assert_eq!(part1(INPUT), 2368);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 26);
        assert_eq!(part2(INPUT), 2124);
    }
}
