#[derive(Clone, Copy, Eq, PartialEq)]
enum Seat {
    Empty,
    Occupied,
}

#[derive(Eq, PartialEq)]
struct SeatMap {
    tiles: Box<[Option<Seat>]>,
    column_count: usize,
}

impl<'a> From<&'a str> for SeatMap {
    fn from(input: &'a str) -> Self {
        let tiles: Vec<_> = input
            .lines()
            .flat_map(|line| {
                line.chars().map(move |c| match c {
                    'L' => Some(Seat::Empty),
                    '#' => Some(Seat::Occupied),
                    '.' => None,
                    _ => panic!("Invalid char: {}", c),
                })
            })
            .collect();

        let row_count = input.lines().count();
        let column_count = tiles.len() / row_count;

        Self {
            tiles: tiles.into(),
            column_count,
        }
    }
}

impl ToString for SeatMap {
    fn to_string(&self) -> String {
        self.tiles
            .chunks_exact(self.column_count)
            .flat_map(|line| {
                line.iter()
                    .map(|tile| match tile {
                        Some(Seat::Empty) => 'L',
                        Some(Seat::Occupied) => '#',
                        None => '.',
                    })
                    .chain(std::iter::once('\n'))
            })
            .collect()
    }
}

const DIFFS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl SeatMap {
    pub fn adjacent_occupied_seat_count(&self, pos: usize) -> usize {
        let column_count = self.column_count as isize;
        let row_count = (self.tiles.len() as isize) / column_count;

        let x = (pos as isize) % column_count;
        let y = (pos as isize) / column_count;

        DIFFS
            .iter()
            .filter(|(dx, dy)| {
                let x = x + dx;
                let y = y + dy;
                if 0 <= x && x < column_count && 0 <= y && y < row_count {
                    let pos = (y * column_count + x) as usize;
                    let seat = self.tiles[pos];
                    matches!(seat, Some(Seat::Occupied))
                } else {
                    false
                }
            })
            .count()
    }

    pub fn visible_occupied_seat_count(&self, pos: usize) -> usize {
        let column_count = self.column_count as isize;
        let row_count = (self.tiles.len() as isize) / column_count;

        let x = (pos as isize) % column_count;
        let y = (pos as isize) / column_count;

        DIFFS
            .iter()
            .filter(|(dx, dy)| {
                let first_seat = (1..)
                    .map(|n| (x + n * dx, y + n * dy))
                    .take_while(|(x, y)| 0 <= *x && *x < column_count && 0 <= *y && *y < row_count)
                    .find_map(|(x, y)| {
                        let pos = (y * column_count + x) as usize;
                        self.tiles[pos]
                    });

                matches!(first_seat, Some(Seat::Occupied))
            })
            .count()
    }

    pub fn transformed(&self, rules: impl Fn(Seat, usize) -> Seat) -> Self {
        let tiles = self
            .tiles
            .iter()
            .enumerate()
            .map(|(pos, seat)| seat.map(|seat| rules(seat, pos)))
            .collect();
        Self { tiles, ..*self }
    }
}

pub fn part1(input: &str) -> usize {
    let mut seats = SeatMap::from(input);

    loop {
        let next_seats = seats.transformed(|seat, pos| match seat {
            Seat::Empty if seats.adjacent_occupied_seat_count(pos) == 0 => Seat::Occupied,
            Seat::Occupied if seats.adjacent_occupied_seat_count(pos) >= 4 => Seat::Empty,
            seat => seat,
        });

        if seats == next_seats {
            break seats
                .tiles
                .iter()
                .filter(|seat| matches!(seat, Some(Seat::Occupied)))
                .count();
        }

        seats = next_seats;
    }
}

pub fn part2(input: &str) -> usize {
    let mut seats = SeatMap::from(input);

    loop {
        let next_seats = seats.transformed(|seat, pos| match seat {
            Seat::Empty if seats.visible_occupied_seat_count(pos) == 0 => Seat::Occupied,
            Seat::Occupied if seats.visible_occupied_seat_count(pos) >= 5 => Seat::Empty,
            seat => seat,
        });

        if seats == next_seats {
            break seats
                .tiles
                .iter()
                .filter(|seat| matches!(seat, Some(Seat::Occupied)))
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
